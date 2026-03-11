use anyhow::{anyhow, Context, Result};
use log::{debug, info};
use std::process::Command;

use crate::config::Config;

pub struct IpmiClient {
    login_string: Vec<String>,
}

impl IpmiClient {
    pub fn new(config: &Config) -> Result<Self> {
        let login_string = if config.is_local() {
            // Check if IPMI device is accessible
            let paths = ["/dev/ipmi0", "/dev/ipmi/0", "/dev/ipmidev/0"];
            if !paths.iter().any(|p| std::path::Path::new(p).exists()) {
                return Err(anyhow!(
                    "Could not open device at /dev/ipmi0 or /dev/ipmi/0 or /dev/ipmidev/0"
                ));
            }
            vec!["-I".to_string(), "open".to_string()]
        } else {
            vec![
                "-I".to_string(),
                "lanplus".to_string(),
                "-H".to_string(),
                config.idrac_host.clone(),
                "-U".to_string(),
                config.idrac_username.clone(),
                "-P".to_string(),
                config.idrac_password.clone(),
            ]
        };

        Ok(IpmiClient { login_string })
    }

    fn run_command(&self, args: &[&str]) -> Result<String> {
        let mut cmd = Command::new("ipmitool");
        cmd.args(&self.login_string);
        cmd.args(args);

        debug!("Running ipmitool command: {:?}", cmd);

        let output = cmd.output().context("Failed to execute ipmitool")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("ipmitool command failed: {}", stderr));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn get_server_info(&self) -> Result<ServerInfo> {
        let output = self.run_command(&["fru"])?;

        let manufacturer = output
            .lines()
            .find(|l| l.contains("Product Manufacturer"))
            .and_then(|l| l.split(':').nth(1))
            .map(|s| s.trim().to_string())
            .or_else(|| {
                output
                    .lines()
                    .find(|l| l.contains("Board Mfg"))
                    .and_then(|l| l.split(':').nth(1))
                    .map(|s| s.trim().to_string())
            })
            .unwrap_or_default();

        let model = output
            .lines()
            .find(|l| l.contains("Product Name"))
            .and_then(|l| l.split(':').nth(1))
            .map(|s| s.trim().to_string())
            .or_else(|| {
                output
                    .lines()
                    .find(|l| l.contains("Board Product"))
                    .and_then(|l| l.split(':').nth(1))
                    .map(|s| s.trim().to_string())
            })
            .unwrap_or_default();

        // Check if Gen 14 or newer (R/T x40, x50, x60, etc.)
        let is_gen_14_or_newer = model
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .chars()
            .nth(0)
            .map(|c| c >= '4')
            .unwrap_or(false);

        Ok(ServerInfo {
            manufacturer,
            model,
            is_gen_14_or_newer,
        })
    }

    pub fn get_temperatures(&self, server_info: &ServerInfo) -> Result<Temperatures> {
        let output = self.run_command(&["sdr", "type", "temperature"])?;

        let temp_lines: Vec<&str> = output.lines().filter(|l| l.contains("degrees")).collect();

        // Parse CPU temperatures - look for lines containing "Temp" and "3."
        let cpu_data: Vec<&str> = temp_lines
            .iter()
            .filter(|l| l.contains("3.") && l.contains("Temp"))
            .copied()
            .collect();

        // Extract all temperature values from CPU lines
        let cpu_temps: Vec<i32> = cpu_data
            .iter()
            .filter_map(|line| extract_single_temperature(line))
            .collect();

        // Use first available temperature as CPU1
        let cpu1_temp = cpu_temps
            .get(0)
            .copied()
            .ok_or_else(|| anyhow!("No CPU temperature sensors found"))?;
        
        // Use second temperature as CPU2 if available
        let cpu2_temp = cpu_temps.get(1).copied();

        // Parse inlet temperature
        let inlet_temp = temp_lines
            .iter()
            .find(|l| l.contains("Inlet"))
            .and_then(|l| extract_single_temperature(l))
            .unwrap_or(0);

        // Parse exhaust temperature
        let exhaust_temp = temp_lines
            .iter()
            .find(|l| l.contains("Exhaust"))
            .and_then(|l| extract_single_temperature(l));

        Ok(Temperatures {
            cpu1: cpu1_temp,
            cpu2: cpu2_temp,
            inlet: inlet_temp,
            exhaust: exhaust_temp,
        })
    }

    pub fn set_manual_fan_control(&self) -> Result<()> {
        debug!("Setting manual fan control mode");
        self.run_command(&["raw", "0x30", "0x30", "0x01", "0x00"])?;
        Ok(())
    }

    pub fn set_dell_default_fan_control(&self) -> Result<()> {
        info!("Applying Dell default dynamic fan control profile");
        self.run_command(&["raw", "0x30", "0x30", "0x01", "0x01"])?;
        Ok(())
    }

    pub fn set_fan_speed(&self, speed: u8) -> Result<()> {
        let speed = speed.min(100);
        let hex_speed = format!("0x{:02x}", speed);
        debug!("Setting fan speed to {}% ({})", speed, hex_speed);
        
        self.run_command(&["raw", "0x30", "0x30", "0x02", "0xff", &hex_speed])?;
        Ok(())
    }

    pub fn disable_third_party_pcie_cooling(&self) -> Result<()> {
        debug!("Disabling third-party PCIe card Dell default cooling response");
        self.run_command(&[
            "raw", "0x30", "0xce", "0x00", "0x16", "0x05", "0x00", "0x00", "0x00", "0x05", "0x00",
            "0x01", "0x00", "0x00",
        ])?;
        Ok(())
    }

    pub fn enable_third_party_pcie_cooling(&self) -> Result<()> {
        debug!("Enabling third-party PCIe card Dell default cooling response");
        self.run_command(&[
            "raw", "0x30", "0xce", "0x00", "0x16", "0x05", "0x00", "0x00", "0x00", "0x05", "0x00",
            "0x00", "0x00", "0x00",
        ])?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub manufacturer: String,
    pub model: String,
    pub is_gen_14_or_newer: bool,
}

#[derive(Debug, Clone)]
pub struct Temperatures {
    pub cpu1: i32,
    pub cpu2: Option<i32>,
    pub inlet: i32,
    pub exhaust: Option<i32>,
}

fn extract_temperature(data: &[&str], index: usize) -> Result<i32> {
    let all_nums: Vec<i32> = data
        .iter()
        .flat_map(|line| {
            line.split_whitespace()
                .filter_map(|word| word.parse::<i32>().ok())
        })
        .collect();

    all_nums
        .get(index)
        .copied()
        .ok_or_else(|| anyhow!("Could not extract temperature at index {}", index))
}

fn extract_single_temperature(line: &str) -> Option<i32> {
    line.split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .last()
}

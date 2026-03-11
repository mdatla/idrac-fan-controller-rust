mod config;
mod fan_curve;
mod ipmi;

use anyhow::{Context, Result};
use log::{error, info, warn};
use std::time::Duration;
use tokio::time;

use config::Config;
use fan_curve::{calculate_fan_speed, get_max_cpu_temp};
use ipmi::{IpmiClient, ServerInfo, Temperatures};

struct Controller {
    config: Config,
    client: IpmiClient,
    server_info: ServerInfo,
    last_fan_speed: Option<u8>,
}

impl Controller {
    async fn new(config: Config) -> Result<Self> {
        let client = IpmiClient::new(&config).context("Failed to initialize IPMI client")?;

        let server_info = client
            .get_server_info()
            .context("Failed to get server information")?;

        info!("Server model: {} {}", server_info.manufacturer, server_info.model);
        info!("iDRAC/IPMI host: {}", config.idrac_host);
        info!("Generation 14 or newer: {}", server_info.is_gen_14_or_newer);
        info!("");
        info!("Fan curve configuration:");
        info!("  Min fan speed: {}%", config.min_fan_speed);
        info!("  Max fan speed: {}%", config.max_fan_speed);
        info!("  Base temperature: {}°C", config.base_temp);
        info!("  Critical temperature: {}°C", config.critical_temp);
        info!("  Curve steepness: {}", config.curve_steepness);
        info!("  Check interval: {}s", config.check_interval);
        info!("");

        if server_info.manufacturer != "DELL" {
            return Err(anyhow::anyhow!("Server is not a Dell product"));
        }

        // Set manual fan control mode initially
        client
            .set_manual_fan_control()
            .context("Failed to set manual fan control")?;

        // Handle third-party PCIe cards (Gen 13 and older)
        if !server_info.is_gen_14_or_newer {
            if config.disable_third_party_pcie {
                client
                    .disable_third_party_pcie_cooling()
                    .context("Failed to disable third-party PCIe cooling")?;
            } else {
                client
                    .enable_third_party_pcie_cooling()
                    .context("Failed to enable third-party PCIe cooling")?;
            }
        }

        Ok(Controller {
            config,
            client,
            server_info,
            last_fan_speed: None,
        })
    }

    async fn control_loop(&mut self) -> Result<()> {
        let mut interval = time::interval(Duration::from_secs(self.config.check_interval));
        let mut line_counter = 0;

        loop {
            interval.tick().await;

            match self.process_iteration(&mut line_counter) {
                Ok(_) => {}
                Err(e) => {
                    error!("Error in control loop: {:#}", e);
                    // Continue running even on errors
                }
            }
        }
    }

    fn process_iteration(&mut self, line_counter: &mut usize) -> Result<()> {
        // Get current temperatures
        let temps = self
            .client
            .get_temperatures(&self.server_info)
            .context("Failed to read temperatures")?;

        // Print header every 10 lines
        if *line_counter % 10 == 0 {
            self.print_header(&temps);
        }

        // Calculate maximum CPU temperature
        let max_cpu_temp = get_max_cpu_temp(temps.cpu1, temps.cpu2);

        // Calculate desired fan speed using exponential curve
        let desired_fan_speed = calculate_fan_speed(max_cpu_temp as f64, &self.config);

        // Only update if fan speed changed (with hysteresis of ±2%)
        let should_update = match self.last_fan_speed {
            None => true,
            Some(last) => (desired_fan_speed as i16 - last as i16).abs() >= 2,
        };

        if should_update {
            self.client
                .set_manual_fan_control()
                .context("Failed to set manual fan control")?;
            self.client
                .set_fan_speed(desired_fan_speed)
                .context("Failed to set fan speed")?;
            self.last_fan_speed = Some(desired_fan_speed);
        }

        // Print current status
        self.print_status(&temps, desired_fan_speed, should_update);

        *line_counter += 1;

        Ok(())
    }

    fn print_header(&self, temps: &Temperatures) {
        let cpu_count = if temps.cpu2.is_some() { 2 } else { 1 };
        let has_exhaust = temps.exhaust.is_some();

        print!("    Date & time          Inlet  CPU 1");
        if cpu_count > 1 {
            print!("  CPU 2");
        }
        if has_exhaust {
            print!("  Exhaust");
        }
        println!("  Fan Speed  Comment");
    }

    fn print_status(&self, temps: &Temperatures, fan_speed: u8, updated: bool) {
        let now = chrono::Local::now();
        let timestamp = now.format("%d-%m-%Y %T");

        print!("{}  {:3}°C  {:3}°C", timestamp, temps.inlet, temps.cpu1);

        if let Some(cpu2) = temps.cpu2 {
            print!("  {:3}°C", cpu2);
        } else {
            print!("     -");
        }

        if let Some(exhaust) = temps.exhaust {
            print!("    {:3}°C", exhaust);
        } else {
            print!("       -");
        }

        print!("      {:3}%", fan_speed);

        if updated {
            print!("  Fan speed adjusted");
        } else {
            print!("  -");
        }

        println!();
    }

    async fn shutdown(&self) {
        info!("Shutting down gracefully...");

        // Restore Dell default fan control
        if let Err(e) = self.client.set_dell_default_fan_control() {
            error!("Failed to restore Dell default fan control: {}", e);
        }

        // Handle third-party PCIe cards on shutdown
        if !self.server_info.is_gen_14_or_newer
            && !self.config.keep_third_party_state_on_exit
        {
            if let Err(e) = self.client.enable_third_party_pcie_cooling() {
                error!("Failed to enable third-party PCIe cooling on exit: {}", e);
            }
        }

        info!("Dell default dynamic fan control profile applied for safety");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .write_style(env_logger::WriteStyle::Always)
        .init();

    eprintln!("Dell iDRAC Fan Controller (Rust Edition with Exponential Curve)");
    eprintln!("================================================================");
    eprintln!("");

    let config = Config::from_env().context("Failed to load configuration")?;
    let mut controller = Controller::new(config).await?;

    // Setup signal handlers for graceful shutdown
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    let signal_tx = tx.clone();
    tokio::spawn(async move {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sigterm = signal(SignalKind::terminate()).expect("Failed to setup SIGTERM handler");
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to setup SIGINT handler");
        let mut sigquit = signal(SignalKind::quit()).expect("Failed to setup SIGQUIT handler");

        tokio::select! {
            _ = sigterm.recv() => {
                info!("Received SIGTERM");
            }
            _ = sigint.recv() => {
                info!("Received SIGINT");
            }
            _ = sigquit.recv() => {
                info!("Received SIGQUIT");
            }
        }

        let _ = signal_tx.send(()).await;
    });

    // Run control loop until signal received
    tokio::select! {
        result = controller.control_loop() => {
            if let Err(e) = result {
                error!("Control loop exited with error: {:#}", e);
            }
        }
        _ = rx.recv() => {
            info!("Shutdown signal received");
        }
    }

    controller.shutdown().await;

    Ok(())
}

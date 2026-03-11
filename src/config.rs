use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub idrac_host: String,
    pub idrac_username: String,
    pub idrac_password: String,
    pub min_fan_speed: u8,
    pub max_fan_speed: u8,
    pub base_temp: f64,
    pub critical_temp: f64,
    pub curve_steepness: f64,
    pub check_interval: u64,
    pub disable_third_party_pcie: bool,
    pub keep_third_party_state_on_exit: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config {
            idrac_host: env::var("IDRAC_HOST").unwrap_or_else(|_| "local".to_string()),
            idrac_username: env::var("IDRAC_USERNAME").unwrap_or_else(|_| "root".to_string()),
            idrac_password: env::var("IDRAC_PASSWORD").unwrap_or_else(|_| "calvin".to_string()),
            
            // Minimum fan speed when temperature is at base_temp (default 5%)
            min_fan_speed: env::var("MIN_FAN_SPEED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            
            // Maximum fan speed at critical_temp (default 100%)
            max_fan_speed: env::var("MAX_FAN_SPEED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            
            // Base temperature where minimum fan speed is applied (default 40°C)
            base_temp: env::var("BASE_TEMP")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(40.0),
            
            // Critical temperature where maximum fan speed is applied (default 70°C)
            critical_temp: env::var("CRITICAL_TEMP")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(70.0),
            
            // Curve steepness factor (higher = steeper curve, default 0.15)
            curve_steepness: env::var("CURVE_STEEPNESS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.15),
            
            check_interval: env::var("CHECK_INTERVAL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60),
            
            disable_third_party_pcie: env::var("DISABLE_THIRD_PARTY_PCIE_CARD_DELL_DEFAULT_COOLING_RESPONSE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
            
            keep_third_party_state_on_exit: env::var("KEEP_THIRD_PARTY_PCIE_CARD_COOLING_RESPONSE_STATE_ON_EXIT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
        })
    }
    
    pub fn is_local(&self) -> bool {
        self.idrac_host == "local"
    }
}

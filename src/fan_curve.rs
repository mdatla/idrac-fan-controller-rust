use crate::config::Config;

/// Calculate fan speed using an exponential curve
///
/// The exponential curve provides smooth transitions:
/// - At base_temp: returns min_fan_speed
/// - Between base_temp and critical_temp: exponential increase
/// - At or above critical_temp: returns max_fan_speed
///
/// Formula: fan_speed = min + (max - min) * (1 - e^(-k * (T - T_base))) / (1 - e^(-k * (T_crit - T_base)))
/// Where k is the curve steepness factor
pub fn calculate_fan_speed(temp: f64, config: &Config) -> u8 {
    // If temperature is at or below base temperature, use minimum fan speed
    if temp <= config.base_temp {
        return config.min_fan_speed;
    }

    // If temperature is at or above critical temperature, use maximum fan speed
    if temp >= config.critical_temp {
        return config.max_fan_speed;
    }

    // Calculate normalized temperature (0.0 to 1.0)
    let temp_range = config.critical_temp - config.base_temp;
    let normalized_temp = (temp - config.base_temp) / temp_range;

    // Apply exponential curve
    // Using: y = 1 - e^(-k*x) normalized to [0, 1]
    let k = config.curve_steepness;
    let exp_value = 1.0 - (-k * normalized_temp * 10.0).exp();
    let exp_normalized = exp_value / (1.0 - (-k * 10.0).exp());

    // Map to fan speed range
    let fan_range = (config.max_fan_speed - config.min_fan_speed) as f64;
    let fan_speed = config.min_fan_speed as f64 + (fan_range * exp_normalized);

    // Clamp and round
    fan_speed.round().max(config.min_fan_speed as f64).min(config.max_fan_speed as f64) as u8
}

/// Get the maximum temperature from CPU readings
pub fn get_max_cpu_temp(cpu1: i32, cpu2: Option<i32>) -> i32 {
    cpu2.map(|c2| cpu1.max(c2)).unwrap_or(cpu1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config {
            idrac_host: "local".to_string(),
            idrac_username: "root".to_string(),
            idrac_password: "calvin".to_string(),
            min_fan_speed: 5,
            max_fan_speed: 100,
            base_temp: 40.0,
            critical_temp: 70.0,
            curve_steepness: 0.15,
            check_interval: 60,
            disable_third_party_pcie: false,
            keep_third_party_state_on_exit: false,
            temp_smoothing_window: 3,
            min_change_interval: 60,
            emergency_temp_delta: 10.0,
            hysteresis_percent: 5,
        }
    }

    #[test]
    fn test_fan_curve_at_base_temp() {
        let config = test_config();
        let speed = calculate_fan_speed(40.0, &config);
        assert_eq!(speed, 5);
    }

    #[test]
    fn test_fan_curve_at_critical_temp() {
        let config = test_config();
        let speed = calculate_fan_speed(70.0, &config);
        assert_eq!(speed, 100);
    }

    #[test]
    fn test_fan_curve_below_base_temp() {
        let config = test_config();
        let speed = calculate_fan_speed(30.0, &config);
        assert_eq!(speed, 5);
    }

    #[test]
    fn test_fan_curve_above_critical_temp() {
        let config = test_config();
        let speed = calculate_fan_speed(80.0, &config);
        assert_eq!(speed, 100);
    }

    #[test]
    fn test_fan_curve_mid_range() {
        let config = test_config();
        let speed = calculate_fan_speed(55.0, &config);
        // Should be somewhere between min and max, with exponential growth
        assert!(speed > 5 && speed < 100);
        println!("Fan speed at 55°C: {}%", speed);
    }

    #[test]
    fn test_exponential_growth() {
        let config = test_config();
        
        // Test that the curve is exponential (later temps have bigger jumps)
        let speed_45 = calculate_fan_speed(45.0, &config);
        let speed_50 = calculate_fan_speed(50.0, &config);
        let speed_55 = calculate_fan_speed(55.0, &config);
        let speed_60 = calculate_fan_speed(60.0, &config);
        let speed_65 = calculate_fan_speed(65.0, &config);
        
        let diff1 = speed_50 - speed_45;
        let diff2 = speed_55 - speed_50;
        let diff3 = speed_60 - speed_55;
        let diff4 = speed_65 - speed_60;
        
        println!("Temperature progression (5°C steps):");
        println!("45°C: {}%, 50°C: {}%, 55°C: {}%, 60°C: {}%, 65°C: {}%", 
                 speed_45, speed_50, speed_55, speed_60, speed_65);
        println!("Differences: {}, {}, {}, {}", diff1, diff2, diff3, diff4);
        
        // Exponential curve should show increasing differences
        assert!(diff2 >= diff1);
        assert!(diff3 >= diff2);
    }
}

/// Demo script to visualize the exponential fan curve
/// Run with: cargo run --example curve_demo

use std::env;

fn calculate_fan_speed(temp: f64, min_speed: u8, max_speed: u8, base_temp: f64, critical_temp: f64, steepness: f64) -> u8 {
    if temp <= base_temp {
        return min_speed;
    }
    if temp >= critical_temp {
        return max_speed;
    }

    let temp_range = critical_temp - base_temp;
    let normalized_temp = (temp - base_temp) / temp_range;
    let k = steepness;
    let exp_value = 1.0 - (-k * normalized_temp * 10.0).exp();
    let exp_normalized = exp_value / (1.0 - (-k * 10.0).exp());
    let fan_range = (max_speed - min_speed) as f64;
    let fan_speed = min_speed as f64 + (fan_range * exp_normalized);

    fan_speed.round().max(min_speed as f64).min(max_speed as f64) as u8
}

fn main() {
    let min_speed: u8 = env::var("MIN_FAN_SPEED").ok().and_then(|s| s.parse().ok()).unwrap_or(5);
    let max_speed: u8 = env::var("MAX_FAN_SPEED").ok().and_then(|s| s.parse().ok()).unwrap_or(100);
    let base_temp: f64 = env::var("BASE_TEMP").ok().and_then(|s| s.parse().ok()).unwrap_or(40.0);
    let critical_temp: f64 = env::var("CRITICAL_TEMP").ok().and_then(|s| s.parse().ok()).unwrap_or(70.0);
    let steepness: f64 = env::var("CURVE_STEEPNESS").ok().and_then(|s| s.parse().ok()).unwrap_or(0.15);

    println!("Dell iDRAC Fan Controller - Exponential Curve Visualization");
    println!("============================================================");
    println!();
    println!("Configuration:");
    println!("  Min Fan Speed: {}%", min_speed);
    println!("  Max Fan Speed: {}%", max_speed);
    println!("  Base Temperature: {}°C", base_temp);
    println!("  Critical Temperature: {}°C", critical_temp);
    println!("  Curve Steepness: {}", steepness);
    println!();
    println!("Temperature vs Fan Speed:");
    println!("------------------------");
    println!("  Temp (°C)  | Fan Speed (%) | Visual");
    println!("  -----------+---------------+----------------------------------------");

    let start_temp = (base_temp - 10.0).max(0.0) as i32;
    let end_temp = (critical_temp + 10.0) as i32;

    for temp in (start_temp..=end_temp).step_by(1) {
        let speed = calculate_fan_speed(
            temp as f64,
            min_speed,
            max_speed,
            base_temp,
            critical_temp,
            steepness,
        );

        let bar_length = (speed as f64 / 100.0 * 40.0) as usize;
        let bar: String = "█".repeat(bar_length);

        let marker = if (temp as f64 - base_temp).abs() < 0.5 {
            "  <-- Base temp"
        } else if (temp as f64 - critical_temp).abs() < 0.5 {
            "  <-- Critical temp"
        } else {
            ""
        };

        println!("    {:3}      |     {:3}%      | {}{}", 
                 temp, speed, bar, marker);
    }

    println!();
    println!("Key characteristics of exponential curve:");
    println!("- Slow, gradual increase at lower temperatures");
    println!("- Rapid increase as temperature approaches critical point");
    println!("- Smooth transitions prevent sudden fan speed changes");
    println!("- Better noise management compared to binary on/off");
}

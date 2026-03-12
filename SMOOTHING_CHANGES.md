# Fan Speed Smoothing and Rate Limiting Changes

## Summary

This update adds temperature smoothing and fan speed change rate limiting to reduce audible fan noise from frequent speed adjustments while maintaining responsive temperature monitoring.

## Problem Being Solved

- **Issue**: Even with 2% hysteresis, polling every 10-60 seconds can cause frequent fan speed changes
- **Impact**: Each fan speed change causes an audible voltage adjustment spike, making it noticeable and annoying
- **Goal**: Poll frequently (10s) for safety, but only change fan speeds when truly needed

## Solution

Implemented a multi-layered approach:

### 1. Temperature Smoothing (Moving Average)
- Maintains a rolling buffer of the last N temperature readings
- Calculates fan speed based on the **average** temperature, not instantaneous spikes
- Filters out brief temperature fluctuations that don't require fan adjustment

### 2. Minimum Dwell Time
- Enforces a minimum time interval between fan speed changes
- Default: 60 seconds between adjustments under normal conditions
- Prevents rapid oscillation in fan speed

### 3. Emergency Override
- Allows immediate fan speed changes if temperature spikes dramatically
- Default: 10°C spike overrides the minimum dwell time
- Ensures safety is never compromised for noise reduction

### 4. Configurable Hysteresis
- Changed from hardcoded ±2% to configurable
- Default increased to ±5% for better stability
- Prevents small oscillations around a setpoint

## New Configuration Options

```bash
# Temperature smoothing - average last N readings
TEMP_SMOOTHING_WINDOW=3        # Default: 3 readings

# Rate limiting - minimum seconds between fan changes
MIN_CHANGE_INTERVAL=60         # Default: 60 seconds

# Emergency override - temp spike that bypasses rate limiting
EMERGENCY_TEMP_DELTA=10        # Default: 10°C

# Hysteresis - minimum fan % change to trigger adjustment
HYSTERESIS_PERCENT=5           # Default: 5% (was hardcoded at 2%)
```

## Behavior Examples

### Example 1: Normal Operation with 10s Polling
```
Time    Temp    Smoothed    Fan Speed    Action
0:00    45°C    45.0°C      13%          Initial set
0:10    47°C    46.0°C      13%          No change (hysteresis not met)
0:20    48°C    46.7°C      13%          No change (hysteresis not met)
0:30    49°C    48.0°C      18%          Changed (60s elapsed + 5% change met)
0:40    50°C    49.0°C      18%          No change (60s not elapsed)
0:50    51°C    50.0°C      18%          No change (60s not elapsed)
1:00    52°C    51.0°C      18%          No change (60s not elapsed)
1:10    53°C    52.0°C      18%          No change (60s not elapsed)
1:20    54°C    53.0°C      23%          Changed (60s elapsed + 5% change met)
```

### Example 2: Emergency Spike
```
Time    Temp    Smoothed    Fan Speed    Action
0:00    45°C    45.0°C      13%          Initial set
0:10    47°C    46.0°C      13%          No change
0:20    58°C    50.0°C      28%          EMERGENCY! (12°C spike, override 60s rule)
```

## Recommended Settings

### For Quiet Operation (Default)
```bash
CHECK_INTERVAL=10              # Poll every 10s for safety
TEMP_SMOOTHING_WINDOW=3        # Average last 3 readings (30s of data)
MIN_CHANGE_INTERVAL=60         # Only change fans every 60s
HYSTERESIS_PERCENT=5           # Require 5% change
EMERGENCY_TEMP_DELTA=10        # Emergency on 10°C spike
```

### For Very Quiet Operation (Less Responsive)
```bash
CHECK_INTERVAL=10
TEMP_SMOOTHING_WINDOW=5        # More smoothing (50s of data)
MIN_CHANGE_INTERVAL=120        # Only change every 2 minutes
HYSTERESIS_PERCENT=8           # Require larger change
EMERGENCY_TEMP_DELTA=10
```

### For Responsive Operation (More Noise)
```bash
CHECK_INTERVAL=10
TEMP_SMOOTHING_WINDOW=1        # No smoothing (instant response)
MIN_CHANGE_INTERVAL=10         # Allow changes every 10s
HYSTERESIS_PERCENT=2           # Sensitive to small changes
EMERGENCY_TEMP_DELTA=5         # Lower emergency threshold
```

## Implementation Details

### Code Changes

1. **config.rs**
   - Added 4 new config fields
   - Default values chosen for good balance
   - Validation (e.g., window ≥ 1)

2. **main.rs**
   - Added `VecDeque<i32>` for temperature history buffer
   - Added `last_temp` for spike detection
   - Added `last_change_time` for dwell time tracking
   - Updated control loop with smoothing logic
   - Enhanced status output to show smoothed temperature

3. **GitHub Actions**
   - PR builds now push to `:beta` tag
   - Main branch builds push to `:latest` tag
   - `:stable` tag remains manual (untouched)

### Output Format Changes

New output includes smoothed temperature:
```
Date & time          Inlet  CPU 1  CPU 2  Exhaust  Smoothed  Fan Speed  Comment
11-03-2026 14:23:45   28°C   45°C   47°C     52°C     46.0°C      13%  -
11-03-2026 14:24:45   28°C   48°C   49°C     54°C     48.0°C      18%  Fan speed adjusted
```

## Testing with :beta Tag

When you create a PR with these changes:

1. GitHub Actions will automatically build and push to `maanstr/idrac-fan-controller-rust:beta`
2. Test without losing your current `:latest` container:
   ```bash
   docker pull maanstr/idrac-fan-controller-rust:beta
   docker run -d --name idrac-fan-beta \
     -e IDRAC_HOST=192.168.1.100 \
     -e IDRAC_USERNAME=root \
     -e IDRAC_PASSWORD=yourpassword \
     -e CHECK_INTERVAL=10 \
     -e TEMP_SMOOTHING_WINDOW=3 \
     -e MIN_CHANGE_INTERVAL=60 \
     maanstr/idrac-fan-controller-rust:beta
   ```
3. Monitor the logs to see smoothing in action:
   ```bash
   docker logs -f idrac-fan-beta
   ```
4. Once satisfied, merge the PR and `:latest` will be updated automatically

## Safety Considerations

- Emergency override ensures safety is never compromised
- Temperature still polled at configured interval (default 60s, can be 10s)
- Smoothing only affects **when** fan changes occur, not the curve itself
- If temp spikes ≥ EMERGENCY_TEMP_DELTA, fan responds immediately
- All existing safety features remain (auto-restore on exit, etc.)

## Backward Compatibility

All new settings have sensible defaults:
- Existing deployments will continue working without changes
- Behavior will be slightly different (better) due to new defaults
- To restore old behavior (less recommended):
  ```bash
  TEMP_SMOOTHING_WINDOW=1
  MIN_CHANGE_INTERVAL=0
  HYSTERESIS_PERCENT=2
  ```

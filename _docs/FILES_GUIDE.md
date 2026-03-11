# File Guide - Which File Should I Read?

Quick reference to help you find what you need:

## I Want To...

### Get Started Quickly
→ **[QUICKSTART.md](QUICKSTART.md)** - Step-by-step setup instructions

### Test Safely Without Breaking My Server
→ **[SAFE_TESTING.md](SAFE_TESTING.md)** - 6-level testing strategy from dry-run to production

### Understand What's Different From Original
→ **[COMPARISON.md](COMPARISON.md)** - Detailed technical comparison, architecture, behavior

### Learn About the Features
→ **[README.md](README.md)** - Complete documentation of features and configuration

### Get a High-Level Overview
→ **[PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)** - TL;DR of the entire project

### See the Code
→ **[src/](src/)** directory:
- `main.rs` - Main control loop
- `fan_curve.rs` - Exponential curve calculations
- `ipmi.rs` - IPMI communication
- `config.rs` - Configuration management

### Visualize the Fan Curve
→ **[examples/curve_demo.rs](examples/curve_demo.rs)** - Run with `cargo run --example curve_demo`

### Deploy With Docker
→ **[docker-compose.yml](docker-compose.yml)** - Edit and run `docker-compose up -d`

## File Purpose Summary

| File | Purpose | When to Read |
|------|---------|--------------|
| **README.md** | Complete documentation | After quickstart, for reference |
| **QUICKSTART.md** | Get running fast | First time setup |
| **SAFE_TESTING.md** | Testing strategy | Before first run! |
| **COMPARISON.md** | Original vs Rust | Understanding differences |
| **PROJECT_SUMMARY.md** | High-level overview | Quick understanding |
| **FILES_GUIDE.md** | This file | When lost |
| **Cargo.toml** | Rust dependencies | Building from source |
| **Dockerfile** | Container build | Building Docker image |
| **docker-compose.yml** | Easy deployment | Deploying with Docker |

## Recommended Reading Order

### For First-Time Users
1. **PROJECT_SUMMARY.md** - Understand what this is (5 min)
2. **SAFE_TESTING.md** - Learn how to test safely (10 min)
3. **QUICKSTART.md** - Get it running (10 min)
4. **README.md** - Full reference (when needed)

### For Experienced Users of Original
1. **COMPARISON.md** - See what changed (10 min)
2. **QUICKSTART.md** - Migration guide (5 min)
3. **README.md** - Configuration reference (as needed)

### For Developers
1. **PROJECT_SUMMARY.md** - Architecture overview (5 min)
2. **src/** - Read the source code
3. **Cargo.toml** - Dependencies
4. **COMPARISON.md** - Design decisions

## Quick Answers

**Q: Is Rust included in the Docker image?**  
A: No! Multi-stage build compiles with Rust, final image is just the binary + ipmitool (~50 MB)

**Q: How do I know if the exponential curve will work for me?**  
A: Run `cargo run --example curve_demo` to see temperature → fan speed mapping

**Q: Is this safe to use?**  
A: Yes, with proper testing. Follow [SAFE_TESTING.md](SAFE_TESTING.md) levels 1-6

**Q: Can I go back to the original?**  
A: Yes! See "Rollback Plan" in [SAFE_TESTING.md](SAFE_TESTING.md)

**Q: What if I just want the original behavior?**  
A: Set `MIN_FAN_SPEED=5`, `MAX_FAN_SPEED=5` for static speed (but defeats the purpose!)

**Q: How do I adjust the curve?**  
A: Tune `BASE_TEMP`, `CRITICAL_TEMP`, and `CURVE_STEEPNESS`. See examples in [SAFE_TESTING.md](SAFE_TESTING.md)

## File Size Reference

| What | Size |
|------|------|
| Source code | ~5 KB (Rust) |
| Compiled binary | ~2 MB (stripped) |
| Docker build cache | ~1 GB (temporary) |
| Final Docker image | ~50 MB |
| Documentation | ~40 KB |

## Still Lost?

Start here: **[QUICKSTART.md](QUICKSTART.md)** → Section "Visualize the Fan Curve"

This will show you what the controller does without touching your hardware.

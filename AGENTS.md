# Agent Instructions

This project has a **memory bank** at `_memory_bank/` that persists context across sessions. Read it before starting any work.

## Before You Start

Read these files in order:

1. `_memory_bank/memory-bank-instructions.md` -- how the memory bank works
2. `_memory_bank/long-term-memory/productContext.md` -- what this project is and why
3. `_memory_bank/long-term-memory/systemPatterns.md` -- architecture, build patterns, key decisions
4. `_memory_bank/long-term-memory/techContext.md` -- tech stack, build commands, constraints
5. `_memory_bank/short-term-memory/activeContext.md` -- what's being worked on now
6. `_memory_bank/short-term-memory/progress.md` -- what's done, what's left, known issues

## Project Summary

Dell iDRAC fan controller rewritten in Rust with an exponential fan curve. Deployed as a Docker container on Unraid (Dell R720xd). See `_memory_bank/long-term-memory/productContext.md` for full context.

## Build

```bash
# Local build (cross-compiles from Apple Silicon to AMD64)
./build-local.sh --tag test --push

# Critical: all builds MUST use these flags
# --platform linux/amd64 --provenance=false (no cache)
```

## Test

```bash
# On the target machine (Dell R720xd / Unraid)
docker pull maanstr/idrac-fan-controller-rust:test
docker run -d --name idrac-fan-test --device /dev/ipmi0 -e IDRAC_HOST=local maanstr/idrac-fan-controller-rust:test
docker logs -f idrac-fan-test
```

## Code Structure

```
src/main.rs       -- Controller struct, control loop, signal handling
src/config.rs     -- Config from environment variables
src/ipmi.rs       -- ipmitool CLI wrapper, temperature parsing
src/fan_curve.rs  -- Exponential curve calculation, unit tests
```

## After You Finish

Update the memory bank:
- `_memory_bank/short-term-memory/activeContext.md` -- what changed, next steps
- `_memory_bank/short-term-memory/progress.md` -- current status
- If new stable patterns emerged, update `_memory_bank/long-term-memory/`

# Getting Started - Quick Navigation

## What do you want to do?

### 🚀 I want to build and deploy to Unraid (Manual)

**Follow this workflow:**
1. Read [MANUAL_WORKFLOW.md](MANUAL_WORKFLOW.md) - Complete step-by-step guide
2. Build → Push to Docker Hub → Test on Unraid → Iterate

**Quick steps:**
```bash
# 1. Build
./build-local.sh YOUR_DOCKERHUB_USERNAME

# 2. Push
docker push YOUR_DOCKERHUB_USERNAME/idrac-fan-controller-rust:latest

# 3. Install on Unraid from Docker Hub
# See MANUAL_WORKFLOW.md Phase 2
```

---

### 🧪 I want to test locally first (No hardware risk)

```bash
# Build
./build-local.sh

# Test (choose option 1 for dry-run)
./test-local.sh
```

Then proceed to manual workflow above.

---

### 📚 I want to understand what this does

Read in this order:
1. [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - 5 min overview
2. [COMPARISON.md](COMPARISON.md) - How it differs from original
3. [README.md](README.md) - Complete documentation

---

### ⚙️ I'm already using this and want to tune it

**Adjust these settings:**
- Too loud? → Increase `BASE_TEMP`, decrease `MAX_FAN_SPEED`
- Temps high? → Decrease `BASE_TEMP`, increase `MIN_FAN_SPEED`
- See [MANUAL_WORKFLOW.md](MANUAL_WORKFLOW.md) Phase 3 for details

---

### 🤖 I want to automate builds (GitHub Actions)

**First:** Complete manual workflow and verify it works on Unraid

**Then:** We can set up GitHub Actions to automatically:
- Build on git push
- Tag releases
- Push to Docker Hub
- Support multi-architecture (AMD64 + ARM64)

(We'll add this after manual testing is successful!)

---

## The Simple Path: Build → Docker Hub → Unraid

```
┌─────────────────┐
│ Your Dev Machine│
│                 │
│ 1. Build image  │
│ 2. Push to Hub  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Docker Hub    │
│                 │
│  Public image   │
│  available      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Unraid Server  │
│                 │
│ 3. Pull & run   │
│ 4. Test & tune  │
└─────────────────┘
```

**Estimated time:**
- Build: 5-10 minutes
- Push: 1-3 minutes  
- Setup on Unraid: 5 minutes
- Testing/tuning: 1-2 hours

---

## Files You Need

| When | Read This |
|------|-----------|
| Starting out | **MANUAL_WORKFLOW.md** ← Start here! |
| Testing locally | test-local.sh |
| Understanding | PROJECT_SUMMARY.md, COMPARISON.md |
| Unraid specifics | UNRAID_SETUP.md |
| Safety/testing | SAFE_TESTING.md |
| Reference | README.md |

---

## Prerequisites

- [ ] Docker installed on dev machine
- [ ] Docker Hub account created
- [ ] Unraid server with network access
- [ ] Dell PowerEdge server with iDRAC
- [ ] iDRAC credentials (username/password)
- [ ] 30 minutes free time

---

## First Time? Start Here:

```bash
# 1. Navigate to project
cd oss-repos/Dell_iDRAC_fan_controller_Docker/rust-version

# 2. Open the workflow guide
cat MANUAL_WORKFLOW.md
# or open in browser/editor

# 3. Follow Phase 1 (Build & Push to Docker Hub)
```

That's it! The workflow guide will walk you through everything step-by-step.

---

## Questions?

**Q: Do I need Rust installed?**  
A: No! Docker handles the Rust build. You only need Docker.

**Q: Will this work on my Unraid server?**  
A: Yes, if you have a Dell PowerEdge with iDRAC firmware < 3.30.30.30

**Q: Is it safe to test?**  
A: Yes! See SAFE_TESTING.md for risk-free testing levels. The controller auto-restores Dell default on exit.

**Q: How big is the image?**  
A: ~50 MB (Rust is NOT included in final image)

**Q: Can I test before deploying?**  
A: Yes! Use `./test-local.sh` for safe local testing first.

---

**Ready? → Open [MANUAL_WORKFLOW.md](MANUAL_WORKFLOW.md) and let's go!**

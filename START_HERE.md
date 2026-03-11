# 🎯 START HERE - Complete Workflow

## Your Path: Manual Build → Docker Hub → Unraid → (Later) GitHub Actions

### What You'll Do

1. **Build** the Docker image locally
2. **Push** to your Docker Hub account  
3. **Deploy** on Unraid and test
4. **Tune** settings for your environment
5. **(Later)** Automate with GitHub Actions

---

## Step-by-Step Commands

### Prerequisites
- Docker installed on your machine
- Docker Hub account (free): https://hub.docker.com
- Your Dell server's iDRAC IP and credentials

---

### 1️⃣ Build Locally

```bash
cd oss-repos/Dell_iDRAC_fan_controller_Docker/rust-version

# Replace YOUR_USERNAME with your Docker Hub username
./build-local.sh YOUR_USERNAME

# Example: ./build-local.sh johndoe
```

⏱️ Takes 5-10 minutes (first time only)

---

### 2️⃣ Login to Docker Hub

```bash
docker login
# Enter your Docker Hub username
# Enter your password (or access token)
```

💡 **Recommended:** Use access token instead of password
- Go to: https://hub.docker.com/settings/security
- Create token → Use as password

---

### 3️⃣ Create Docker Hub Repository

1. Go to https://hub.docker.com
2. Click **Repositories** → **Create Repository**
3. Name: `idrac-fan-controller-rust`
4. Visibility: **Public**
5. Click **Create**

---

### 4️⃣ Push to Docker Hub

```bash
docker push YOUR_USERNAME/idrac-fan-controller-rust:latest

# Example: docker push johndoe/idrac-fan-controller-rust:latest
```

⏱️ Takes 1-3 minutes

✅ Verify at: https://hub.docker.com/r/YOUR_USERNAME/idrac-fan-controller-rust

---

### 5️⃣ Install on Unraid

1. Open Unraid WebUI → **Docker** tab
2. Click **Add Container**
3. Fill in:
   - **Name:** `iDRAC-Fan-Controller`
   - **Repository:** `YOUR_USERNAME/idrac-fan-controller-rust:latest`
   - **Network:** `bridge`

4. Add environment variables (click "Add another Variable" for each):

   | Key | Value |
   |-----|-------|
   | `IDRAC_HOST` | Your iDRAC IP (e.g., 192.168.1.100) |
   | `IDRAC_USERNAME` | root |
   | `IDRAC_PASSWORD` | Your iDRAC password |
   | `MIN_FAN_SPEED` | 10 |
   | `MAX_FAN_SPEED` | 80 |
   | `BASE_TEMP` | 40 |
   | `CRITICAL_TEMP` | 70 |

5. Click **Apply**

---

### 6️⃣ Check Logs

Click container icon → **Logs**

Should see:
```
Dell iDRAC Fan Controller (Rust Edition)
Server model: DELL PowerEdge R720
...
Date & time          Inlet  CPU 1  CPU 2  Exhaust  Fan Speed
```

✅ If you see temperature readings and fan adjustments → **Success!**

---

### 7️⃣ Monitor & Tune

Watch for 1-2 hours during normal use.

**Too loud?**
- Edit container → Increase `BASE_TEMP` to 45
- Decrease `MAX_FAN_SPEED` to 70

**Temps too high?**
- Edit container → Decrease `BASE_TEMP` to 35
- Increase `MIN_FAN_SPEED` to 15

---

### 8️⃣ Enable Auto-Start

Once stable:
1. Edit container
2. Set **Autostart:** `Yes`
3. Apply

Container will start automatically on Unraid boot.

---

## Testing Before Unraid (Optional)

Want to test locally first?

```bash
./test-local.sh
# Choose option 1 for dry-run (no hardware)
# Choose option 2 for 5-minute real test
```

---

## What's Next?

✅ **After it's working on Unraid:**

We can set up GitHub Actions to automate:
- Build on every commit
- Auto-push to Docker Hub
- Version tagging
- Multi-architecture support (AMD64 + ARM64)

But first, let's make sure it works manually!

---

## Need More Details?

📖 **Complete guide:** [MANUAL_WORKFLOW.md](MANUAL_WORKFLOW.md)  
🔧 **Unraid specifics:** [UNRAID_SETUP.md](UNRAID_SETUP.md)  
🧪 **Safety testing:** [SAFE_TESTING.md](SAFE_TESTING.md)  
📊 **Understanding:** [COMPARISON.md](COMPARISON.md)

---

## Quick Troubleshooting

**Build fails?**
- Check Docker is running: `docker ps`
- Free up space: `docker system prune -a`

**Push fails?**
- Run `docker login` again
- Check repository exists on Docker Hub

**Container won't start on Unraid?**
- Check logs for errors
- Verify iDRAC IP is reachable from Unraid
- Test manually: `docker logs iDRAC-Fan-Controller`

---

**Ready to start?** → Run `./build-local.sh YOUR_USERNAME`

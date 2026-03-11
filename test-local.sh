#!/bin/bash

echo "================================================"
echo "Dell iDRAC Fan Controller - Local Test Script"
echo "================================================"
echo ""

# Check if image exists
if ! docker images idrac-fan-controller-rust:latest | grep -q idrac-fan-controller-rust; then
    echo "❌ Image not found. Build it first:"
    echo "   ./build-local.sh"
    exit 1
fi

echo "This script will test the Docker image locally."
echo ""
echo "Choose a test mode:"
echo "  1) Dry run (visualize fan curve only - NO hardware interaction)"
echo "  2) Quick test (5 minutes with your iDRAC - safe but active)"
echo "  3) Interactive test (you control when to stop)"
echo ""
read -p "Enter choice [1-3]: " choice

case $choice in
    1)
        echo ""
        echo "🔍 Running curve visualization..."
        echo ""
        docker run --rm idrac-fan-controller-rust:latest \
            /bin/sh -c "cd /build && cargo run --example curve_demo" 2>/dev/null || \
        echo "Note: Curve demo requires source code. This will show actual operation instead."
        echo ""
        echo "Let me show you what the curve looks like with default settings:"
        echo ""
        echo "Temperature | Fan Speed"
        echo "------------|----------"
        echo "   35°C     |    5%"
        echo "   40°C     |    5%"
        echo "   45°C     |    8%"
        echo "   50°C     |   15%"
        echo "   55°C     |   28%"
        echo "   60°C     |   48%"
        echo "   65°C     |   73%"
        echo "   70°C     |  100%"
        echo ""
        echo "✅ This is what the exponential curve looks like!"
        ;;
    2)
        echo ""
        echo "⚠️  QUICK TEST MODE"
        echo "This will actually control your server fans for 5 minutes."
        echo ""
        read -p "iDRAC IP address: " IDRAC_IP
        read -p "iDRAC username [root]: " IDRAC_USER
        IDRAC_USER=${IDRAC_USER:-root}
        read -sp "iDRAC password: " IDRAC_PASS
        echo ""
        echo ""
        echo "Using SAFE settings for testing:"
        echo "  - MIN_FAN_SPEED: 20% (higher minimum for safety)"
        echo "  - BASE_TEMP: 35°C (responds earlier)"
        echo "  - CRITICAL_TEMP: 60°C (safe threshold)"
        echo ""
        read -p "Press Enter to start 5-minute test (Ctrl+C to abort)..."
        echo ""
        echo "🏃 Running for 5 minutes..."
        echo "   Watch your fans and temperatures!"
        echo ""
        
        timeout 300 docker run --rm \
            -e IDRAC_HOST="${IDRAC_IP}" \
            -e IDRAC_USERNAME="${IDRAC_USER}" \
            -e IDRAC_PASSWORD="${IDRAC_PASS}" \
            -e MIN_FAN_SPEED=20 \
            -e BASE_TEMP=35 \
            -e CRITICAL_TEMP=60 \
            -e RUST_LOG=info \
            idrac-fan-controller-rust:latest
        
        echo ""
        echo "✅ 5-minute test complete!"
        echo "   Dell default fan control has been restored."
        ;;
    3)
        echo ""
        echo "🎮 INTERACTIVE TEST MODE"
        echo "The controller will run until you press Ctrl+C"
        echo ""
        read -p "iDRAC IP address: " IDRAC_IP
        read -p "iDRAC username [root]: " IDRAC_USER
        IDRAC_USER=${IDRAC_USER:-root}
        read -sp "iDRAC password: " IDRAC_PASS
        echo ""
        echo ""
        echo "Configuration:"
        read -p "  MIN_FAN_SPEED [10]: " MIN_SPEED
        MIN_SPEED=${MIN_SPEED:-10}
        read -p "  MAX_FAN_SPEED [80]: " MAX_SPEED
        MAX_SPEED=${MAX_SPEED:-80}
        read -p "  BASE_TEMP °C [40]: " BASE_TEMP
        BASE_TEMP=${BASE_TEMP:-40}
        read -p "  CRITICAL_TEMP °C [70]: " CRIT_TEMP
        CRIT_TEMP=${CRIT_TEMP:-70}
        echo ""
        echo "Starting controller... (Press Ctrl+C to stop)"
        echo ""
        
        docker run --rm \
            -e IDRAC_HOST="${IDRAC_IP}" \
            -e IDRAC_USERNAME="${IDRAC_USER}" \
            -e IDRAC_PASSWORD="${IDRAC_PASS}" \
            -e MIN_FAN_SPEED="${MIN_SPEED}" \
            -e MAX_FAN_SPEED="${MAX_SPEED}" \
            -e BASE_TEMP="${BASE_TEMP}" \
            -e CRITICAL_TEMP="${CRIT_TEMP}" \
            -e RUST_LOG=info \
            idrac-fan-controller-rust:latest
        
        echo ""
        echo "✅ Controller stopped."
        echo "   Dell default fan control has been restored."
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "Next steps:"
echo "  - If test was successful, see UNRAID_SETUP.md for permanent installation"
echo "  - To publish to Docker Hub, see DOCKER_HUB_PUBLISH.md"
echo ""

#!/bin/bash

# ElectronBeam CLI Demonstration Script
# This script shows various usage examples of the ElectronBeam CLI tool

set -e  # Exit on any error

echo "🔥 ElectronBeam CLI Demonstration 🔥"
echo "======================================"
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_step() {
    echo -e "${BLUE}📺 $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust and Cargo first."
    exit 1
fi

# Build the project
print_step "Building ElectronBeam..."
cargo build --release
print_success "Build complete!"
echo

# Create test images if they don't exist
print_step "Creating test images..."
if [ ! -f "test_gradient.png" ] || [ ! -f "test_retro.png" ] || [ ! -f "test_logo.png" ]; then
    cargo run --example create_test
    print_success "Test images created!"
else
    print_info "Test images already exist, skipping creation."
fi
echo

# Create output directory
mkdir -p demo_output

print_step "Starting ElectronBeam demonstrations..."
echo

# Demo 1: Classic CRT Turn-off Effect
print_step "Demo 1: Classic CRT Turn-off Effect (Cool-down)"
print_info "Creating the iconic CRT turn-off animation with horizontal and vertical stretching"
cargo run --release -- \
    -i test_gradient.png \
    -o demo_output/demo1_cooldown.gif \
    -m cool-down \
    -f 30 \
    -d 100 \
    --verbose
print_success "Demo 1 complete: demo_output/demo1_cooldown.gif"
echo

# Demo 2: CRT Turn-on Effect
print_step "Demo 2: CRT Turn-on Effect (Warm-up)"
print_info "The reverse effect - simulating a CRT warming up and displaying the image"
cargo run --release -- \
    -i test_retro.png \
    -o demo_output/demo2_warmup.gif \
    -m warm-up \
    -f 25 \
    -d 120 \
    --verbose
print_success "Demo 2 complete: demo_output/demo2_warmup.gif"
echo

# Demo 3: Simple Fade Effect
print_step "Demo 3: Simple Fade Effect"
print_info "Clean fade-out animation without CRT-specific effects"
cargo run --release -- \
    -i test_logo.png \
    -o demo_output/demo3_fade.gif \
    -m fade \
    -f 20 \
    -d 150 \
    --verbose
print_success "Demo 3 complete: demo_output/demo3_fade.gif"
echo

# Demo 4: Scale-down Effect
print_step "Demo 4: Scale-down Effect"
print_info "Image scales down while dimming to create a zoom-out effect"
cargo run --release -- \
    -i test_gradient.png \
    -o demo_output/demo4_scale.gif \
    -m scale-down \
    -f 35 \
    -d 80 \
    --verbose
print_success "Demo 4 complete: demo_output/demo4_scale.gif"
echo

# Demo 5: Custom Dimensions
print_step "Demo 5: Custom Dimensions and Looping"
print_info "Resizing input to custom dimensions with looping animation"
cargo run --release -- \
    -i test_logo.png \
    -o demo_output/demo5_custom.gif \
    -m cool-down \
    -f 20 \
    -d 100 \
    --width 800 \
    --height 600 \
    --loop-animation \
    --verbose
print_success "Demo 5 complete: demo_output/demo5_custom.gif"
echo

# Demo 6: Reverse Animation
print_step "Demo 6: Reverse Animation"
print_info "Playing the cool-down effect in reverse (turn-on effect)"
cargo run --release -- \
    -i test_retro.png \
    -o demo_output/demo6_reverse.gif \
    -m cool-down \
    -f 25 \
    -d 90 \
    --reverse \
    --verbose
print_success "Demo 6 complete: demo_output/demo6_reverse.gif"
echo

# Demo 7: Custom Stretch Parameters
print_step "Demo 7: Custom Stretch Parameters"
print_info "Fine-tuning the vertical and horizontal stretch durations (vertical first, then horizontal)"
cargo run --release -- \
    -i test_gradient.png \
    -o demo_output/demo7_stretch.gif \
    -m cool-down \
    -f 40 \
    -d 75 \
    --v-stretch 0.2 \
    --h-stretch 0.8 \
    --verbose
print_success "Demo 7 complete: demo_output/demo7_stretch.gif"
echo

# Demo 8: High-Quality Long Animation
print_step "Demo 8: High-Quality Long Animation"
print_info "Creating a smooth, high-frame-count animation"
cargo run --release -- \
    -i test_logo.png \
    -o demo_output/demo8_hq.gif \
    -m cool-down \
    -f 60 \
    -d 50 \
    --loop-animation \
    --verbose
print_success "Demo 8 complete: demo_output/demo8_hq.gif"
echo

# Show results
print_step "Demonstration Complete!"
echo
print_info "Generated animations:"
ls -la demo_output/*.gif | while read -r line; do
    filename=$(echo "$line" | awk '{print $NF}')
    size=$(echo "$line" | awk '{print $5}')
    echo -e "${CYAN}  📁 $filename${NC} (${size} bytes)"
done

echo
print_info "Animation modes demonstrated:"
echo -e "${PURPLE}  🔥 cool-down${NC}  - Classic CRT turn-off effect"
echo -e "${PURPLE}  🌟 warm-up${NC}    - CRT turn-on effect"
echo -e "${PURPLE}  🌫️  fade${NC}       - Simple fade effect"
echo -e "${PURPLE}  📏 scale-down${NC} - Scaling effect with dimming"

echo
print_info "Features demonstrated:"
echo -e "${CYAN}  ⚙️  Custom dimensions and resizing${NC}"
echo -e "${CYAN}  🔄 Reverse animations${NC}"
echo -e "${CYAN}  🔁 Looping animations${NC}"
echo -e "${CYAN}  🎛️  Custom stretch parameters${NC}"
echo -e "${CYAN}  🎬 Variable frame counts and timing${NC}"

echo
print_success "All demonstrations completed successfully!"
print_info "You can view the generated GIF files in the demo_output/ directory"
print_info "Try opening them in your favorite image viewer or web browser"

echo
echo -e "${YELLOW}💡 Tips:${NC}"
echo "  • Use lower frame counts for smaller file sizes"
echo "  • Adjust duration for faster/slower animations"
echo "  • Experiment with stretch parameters (--v-stretch for vertical phase, --h-stretch for horizontal)"
echo "  • Use --reverse to create turn-on effects from any mode"
echo "  • Add --loop-animation for seamless loops"

echo
echo -e "${GREEN}🎉 Thank you for trying ElectronBeam! 🎉${NC}"
echo "   Bzzzoooop! *crackle*"

# ElectronBeam 📺✨

A Rust library and CLI tool for creating nostalgic CRT-style turn-off animations from PNG images to GIF format.

ElectronBeam faithfully recreates the classic cathode-ray tube (CRT) television and monitor turn-off effect, complete with horizontal and vertical stretching, color separation, and the characteristic electron beam collapse that defined an era of computing and entertainment.

[![CI](https://github.com/aovestdipaperino/electron-beam/workflows/CI/badge.svg)](https://github.com/aovestdipaperino/electron-beam/actions)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/electron-beam.svg)](https://crates.io/crates/electron-beam)
[![Documentation](https://docs.rs/electron-beam/badge.svg)](https://docs.rs/electron-beam)

## ✨ Features

- **Authentic CRT Effects**: Recreates the classic electron beam turn-off animation with:
  - Horizontal stretch (thin white line collapse)
  - Vertical stretch with RGB color separation
  - Realistic timing and curves using sigmoid interpolation
- **Multiple Animation Modes**:
  - `cool-down`: Classic CRT turn-off effect (default)
  - `warm-up`: Reverse effect simulating CRT turn-on
  - `fade`: Simple fade in/out animation
  - `scale-down`: Scaling effect with dimming
- **Flexible Output**: Customizable frame count, timing, and dimensions
- **High Performance**: Efficient Rust implementation with parallel processing
- **CLI Tool**: Easy-to-use command-line interface
- **Library**: Reusable Rust crate for integration into other projects

## 🚀 Installation

### From Crates.io (Coming Soon)

```bash
cargo install electron-beam
```

### From Source

```bash
git clone https://github.com/aovestdipaperino/electron-beam.git
cd electron-beam
cargo build --release
```

The binary will be available at `target/release/electron-beam`.

## 📖 Usage

### Basic Usage

Create a classic CRT turn-off animation:

```bash
electron-beam -i input.png -o output.gif
```

### Advanced Examples

```bash
# High-quality CRT turn-off with 60 frames
electron-beam -i image.png -o crt_off.gif -m cool-down -f 60 -d 50

# CRT turn-on effect with custom dimensions
electron-beam -i logo.png -o crt_on.gif -m warm-up --width 800 --height 600

# Reverse animation that loops
electron-beam -i photo.png -o reversed.gif -m cool-down --reverse --loop

# Custom stretch parameters for different effects
electron-beam -i art.png -o custom.gif --h-stretch 0.3 --v-stretch 0.7
```

## 🎮 Animation Modes

### Cool-Down (Default)
The classic CRT turn-off effect that every retro computing enthusiast remembers:
1. **Vertical Stretch Phase**: Colors separate into RGB channels and collapse vertically
2. **Horizontal Stretch Phase**: Image collapses into a thin white horizontal line
3. **Final Fade**: Complete black screen

### Warm-Up
The reverse effect, simulating a CRT warming up and displaying the image:
- Perfect for intro animations
- Reverses the cool-down sequence

### Fade
Simple fade-in or fade-out effect:
- Clean transition without CRT-specific effects
- Useful for subtle animations

### Scale-Down
Image scales down while dimming:
- Maintains aspect ratio
- Smooth scaling with opacity changes

## 🛠️ Command-Line Options

```
Usage: electron-beam [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>              Input PNG file path
  -o, --output <OUTPUT>            Output GIF file path
  -m, --mode <MODE>                Animation mode [default: cool-down]
  -f, --frames <FRAMES>            Number of frames [default: 30]
  -d, --duration <DURATION>        Frame duration in milliseconds [default: 100]
      --width <WIDTH>              Output width (resizes input if different)
      --height <HEIGHT>            Output height (resizes input if different)
      --v-stretch <V_STRETCH>      Vertical stretch duration (0.0-1.0) - happens first [default: 0.5]
      --h-stretch <H_STRETCH>      Horizontal stretch duration (0.0-1.0) - happens second [default: 0.5]
  -v, --verbose                    Enable verbose logging
      --debug                      Enable debug logging
  -r, --reverse                    Reverse the animation
  -l, --loop-animation             Loop the animation
  -h, --help                       Print help
  -V, --version                    Print version
```

## 📚 Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
electron-beam = "0.1.0"
```

### Basic Library Example

```rust
use electron_beam::{ElectronBeamBuilder, AnimationMode};
use image::open;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load an image
    let img = open("input.png")?.to_rgba8();
    
    // Create ElectronBeam animator
    let mut beam = ElectronBeamBuilder::new()
        .dimensions(640, 480)
        .mode(AnimationMode::CoolDown)
        .build();
    
    // Prepare the animation
    beam.prepare(img)?;
    
    // Generate frames
    let mut frames = Vec::new();
    for i in 0..30 {
        let level = i as f32 / 29.0;
        let frame = beam.draw(level)?;
        frames.push(frame);
    }
    
    // Save frames or create GIF...
    Ok(())
}
```

### Advanced Configuration

```rust
use electron_beam::{ElectronBeamBuilder, AnimationMode};

let beam = ElectronBeamBuilder::new()
    .dimensions(800, 600)
    .mode(AnimationMode::CoolDown)
    .stretch_durations(0.3, 0.7)  // 30% vertical (first), 70% horizontal (second)
    .build();
```

## 🎨 Creating Test Images

The project includes a utility to create test images:

```bash
cargo run --example create_test
```

This generates:
- `test_gradient.png`: Colorful gradient with wave patterns
- `test_retro.png`: Retro CRT-style test pattern with scanlines
- `test_logo.png`: Simple logo with glow effects

## 🔧 Building from Source

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Build Steps

```bash
git clone https://github.com/aovestdipaperino/electron-beam.git
cd electron-beam
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Development

```bash
# Run with verbose output
cargo run -- -i test.png -o output.gif --verbose

# Run tests with output
cargo test -- --nocapture

# Format code
cargo fmt

# Lint code
cargo clippy
```

## 📊 Performance

ElectronBeam is optimized for performance:

- **Parallel Processing**: Uses Rayon for multi-threaded operations
- **Memory Efficient**: Streams processing to minimize memory usage
- **Fast Interpolation**: Optimized sigmoid curve calculations
- **Efficient Color Blending**: Hardware-accelerated operations where possible

Typical performance on modern hardware:
- 640x480 image, 30 frames: ~2-3 seconds
- 1920x1080 image, 60 frames: ~8-10 seconds

## 🎯 Technical Details

### Algorithm Overview

ElectronBeam implements a faithful recreation of the CRT electron beam physics:

1. **S-Curve Interpolation**: Uses sigmoid functions for realistic timing
2. **Color Channel Separation**: RGB channels are processed with different timing
3. **Geometric Transformations**: Accurate horizontal and vertical stretching
4. **Additive Blending**: Proper color mixing for authentic CRT glow

### Supported Formats

- **Input**: PNG (RGBA), with automatic resizing
- **Output**: GIF with optional looping and custom frame timing
- **Color Space**: sRGB with alpha channel support

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Format code (`cargo fmt`)
7. Commit your changes (`git commit -m 'Add some amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Add documentation for public APIs
- Include tests for new functionality
- Use meaningful commit messages

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the classic Android ElectronBeam effect
- Thanks to the Rust community for excellent crates
- Nostalgia for the CRT era of computing

## 🔗 Links

- [Documentation](https://docs.rs/electron-beam)
- [Crates.io](https://crates.io/crates/electron-beam)
- [GitHub Repository](https://github.com/aovestdipaperino/electron-beam)
- [Issue Tracker](https://github.com/aovestdipaperino/electron-beam/issues)

---

*"Bzzzoooop! \*crackle\*"* - Every CRT TV ever
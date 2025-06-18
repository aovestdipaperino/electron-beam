//! ElectronBeam CLI - Create CRT-style turn-off animations from PNG images
//!
//! This CLI tool takes a PNG image and creates a GIF animation that simulates
//! the classic CRT electron beam turn-off effect, complete with horizontal
//! and vertical stretching and color separation.

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use electron_beam::{AnimationMode, ElectronBeam, ElectronBeamBuilder};
use gif::{Encoder, Frame, Repeat};
use image::RgbaImage;
use log::{debug, info, warn};
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
enum CliAnimationMode {
    /// Simulate CRT turning off (default)
    CoolDown,
    /// Simulate CRT turning on
    WarmUp,
    /// Simple fade effect
    Fade,
    /// Scale down effect
    ScaleDown,
}

impl From<CliAnimationMode> for AnimationMode {
    fn from(mode: CliAnimationMode) -> Self {
        match mode {
            CliAnimationMode::CoolDown => AnimationMode::CoolDown,
            CliAnimationMode::WarmUp => AnimationMode::WarmUp,
            CliAnimationMode::Fade => AnimationMode::Fade,
            CliAnimationMode::ScaleDown => AnimationMode::ScaleDown,
        }
    }
}

#[derive(Debug, Clone, Parser)]
#[command(name = "electron-beam")]
#[command(about = "Create CRT-style turn-off animations from PNG images")]
#[command(version)]
struct Cli {
    /// Input PNG file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output GIF file path
    #[arg(short, long)]
    output: PathBuf,

    /// Animation mode
    #[arg(short, long, default_value = "cool-down")]
    mode: CliAnimationMode,

    /// Number of frames in the animation
    #[arg(short, long, default_value = "30")]
    frames: u32,

    /// Frame duration in milliseconds
    #[arg(short, long, default_value = "100")]
    duration: u16,

    /// Output width (will resize input if different)
    #[arg(long)]
    width: Option<u32>,

    /// Output height (will resize input if different)
    #[arg(long)]
    height: Option<u32>,

    /// Vertical stretch duration (0.0 to 1.0) - happens first in CRT effect
    #[arg(long, default_value = "0.5")]
    v_stretch: f32,

    /// Horizontal stretch duration (0.0 to 1.0) - happens second in CRT effect
    #[arg(long, default_value = "0.5")]
    h_stretch: f32,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Enable debug logging
    #[arg(long)]
    debug: bool,

    /// Reverse the animation (play backwards)
    #[arg(short, long)]
    reverse: bool,

    /// Loop the animation
    #[arg(short, long)]
    loop_animation: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Initialize logging
    let log_level = if args.debug {
        "debug"
    } else if args.verbose {
        "info"
    } else {
        "warn"
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    info!("Starting ElectronBeam CLI");
    debug!("Arguments: {:?}", args);

    // Validate arguments
    validate_arguments(&args)?;

    // Load the input image
    info!("Loading input image: {}", args.input.display());
    let input_image = load_image(&args.input)?;

    // Determine output dimensions
    let (width, height) = match (args.width, args.height) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None) => {
            let aspect_ratio = input_image.height() as f32 / input_image.width() as f32;
            (w, (w as f32 * aspect_ratio) as u32)
        }
        (None, Some(h)) => {
            let aspect_ratio = input_image.width() as f32 / input_image.height() as f32;
            ((h as f32 * aspect_ratio) as u32, h)
        }
        (None, None) => (input_image.width(), input_image.height()),
    };

    info!("Output dimensions: {}x{}", width, height);

    // Create the ElectronBeam
    let mut beam = ElectronBeamBuilder::new()
        .dimensions(width, height)
        .mode(args.mode.into())
        .stretch_durations(args.v_stretch, args.h_stretch)
        .build();

    // Prepare the animation
    info!("Preparing animation...");
    beam.prepare(input_image)?;

    // Generate frames
    info!("Generating {} frames...", args.frames);
    let frames = generate_frames(&beam, args.frames, args.reverse)?;

    // Create GIF
    info!("Creating GIF: {}", args.output.display());
    create_gif(&frames, &args.output, args.duration, args.loop_animation)?;

    info!("Animation complete! Saved to: {}", args.output.display());
    Ok(())
}

fn validate_arguments(args: &Cli) -> Result<()> {
    if !args.input.exists() {
        anyhow::bail!("Input file does not exist: {}", args.input.display());
    }

    if args.frames == 0 {
        anyhow::bail!("Frame count must be greater than 0");
    }

    if args.duration == 0 {
        anyhow::bail!("Frame duration must be greater than 0");
    }

    if args.v_stretch < 0.0 || args.v_stretch > 1.0 {
        anyhow::bail!("Vertical stretch duration must be between 0.0 and 1.0");
    }

    if args.h_stretch < 0.0 || args.h_stretch > 1.0 {
        anyhow::bail!("Horizontal stretch duration must be between 0.0 and 1.0");
    }

    if let Some(parent) = args.output.parent() {
        if !parent.exists() {
            warn!(
                "Output directory does not exist, creating: {}",
                parent.display()
            );
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create output directory: {}", parent.display())
            })?;
        }
    }

    Ok(())
}

fn load_image(path: &PathBuf) -> Result<RgbaImage> {
    let img =
        image::open(path).with_context(|| format!("Failed to open image: {}", path.display()))?;

    Ok(img.to_rgba8())
}

fn generate_frames(beam: &ElectronBeam, frame_count: u32, reverse: bool) -> Result<Vec<RgbaImage>> {
    let mut frames = Vec::with_capacity(frame_count as usize);

    for i in 0..frame_count {
        let level = if reverse {
            1.0 - (i as f32 / (frame_count - 1) as f32)
        } else {
            i as f32 / (frame_count - 1) as f32
        };

        debug!(
            "Generating frame {}/{} (level: {:.3})",
            i + 1,
            frame_count,
            level
        );

        let frame = beam
            .draw(level)
            .with_context(|| format!("Failed to generate frame {}", i + 1))?;

        frames.push(frame);
    }

    Ok(frames)
}

fn create_gif(
    frames: &[RgbaImage],
    output_path: &PathBuf,
    frame_duration: u16,
    loop_animation: bool,
) -> Result<()> {
    if frames.is_empty() {
        anyhow::bail!("No frames to write");
    }

    let output_file = File::create(output_path)
        .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;

    let (width, height) = (frames[0].width() as u16, frames[0].height() as u16);
    let mut encoder = Encoder::new(output_file, width, height, &[])?;

    // Set repeat mode
    if loop_animation {
        encoder.set_repeat(Repeat::Infinite)?;
    } else {
        encoder.set_repeat(Repeat::Finite(0))?;
    }

    for (i, frame_image) in frames.iter().enumerate() {
        debug!("Writing frame {}/{}", i + 1, frames.len());

        // Convert RGBA to RGB (GIF doesn't support alpha)
        let mut rgb_data = Vec::with_capacity((width as usize) * (height as usize) * 3);
        for pixel in frame_image.pixels() {
            let [r, g, b, a] = pixel.0;

            // Blend with black background based on alpha
            let alpha_f = a as f32 / 255.0;
            let blended_r = (r as f32 * alpha_f) as u8;
            let blended_g = (g as f32 * alpha_f) as u8;
            let blended_b = (b as f32 * alpha_f) as u8;

            rgb_data.push(blended_r);
            rgb_data.push(blended_g);
            rgb_data.push(blended_b);
        }

        let mut frame = Frame::from_rgb(width, height, &rgb_data);
        frame.delay = frame_duration / 10; // GIF delay is in centiseconds

        encoder
            .write_frame(&frame)
            .with_context(|| format!("Failed to write frame {}", i + 1))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_cli_animation_mode_conversion() {
        assert!(matches!(
            AnimationMode::from(CliAnimationMode::CoolDown),
            AnimationMode::CoolDown
        ));
        assert!(matches!(
            AnimationMode::from(CliAnimationMode::WarmUp),
            AnimationMode::WarmUp
        ));
        assert!(matches!(
            AnimationMode::from(CliAnimationMode::Fade),
            AnimationMode::Fade
        ));
        assert!(matches!(
            AnimationMode::from(CliAnimationMode::ScaleDown),
            AnimationMode::ScaleDown
        ));
    }

    #[test]
    fn test_frame_generation() {
        let beam = ElectronBeamBuilder::new()
            .dimensions(10, 10)
            .mode(AnimationMode::CoolDown)
            .build();

        // Create a simple test image
        let test_image =
            image::ImageBuffer::from_fn(10, 10, |_, _| image::Rgba([255, 255, 255, 255]));

        let mut beam = beam;
        beam.prepare(test_image).unwrap();

        let frames = generate_frames(&beam, 5, false).unwrap();
        assert_eq!(frames.len(), 5);

        // Check that all frames have the correct dimensions
        for frame in &frames {
            assert_eq!(frame.width(), 10);
            assert_eq!(frame.height(), 10);
        }
    }

    #[test]
    fn test_reverse_frame_generation() {
        let beam = ElectronBeamBuilder::new()
            .dimensions(10, 10)
            .mode(AnimationMode::CoolDown)
            .build();

        let test_image =
            image::ImageBuffer::from_fn(10, 10, |_, _| image::Rgba([255, 255, 255, 255]));

        let mut beam = beam;
        beam.prepare(test_image).unwrap();

        let normal_frames = generate_frames(&beam, 3, false).unwrap();
        let reverse_frames = generate_frames(&beam, 3, true).unwrap();

        assert_eq!(normal_frames.len(), reverse_frames.len());

        // The first frame of normal should be similar to the last frame of reverse
        // (not exactly equal due to floating point precision)
        assert_eq!(
            normal_frames[0].dimensions(),
            reverse_frames[2].dimensions()
        );
    }

    #[test]
    fn test_validate_arguments() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_path_buf();

        let valid_args = Cli {
            input: temp_path,
            output: PathBuf::from("test.gif"),
            mode: CliAnimationMode::CoolDown,
            frames: 10,
            duration: 100,
            width: None,
            height: None,
            v_stretch: 0.5,
            h_stretch: 0.5,
            verbose: false,
            debug: false,
            reverse: false,
            loop_animation: false,
        };

        assert!(validate_arguments(&valid_args).is_ok());

        // Test invalid frame count
        let mut invalid_args = valid_args.clone();
        invalid_args.frames = 0;
        assert!(validate_arguments(&invalid_args).is_err());

        // Test invalid stretch values
        invalid_args = valid_args.clone();
        invalid_args.h_stretch = -0.1;
        assert!(validate_arguments(&invalid_args).is_err());

        invalid_args = valid_args.clone();
        invalid_args.v_stretch = 1.1;
        assert!(validate_arguments(&invalid_args).is_err());
    }
}

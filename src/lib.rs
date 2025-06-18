//! ElectronBeam - A library for creating CRT-style turn-off animations
//!
//! This library implements the classic CRT electron beam effect that simulates
//! the appearance of an old television or monitor turning off, with the characteristic
//! horizontal and vertical stretching and color separation effects.

use anyhow::Result;

use image::{ImageBuffer, Rgba, RgbaImage};

/// Errors that can occur during ElectronBeam operations
#[derive(Debug, thiserror::Error)]
pub enum ElectronBeamError {
    #[error("Invalid animation mode: {0}")]
    InvalidMode(String),
    #[error("Invalid level value: {0} (must be between 0.0 and 1.0)")]
    InvalidLevel(f32),
    #[error("Image processing error: {0}")]
    ImageError(String),
    #[error("Animation not prepared")]
    NotPrepared,
}

/// Animation modes for the ElectronBeam effect
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationMode {
    /// Animates an electron beam warming up (turning on)
    WarmUp,
    /// Animates an electron beam shutting off (turning off)
    CoolDown,
    /// Simple fade effect
    Fade,
    /// Scale down effect
    ScaleDown,
}

/// Configuration for the ElectronBeam animation
#[derive(Debug, Clone)]
pub struct ElectronBeamConfig {
    /// Width of the output frames
    pub width: u32,
    /// Height of the output frames
    pub height: u32,
    /// Animation mode to use
    pub mode: AnimationMode,
    /// Duration of the vertical stretch effect (0.0 to 1.0) - happens first
    pub v_stretch_duration: f32,
    /// Duration of the horizontal stretch effect (0.0 to 1.0) - happens second
    pub h_stretch_duration: f32,
}

impl Default for ElectronBeamConfig {
    fn default() -> Self {
        Self {
            width: 640,
            height: 480,
            mode: AnimationMode::CoolDown,
            v_stretch_duration: 0.5,
            h_stretch_duration: 0.5,
        }
    }
}

/// The main ElectronBeam struct that handles CRT-style animations
pub struct ElectronBeam {
    config: ElectronBeamConfig,
    source_image: Option<RgbaImage>,
    prepared: bool,
}

impl ElectronBeam {
    /// Create a new ElectronBeam instance
    pub fn new(config: ElectronBeamConfig) -> Self {
        Self {
            config,
            source_image: None,
            prepared: false,
        }
    }

    /// Prepare the animation with a source image
    pub fn prepare(&mut self, image: RgbaImage) -> Result<()> {
        // Resize image to match config dimensions if needed
        let resized_image =
            if image.width() != self.config.width || image.height() != self.config.height {
                image::imageops::resize(
                    &image,
                    self.config.width,
                    self.config.height,
                    image::imageops::FilterType::Lanczos3,
                )
            } else {
                image
            };

        self.source_image = Some(resized_image);
        self.prepared = true;
        Ok(())
    }

    /// Generate a frame at the specified animation level (0.0 to 1.0)
    pub fn draw(&self, level: f32) -> Result<RgbaImage> {
        if !self.prepared {
            return Err(ElectronBeamError::NotPrepared.into());
        }

        if level < 0.0 || level > 1.0 {
            return Err(ElectronBeamError::InvalidLevel(level).into());
        }

        let source = self.source_image.as_ref().unwrap();
        let mut output = ImageBuffer::new(self.config.width, self.config.height);

        match self.config.mode {
            AnimationMode::Fade => self.draw_fade(source, &mut output, level),
            AnimationMode::ScaleDown => self.draw_scale_down(source, &mut output, level),
            AnimationMode::WarmUp | AnimationMode::CoolDown => {
                if level < self.config.v_stretch_duration {
                    self.draw_v_stretch(source, &mut output, level / self.config.v_stretch_duration)
                } else {
                    let h_level =
                        (level - self.config.v_stretch_duration) / self.config.h_stretch_duration;
                    self.draw_h_stretch(source, &mut output, h_level)
                }
            }
        }

        Ok(output)
    }

    /// Draw a simple fade effect
    fn draw_fade(&self, source: &RgbaImage, output: &mut RgbaImage, level: f32) {
        let alpha = if self.config.mode == AnimationMode::WarmUp {
            level
        } else {
            1.0 - level
        };

        for (x, y, pixel) in source.enumerate_pixels() {
            let mut new_pixel = *pixel;
            new_pixel[3] = (new_pixel[3] as f32 * alpha) as u8;
            output.put_pixel(x, y, new_pixel);
        }
    }

    /// Draw the scale down effect
    fn draw_scale_down(&self, source: &RgbaImage, output: &mut RgbaImage, level: f32) {
        // Clear to black
        for pixel in output.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 255]);
        }

        let curved_scale = self.scurve(level, 8.0);
        let scale = if self.config.mode == AnimationMode::WarmUp {
            curved_scale
        } else {
            1.0 - curved_scale
        };

        let new_width = (self.config.width as f32 * scale) as u32;
        let new_height = (self.config.height as f32 * scale) as u32;

        if new_width > 0 && new_height > 0 {
            let scaled = image::imageops::resize(
                source,
                new_width,
                new_height,
                image::imageops::FilterType::Lanczos3,
            );

            let offset_x = (self.config.width - new_width) / 2;
            let offset_y = (self.config.height - new_height) / 2;

            // Copy scaled image to center of output
            for (x, y, pixel) in scaled.enumerate_pixels() {
                let dest_x = x + offset_x;
                let dest_y = y + offset_y;
                if dest_x < self.config.width && dest_y < self.config.height {
                    // Apply dimming effect
                    let mut dimmed_pixel = *pixel;
                    let dim_factor = if self.config.mode == AnimationMode::WarmUp {
                        scale
                    } else {
                        scale * (1.0 - curved_scale * 0.5)
                    };

                    for i in 0..3 {
                        dimmed_pixel[i] = (dimmed_pixel[i] as f32 * dim_factor) as u8;
                    }

                    output.put_pixel(dest_x, dest_y, dimmed_pixel);
                }
            }
        }
    }

    /// Draw the horizontal stretch effect (thin white line)
    fn draw_h_stretch(&self, _source: &RgbaImage, output: &mut RgbaImage, stretch: f32) {
        // Clear to black
        for pixel in output.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 255]);
        }

        if stretch < 1.0 {
            let ag = self.scurve(stretch, 8.0);
            let width = 2.0 * self.config.width as f32 * (1.0 - ag);
            let height = 1.0f32.max(2.0);
            let x_start = ((self.config.width as f32 - width) * 0.5) as u32;
            let y_center = self.config.height / 2;
            let half_height = (height * 0.5) as u32;

            // Draw the horizontal line
            let intensity = 1.0 - ag * 0.75;
            let color_value = (255.0 * intensity) as u8;
            let line_color = Rgba([color_value, color_value, color_value, 255]);

            for x in x_start..((x_start as f32 + width) as u32).min(self.config.width) {
                for y in y_center.saturating_sub(half_height)
                    ..=(y_center + half_height).min(self.config.height - 1)
                {
                    output.put_pixel(x, y, line_color);
                }
            }
        }
    }

    /// Draw the vertical stretch effect with color separation
    fn draw_v_stretch(&self, source: &RgbaImage, output: &mut RgbaImage, stretch: f32) {
        // Clear to black
        for pixel in output.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 0]);
        }

        // Compute interpolation scale factors for each color channel
        let ar = self.scurve(stretch, 7.5);
        let ag = self.scurve(stretch, 8.0);
        let ab = self.scurve(stretch, 8.5);

        // Draw each color channel separately with different stretch factors
        self.draw_v_stretch_channel(source, output, ar, 0); // Red
        self.draw_v_stretch_channel(source, output, ag, 1); // Green
        self.draw_v_stretch_channel(source, output, ab, 2); // Blue

        // Add white highlight for cool down mode
        if self.config.mode == AnimationMode::CoolDown {
            let highlight_intensity = ag;
            self.add_highlight(output, highlight_intensity);
        }
    }

    /// Draw a single color channel with vertical stretch
    fn draw_v_stretch_channel(
        &self,
        source: &RgbaImage,
        output: &mut RgbaImage,
        stretch_factor: f32,
        channel: usize,
    ) {
        let width = self.config.width as f32 + (self.config.width as f32 * stretch_factor);
        let height = self.config.height as f32 - (self.config.height as f32 * stretch_factor);
        let x_offset = (self.config.width as f32 - width) * 0.5;
        let y_offset = (self.config.height as f32 - height) * 0.5;

        // Sample and stretch the source image
        for y in 0..self.config.height {
            for x in 0..self.config.width {
                // Map output coordinates back to source coordinates
                let src_x = if width > 0.0 {
                    ((x as f32 - x_offset) / width * self.config.width as f32)
                        .max(0.0)
                        .min(self.config.width as f32 - 1.0)
                } else {
                    self.config.width as f32 * 0.5
                };

                let src_y = if height > 0.0 {
                    ((y as f32 - y_offset) / height * self.config.height as f32)
                        .max(0.0)
                        .min(self.config.height as f32 - 1.0)
                } else {
                    self.config.height as f32 * 0.5
                };

                // Check if we're within the stretched bounds
                if (x as f32) >= x_offset
                    && (x as f32) < (x_offset + width)
                    && (y as f32) >= y_offset
                    && (y as f32) < (y_offset + height)
                {
                    let src_pixel = source.get_pixel(src_x as u32, src_y as u32);
                    let mut dest_pixel = *output.get_pixel(x, y);

                    // Blend the channel (additive blending for the CRT effect)
                    let channel_value = src_pixel[channel];
                    dest_pixel[channel] =
                        (dest_pixel[channel] as u16 + channel_value as u16).min(255) as u8;
                    dest_pixel[3] = 255; // Full alpha

                    output.put_pixel(x, y, dest_pixel);
                }
            }
        }
    }

    /// Add white highlight effect
    fn add_highlight(&self, output: &mut RgbaImage, intensity: f32) {
        let highlight_value = (255.0 * intensity) as u8;

        for pixel in output.pixels_mut() {
            // Add white highlight while preserving existing colors
            for i in 0..3 {
                pixel[i] = (pixel[i] as u16 + highlight_value as u16).min(255) as u8;
            }
        }
    }

    /// S-curve interpolation function
    /// Interpolates a value in the range 0..1 along a sigmoid curve
    fn scurve(&self, value: f32, s: f32) -> f32 {
        let x = value - 0.5;
        let y = self.sigmoid(x, s) - 0.5;
        let v = self.sigmoid(0.5, s) - 0.5;
        y / v * 0.5 + 0.5
    }

    /// Sigmoid function
    fn sigmoid(&self, x: f32, s: f32) -> f32 {
        1.0 / (1.0 + (-x * s).exp())
    }

    /// Get the configuration
    pub fn config(&self) -> &ElectronBeamConfig {
        &self.config
    }

    /// Check if the animation is prepared
    pub fn is_prepared(&self) -> bool {
        self.prepared
    }

    /// Reset the animation state
    pub fn reset(&mut self) {
        self.source_image = None;
        self.prepared = false;
    }
}

/// Builder pattern for ElectronBeam configuration
pub struct ElectronBeamBuilder {
    config: ElectronBeamConfig,
}

impl ElectronBeamBuilder {
    pub fn new() -> Self {
        Self {
            config: ElectronBeamConfig::default(),
        }
    }

    pub fn dimensions(mut self, width: u32, height: u32) -> Self {
        self.config.width = width;
        self.config.height = height;
        self
    }

    pub fn mode(mut self, mode: AnimationMode) -> Self {
        self.config.mode = mode;
        self
    }

    pub fn stretch_durations(mut self, v_duration: f32, h_duration: f32) -> Self {
        self.config.v_stretch_duration = v_duration;
        self.config.h_stretch_duration = h_duration;
        self
    }

    pub fn build(self) -> ElectronBeam {
        ElectronBeam::new(self.config)
    }
}

impl Default for ElectronBeamBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electron_beam_creation() {
        let beam = ElectronBeamBuilder::new()
            .dimensions(320, 240)
            .mode(AnimationMode::CoolDown)
            .build();

        assert_eq!(beam.config().width, 320);
        assert_eq!(beam.config().height, 240);
        assert_eq!(beam.config().mode, AnimationMode::CoolDown);
        assert!(!beam.is_prepared());
    }

    #[test]
    fn test_scurve_function() {
        let beam = ElectronBeamBuilder::new().build();

        // Test boundary conditions
        assert!((beam.scurve(0.0, 8.0) - 0.0).abs() < 0.01);
        assert!((beam.scurve(0.5, 8.0) - 0.5).abs() < 0.01);
        assert!((beam.scurve(1.0, 8.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_invalid_level() {
        let beam = ElectronBeamBuilder::new().build();

        // These should return errors for invalid levels
        assert!(beam.draw(-0.1).is_err());
        assert!(beam.draw(1.1).is_err());
    }

    #[test]
    fn test_prepare_with_image() {
        let mut beam = ElectronBeamBuilder::new().dimensions(100, 100).build();

        let test_image = ImageBuffer::from_fn(50, 50, |_, _| Rgba([255, 255, 255, 255]));

        assert!(beam.prepare(test_image).is_ok());
        assert!(beam.is_prepared());
    }
}

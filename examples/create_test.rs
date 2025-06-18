//! Simple test program to create a PNG image for testing the ElectronBeam CLI
//!
//! Run with: cargo run --example create_test

use image::{ImageBuffer, Rgba, RgbaImage};
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating test images for ElectronBeam CLI...");

    // Create a colorful gradient image
    create_gradient_image("test_gradient.png", 640, 480)?;

    // Create a retro-style image with patterns
    create_retro_image("test_retro.png", 320, 240)?;

    // Create a simple logo-style image
    create_logo_image("test_logo.png", 400, 300)?;

    println!("Test images created successfully!");
    println!("\nYou can now test the CLI with:");
    println!("cargo run --release -- -i test_gradient.png -o output_cooldown.gif -m cool-down -f 30 -d 100 --verbose");
    println!("cargo run --release -- -i test_retro.png -o output_warmup.gif -m warm-up -f 20 -d 120 --verbose");
    println!(
        "cargo run --release -- -i test_logo.png -o output_fade.gif -m fade -f 25 -d 80 --verbose"
    );
    println!("cargo run --release -- -i test_gradient.png -o output_scale.gif -m scale-down -f 35 -d 60 --verbose");
    println!("cargo run --release -- -i test_logo.png -o output_custom_stretch.gif -m cool-down -f 30 -d 80 --v-stretch 0.3 --h-stretch 0.7 --verbose");

    Ok(())
}

fn create_gradient_image(
    filename: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Create a colorful gradient
        let r = (255.0 * x as f32 / width as f32) as u8;
        let g = (255.0 * y as f32 / height as f32) as u8;
        let b = (255.0 * ((x + y) as f32 / (width + height) as f32)) as u8;

        // Add some wave patterns
        let wave_x = (2.0 * PI * x as f32 / width as f32 * 4.0).sin();
        let wave_y = (2.0 * PI * y as f32 / height as f32 * 3.0).sin();
        let wave_intensity = ((wave_x + wave_y) * 0.5 + 1.0) * 0.5;

        let final_r = (r as f32 * (0.7 + 0.3 * wave_intensity)) as u8;
        let final_g = (g as f32 * (0.7 + 0.3 * wave_intensity)) as u8;
        let final_b = (b as f32 * (0.7 + 0.3 * wave_intensity)) as u8;

        *pixel = Rgba([final_r, final_g, final_b, 255]);
    }

    img.save(filename)?;
    println!("Created gradient image: {}", filename);
    Ok(())
}

fn create_retro_image(
    filename: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = ImageBuffer::new(width, height);

    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = x as f32 - cx;
        let dy = y as f32 - cy;
        let distance = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx);

        // Create concentric circles with different colors
        let circle_index = (distance / 15.0) as u32 % 4;
        let (base_r, base_g, base_b) = match circle_index {
            0 => (255, 80, 80),   // Red
            1 => (80, 255, 80),   // Green
            2 => (80, 80, 255),   // Blue
            3 => (255, 255, 80),  // Yellow
            _ => (255, 255, 255), // White
        };

        // Add radial pattern
        let radial_pattern = (angle * 8.0).sin();
        let radial_intensity = (radial_pattern * 0.5 + 1.0) * 0.5;

        // Add scanlines for retro CRT effect
        let scanline_factor = if y % 3 == 0 { 0.6 } else { 1.0 };

        let final_r = (base_r as f32 * radial_intensity * scanline_factor) as u8;
        let final_g = (base_g as f32 * radial_intensity * scanline_factor) as u8;
        let final_b = (base_b as f32 * radial_intensity * scanline_factor) as u8;

        *pixel = Rgba([final_r, final_g, final_b, 255]);
    }

    img.save(filename)?;
    println!("Created retro image: {}", filename);
    Ok(())
}

fn create_logo_image(
    filename: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = ImageBuffer::new(width, height);

    // Fill with dark background
    for pixel in img.pixels_mut() {
        *pixel = Rgba([20, 20, 40, 255]);
    }

    let cx = width / 2;
    let cy = height / 2;

    // Draw a stylized "E" for ElectronBeam
    let stroke_width = 10;
    let letter_width = 80;
    let letter_height = 100;

    let bright_cyan = Rgba([0, 255, 255, 255]);
    let dim_cyan = Rgba([0, 180, 180, 255]);

    // Vertical line of E (with gradient effect)
    for y in (cy - letter_height / 2)..(cy + letter_height / 2) {
        for x in (cx - letter_width / 2)..(cx - letter_width / 2 + stroke_width) {
            if x < width && y < height {
                // Create a gradient effect
                let gradient = (y - (cy - letter_height / 2)) as f32 / letter_height as f32;
                let color = interpolate_color(bright_cyan, dim_cyan, gradient);
                img.put_pixel(x, y, color);
            }
        }
    }

    // Top horizontal line
    for x in (cx - letter_width / 2)..(cx + letter_width / 3 * 2) {
        for y in (cy - letter_height / 2)..(cy - letter_height / 2 + stroke_width) {
            if x < width && y < height {
                img.put_pixel(x, y, bright_cyan);
            }
        }
    }

    // Middle horizontal line
    for x in (cx - letter_width / 2)..(cx + letter_width / 4) {
        for y in (cy - stroke_width / 2)..(cy + stroke_width / 2) {
            if x < width && y < height {
                img.put_pixel(x, y, bright_cyan);
            }
        }
    }

    // Bottom horizontal line
    for x in (cx - letter_width / 2)..(cx + letter_width / 3 * 2) {
        for y in (cy + letter_height / 2 - stroke_width)..(cy + letter_height / 2) {
            if x < width && y < height {
                img.put_pixel(x, y, bright_cyan);
            }
        }
    }

    // Add some glow effect around the letter
    add_glow_effect(&mut img, cx, cy, letter_width, letter_height);

    // Add some decorative elements
    add_decorative_elements(&mut img, width, height);

    img.save(filename)?;
    println!("Created logo image: {}", filename);
    Ok(())
}

fn interpolate_color(color1: Rgba<u8>, color2: Rgba<u8>, t: f32) -> Rgba<u8> {
    let t = t.clamp(0.0, 1.0);
    let r = (color1[0] as f32 * (1.0 - t) + color2[0] as f32 * t) as u8;
    let g = (color1[1] as f32 * (1.0 - t) + color2[1] as f32 * t) as u8;
    let b = (color1[2] as f32 * (1.0 - t) + color2[2] as f32 * t) as u8;
    let a = (color1[3] as f32 * (1.0 - t) + color2[3] as f32 * t) as u8;
    Rgba([r, g, b, a])
}

fn add_glow_effect(img: &mut RgbaImage, cx: u32, cy: u32, width: u32, height: u32) {
    let glow_radius = 25;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = (x as i32 - cx as i32).abs() as f32;
        let dy = (y as i32 - cy as i32).abs() as f32;

        if dx < (width / 2 + glow_radius) as f32 && dy < (height / 2 + glow_radius) as f32 {
            let distance = (dx * dx + dy * dy).sqrt();
            let glow_strength = (1.0 - (distance / glow_radius as f32).min(1.0)) * 0.2;

            if glow_strength > 0.0 {
                let current = pixel.0;
                let glow_r = (30.0 * glow_strength) as u8;
                let glow_g = (60.0 * glow_strength) as u8;
                let glow_b = (60.0 * glow_strength) as u8;

                pixel.0[0] = (current[0] as u16 + glow_r as u16).min(255) as u8;
                pixel.0[1] = (current[1] as u16 + glow_g as u16).min(255) as u8;
                pixel.0[2] = (current[2] as u16 + glow_b as u16).min(255) as u8;
            }
        }
    }
}

fn add_decorative_elements(img: &mut RgbaImage, width: u32, height: u32) {
    // Add some corner decorations (small dots)
    let corner_color = Rgba([100, 100, 200, 255]);
    let dot_size = 3;

    // Top-left corner dots
    for i in 0..5 {
        let x = 20 + i * 15;
        let y = 20;
        draw_dot(img, x, y, dot_size, corner_color);
    }

    // Bottom-right corner dots
    for i in 0..5 {
        let x = width - 80 + i * 15;
        let y = height - 30;
        draw_dot(img, x, y, dot_size, corner_color);
    }

    // Add some vertical lines on the sides
    let line_color = Rgba([60, 60, 120, 255]);
    for y in (50..height - 50).step_by(8) {
        if y < height {
            img.put_pixel(10, y, line_color);
            img.put_pixel(width - 11, y, line_color);
        }
    }
}

fn draw_dot(img: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
    let (width, height) = img.dimensions();

    for dy in -(radius as i32)..=(radius as i32) {
        for dx in -(radius as i32)..=(radius as i32) {
            if (dx * dx + dy * dy) <= (radius as i32 * radius as i32) {
                let x = cx as i32 + dx;
                let y = cy as i32 + dy;

                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    img.put_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
}

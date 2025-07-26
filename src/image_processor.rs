//! Image processing module
//!
//! This module handles image manipulation operations including adding information bars
//! and adjusting aspect ratios.

use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::path::Path;

use crate::resource::Resources;

/// Adds an information bar to the bottom of an image
///
/// # Arguments
/// * `img` - The input image
/// * `input_path` - Path to the input image file
/// * `info_height` - Height of the information bar in pixels
/// * `resources` - Font and scaling resources
/// * `custom_logo_path` - Optional path to a custom logo file
///
/// # Returns
/// * `Result<DynamicImage, Box<dyn std::error::Error>>` - Image with information bar if successful
///
/// # Errors
/// Returns an error if the image cannot be processed or if EXIF data cannot be read
pub fn add_info_bar(
    img: DynamicImage,
    input_path: &Path,
    info_height: u32,
    resources: &Resources,
    custom_logo_path: Option<&Path>,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let (width, height) = img.dimensions();
    let padding = 32u32;
    let mut new_img: RgbaImage =
        ImageBuffer::from_pixel(width, height + info_height, Rgba([255, 255, 255, 255]));
    image::imageops::overlay(&mut new_img, &img.to_rgba8(), 0, 0);

    if let Ok(exif_info) = crate::exif::read_exif_info(input_path) {
        let camera_model = exif_info.camera_model.trim_matches('"');
        let lens_model = exif_info.lens_model.trim_matches('"');

        println!("[INFO] Processing image: {width}x{height}");
        println!("[INFO] Camera: {camera_model}, Lens: {lens_model}");

        let camera_text_height = resources.scale_bold.y.ceil() as u32;
        let lens_text_height = resources.scale_regular.y.ceil() as u32;
        let total_text_height = camera_text_height + lens_text_height + 8;
        let left_text_top = height + (info_height - total_text_height) / 2;
        let camera_y = left_text_top;
        let lens_y = camera_y + camera_text_height + 8;
        draw_text_mut(
            &mut new_img,
            Rgba([0, 0, 0, 255]),
            padding as i32,
            camera_y as i32,
            resources.scale_bold,
            &resources.font_bold,
            camera_model,
        );
        draw_text_mut(
            &mut new_img,
            Rgba([80, 80, 80, 255]),
            padding as i32,
            lens_y as i32,
            resources.scale_regular,
            &resources.font_regular,
            lens_model,
        );

        let params = format!(
            "{}mm | f{} | {}/s | ISO {}",
            exif_info
                .focal_length
                .trim_end_matches(" mm")
                .replace('"', ""),
            exif_info.aperture.replace('"', ""),
            exif_info.shutter_speed.replace('"', ""),
            exif_info.iso.replace('"', "")
        );
        println!("[INFO] Camera settings: {params}");

        let param_width = text_width(&resources.font_regular, resources.scale_regular, &params);
        let param_x = width as i32 - padding as i32 - param_width;
        let param_y =
            height as i32 + (info_height as i32 - resources.scale_regular.y.ceil() as i32) / 2;
        draw_text_mut(
            &mut new_img,
            Rgba([0, 0, 0, 255]),
            param_x,
            param_y,
            resources.scale_regular,
            &resources.font_regular,
            &params,
        );

        // Try to load and draw logo, but continue even if it fails
        if let Ok(Some(logo)) = crate::resource::load_camera_logo(camera_model, custom_logo_path) {
            let logo_target_height = (info_height as f32 * 0.65).round() as u32;
            let logo = logo.resize(
                logo.width() * logo_target_height / logo.height(),
                logo_target_height,
                image::imageops::FilterType::Lanczos3,
            );
            let logo_rgba = logo.to_rgba8();
            let logo_x = (width / 2).saturating_sub(logo_rgba.width() / 2);
            let logo_y = height + (info_height - logo_rgba.height()) / 2;
            for y in 0..logo_rgba.height() {
                for x in 0..logo_rgba.width() {
                    let pixel = logo_rgba.get_pixel(x, y);
                    let dst = new_img.get_pixel_mut(logo_x + x, logo_y + y);
                    let alpha = pixel[3] as f32 / 255.0;
                    for c in 0..3 {
                        dst[c] =
                            ((pixel[c] as f32 * alpha) + (dst[c] as f32 * (1.0 - alpha))) as u8;
                    }
                    dst[3] = 255;
                }
            }
            println!("[INFO] Logo added successfully");
        }
    } else {
        println!("[WARN] Failed to read EXIF information from image");
    }
    Ok(DynamicImage::ImageRgba8(new_img))
}

/// Pads an image to achieve 16:9 aspect ratio
///
/// # Arguments
/// * `orig_img` - Original image for background blur
/// * `img_with_bar` - Image with information bar
///
/// # Returns
/// * `DynamicImage` - Padded image with 16:9 aspect ratio
pub fn pad_to_16_9(orig_img: &DynamicImage, img_with_bar: &DynamicImage) -> DynamicImage {
    let (width, height) = img_with_bar.dimensions();
    let target_ratio = 16.0 / 9.0;
    let cur_ratio = width as f32 / height as f32;
    if (cur_ratio - target_ratio).abs() < 0.001 {
        return img_with_bar.clone();
    }
    let (new_width, new_height) = if cur_ratio > target_ratio {
        let new_height = ((width as f32) / target_ratio).ceil() as u32;
        (width, new_height)
    } else {
        let new_width = ((height as f32) * target_ratio).ceil() as u32;
        (new_width, height)
    };
    // First resize the original image to have a maximum dimension of 512 pixels (maintaining aspect ratio)
    let (orig_width, orig_height) = orig_img.dimensions();
    let scale = 512.0 / (orig_width.max(orig_height) as f32);
    let small_width = (orig_width as f32 * scale).round().max(1.0) as u32;
    let small_height = (orig_height as f32 * scale).round().max(1.0) as u32;
    let small = orig_img.resize_exact(small_width, small_height, FilterType::Triangle);
    // Apply blur to the resized image
    let blurred = small.blur(20.0);
    // Resize to target dimensions
    let bg = blurred.resize_exact(new_width, new_height, FilterType::Gaussian);
    let mut new_img = bg.to_rgba8();
    // Calculate position for the image with bar (centered horizontally, aligned to bottom)
    let x_offset = ((new_width as i32 - width as i32) / 2).max(0) as i64;
    let y_offset = (new_height as i32 - height as i32).max(0) as i64;
    image::imageops::overlay(&mut new_img, &img_with_bar.to_rgba8(), x_offset, y_offset);
    DynamicImage::ImageRgba8(new_img)
}

/// Calculates the width of text when rendered with a specific font and scale
///
/// # Arguments
/// * `font` - Font to use for rendering
/// * `scale` - Scale factor for the font
/// * `text` - Text to measure
///
/// # Returns
/// * `i32` - Width of the text in pixels
fn text_width(font: &Font, scale: Scale, text: &str) -> i32 {
    use rusttype::point;
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font
        .layout(text, scale, point(0.0, v_metrics.ascent))
        .collect();
    if let Some(last) = glyphs.last() {
        if let Some(bb) = last.pixel_bounding_box() {
            bb.max.x
        } else {
            0
        }
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusttype::Font;

    #[test]
    fn test_text_width() {
        // Create a test font
        let font_data = include_bytes!("../fonts/DejaVuSans.ttf");
        let font = Font::try_from_vec(font_data.to_vec()).unwrap();

        // Test with different scales
        let scale = Scale { x: 24.0, y: 24.0 };
        assert!(text_width(&font, scale, "Test") > 0);
        assert!(text_width(&font, scale, "Test") < text_width(&font, scale, "Test Test"));

        // Test with empty string
        assert_eq!(text_width(&font, scale, ""), 0);

        // Test with different scales
        let scale_large = Scale { x: 48.0, y: 48.0 };
        assert!(text_width(&font, scale_large, "Test") > text_width(&font, scale, "Test"));
    }

    #[test]
    fn test_aspect_ratio_calculation() {
        // Create test images
        let img_16_9 = DynamicImage::new_rgba8(1920, 1080);
        let img_4_3 = DynamicImage::new_rgba8(1600, 1200);

        // Test with already 16:9 image
        let result = pad_to_16_9(&img_16_9, &img_16_9);
        assert_eq!(result.dimensions(), (1920, 1080));

        // Test with 4:3 image
        let result = pad_to_16_9(&img_4_3, &img_4_3);
        let (width, height) = result.dimensions();
        let ratio = width as f32 / height as f32;
        assert!((ratio - 16.0 / 9.0).abs() < 0.01);
    }
}

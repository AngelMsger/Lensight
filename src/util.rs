//! Utility functions module
//!
//! This module contains helper functions for processing files and directories.

use std::path::Path;
use walkdir::WalkDir;

/// Processes all JPEG files in a directory
///
/// # Arguments
/// * `input` - Input directory path
/// * `output` - Output directory path
/// * `info_height` - Height of the information bar in pixels
/// * `force_16_9` - Whether to force 16:9 aspect ratio
/// * `custom_logo_path` - Optional path to a custom logo file
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful
///
/// # Errors
/// Returns an error if:
/// - The output directory cannot be created
/// - Any file cannot be processed
pub fn process_directory(
    input: &Path,
    output: &Path,
    info_height: u32,
    force_16_9: bool,
    custom_logo_path: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !output.exists() {
        std::fs::create_dir_all(output)?;
    }

    let entries: Vec<_> = WalkDir::new(input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            if let Some(ext) = e.path().extension() {
                ext == "jpg" || ext == "jpeg"
            } else {
                false
            }
        })
        .collect();
    let total = entries.len();
    for (idx, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let output_path = output.join(path.file_name().unwrap());
        println!("Processing {}/{}: {}", idx + 1, total, path.display());
        process_single_file(
            path,
            &output_path,
            info_height,
            force_16_9,
            custom_logo_path,
        )?;
    }

    Ok(())
}

/// Processes a single image file
///
/// # Arguments
/// * `input` - Input file path
/// * `output` - Output file path
/// * `info_height` - Height of the information bar in pixels
/// * `force_16_9` - Whether to force 16:9 aspect ratio
/// * `custom_logo_path` - Optional path to a custom logo file
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful
///
/// # Errors
/// Returns an error if:
/// - The input file cannot be opened
/// - The image cannot be processed
/// - The output file cannot be saved
pub fn process_single_file(
    input: &Path,
    output: &Path,
    info_height: u32,
    force_16_9: bool,
    custom_logo_path: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let orig_img = image::open(input)?;
    let resources = crate::resource::Resources::new()?;
    let watermarked = crate::image_processor::add_info_bar(
        orig_img.clone(),
        input,
        info_height,
        &resources,
        custom_logo_path,
    )?;
    let final_img = if force_16_9 {
        crate::image_processor::pad_to_16_9(&orig_img, &watermarked)
    } else {
        watermarked
    };
    final_img.save(output)?;
    Ok(())
}

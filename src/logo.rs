//! Camera logo module
//!
//! This module contains base64 encoded camera logos and functions to load them.

use base64::Engine as _;
use image::DynamicImage;
use std::error::Error;

/// Base64 encoded camera logos
pub struct CameraLogos;

impl CameraLogos {
    /// Loads a logo from base64 string
    ///
    /// # Arguments
    /// * `base64_str` - Base64 encoded image string
    ///
    /// # Returns
    /// * `Result<DynamicImage, Box<dyn Error>>` - Decoded image if successful
    pub fn load_from_base64(base64_str: &str) -> Result<DynamicImage, Box<dyn Error>> {
        let decoded = base64::engine::general_purpose::STANDARD.decode(base64_str)?;
        let img = image::load_from_memory(&decoded)?;
        Ok(img)
    }
}

/// Hardcoded base64 encoded camera logos
pub mod logos {
    pub const CANON: &str = include_str!("logos/canon.base64");
    pub const FUJIFILM: &str = include_str!("logos/fujifilm.base64");
    pub const NIKON: &str = include_str!("logos/nikon.base64");
    pub const PANASONIC: &str = include_str!("logos/panasonic.base64");
    pub const SONY: &str = include_str!("logos/sony.base64");
    pub const IPHONE: &str = include_str!("logos/iphone.base64");
    pub const DJI: &str = include_str!("logos/dji.base64");
}

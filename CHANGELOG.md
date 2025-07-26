# Changelog

This project follows [Semantic Versioning](https://semver.org/).

## [1.0.1] - 2025-07-26

### Fixed

- Fixed Sony camera model recognition for ILCE/ILCA/ILME prefixes (e.g., ILCE-7CM2 for Sony A7C2)
- Improved camera brand detection for Canon EOS, Nikon D/Z series, Fujifilm X/GFX series, and Panasonic DC/Lumix series

### Improved

- Code quality improvements: replaced `map_or` with `is_some_and` for better readability
- Code quality improvements: replaced `{}` placeholders with direct variable interpolation in `println!` and `format!` macros

## [0.1.0] - 2025-05-04

### Added

- Initial release
- Support for adding camera information and shooting details to photos
- Support for single image and batch processing
- Built-in support for major camera brands (Canon, Nikon, Sony, Fujifilm, Panasonic)
- Optional 16:9 aspect ratio output
- Custom logo support
- Customizable information bar height with adaptive font and logo sizing

### Technical Details

- Using Rust 2021 edition
- Key dependencies:
  - image 0.24.7
  - kamadak-exif 0.5.5
  - clap 4.4.11
  - walkdir 2.4.0
  - imageproc 0.23.0
  - rusttype 0.9.3
  - base64 0.21.5

## [Unreleased]

### Todo

- Add support for more camera brands
- Performance optimization
- Error handling improvements
- Additional customization options

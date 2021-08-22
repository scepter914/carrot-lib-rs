pub mod gray_image;
pub mod rgb_image;

use std::fs::File;
use std::io::Write;
use std::path::Path;
// use std::env;
// use std::io::prelude::*;

pub fn save_ppm_file(
    image: &image::RgbImage,
    path: &Path,
) -> std::result::Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(format!("P6\n{} {}\n255\n", image.width(), image.height()).as_bytes())?;
    file.write_all(&image.to_vec())?;
    Ok(())
}

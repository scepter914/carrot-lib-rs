use image::{Rgb, RgbImage};
use imageproc::point::Point;

/// Get image infomation
pub fn get_image_info(image: &RgbImage) -> String {
    format!("dimensions {:?}", image.dimensions())
}

/// Get pixel information
pub fn get_pixel_string_from_xy(image: &RgbImage, x: u32, y: u32) -> String {
    let pixel: &Rgb<u8> = image.get_pixel(x, y);
    format!("RGB : {}, {}, {}", pixel[0], pixel[1], pixel[2])
}

/// Get pixel information from imageproc::point::Point
pub fn get_pixel_string(image: &RgbImage, point: Point<f32>) -> String {
    get_pixel_string_from_xy(image, point.x as u32, point.y as u32)
}

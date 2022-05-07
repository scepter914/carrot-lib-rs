use image::{Rgb, RgbImage};
use imageproc::point::Point;

// RGB image
pub fn print_image_info(image: &RgbImage) -> String {
    format!("dimensions {:?}", image.dimensions())
}

pub fn print_pixel_from_point(image: &RgbImage, point: Point<f32>) -> String {
    print_pixel(image, point.x as u32, point.y as u32)
}

pub fn print_pixel(image: &RgbImage, x: u32, y: u32) -> String {
    let pixel: &Rgb<u8> = image.get_pixel(x, y);
    format!("RGB : {}, {}, {}", pixel[0], pixel[1], pixel[2])
}

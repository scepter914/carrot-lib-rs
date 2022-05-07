use image::{Rgb, RgbImage};
use imageproc::drawing::draw_filled_circle;
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

/// Get debug image drawing a point
pub fn draw_point(image: &RgbImage, point: &Point<f32>, rgb: image::Rgb<u8>) -> RgbImage {
    let point_size = image.width() / 80;
    draw_filled_circle(
        image,
        (point.x as i32, point.y as i32),
        point_size as i32,
        rgb,
    )
}

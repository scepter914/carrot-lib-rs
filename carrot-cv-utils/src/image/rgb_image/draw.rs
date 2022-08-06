use image::RgbImage;
use imageproc::drawing;
use imageproc::point::Point;
use imageproc::rect::Rect;

use crate::image::point;

/// Get debug image drawing a point
pub fn draw_point(image: &RgbImage, point: Point<f32>, rgb: image::Rgb<u8>) -> RgbImage {
    let point_size: i32 = image.width() as i32 / 80;
    drawing::draw_filled_circle(image, point::to_i32_tuple(point), point_size, rgb)
}

/// Get debug image drawing a point
pub fn draw_rect(
    image: &RgbImage,
    center_point: Point<f32>,
    width: u32,
    height: u32,
    rgb: image::Rgb<u8>,
) -> RgbImage {
    let x;
    let y;
    (x, y) = point::to_i32_tuple(center_point);
    let rect = Rect::at(x - width as i32 / 2, y - height as i32 / 2).of_size(width, height);
    drawing::draw_hollow_rect(image, rect, rgb)
}

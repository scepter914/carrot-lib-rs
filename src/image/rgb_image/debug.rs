use image::{Rgb, RgbImage};
use imageproc::point::Point;
use log::{debug, error, info, trace, warn};

// RGB image
pub fn print_image_info(image: &RgbImage) {
    info!("dimensions {:?}", image.dimensions());
    //info!("{:?}", self.image.color());
}

pub fn print_pixel_from_point(debug_message: &str, image: &RgbImage, point: Point<f32>) {
    print_pixel(debug_message, image, point.x as u32, point.y as u32);
}

pub fn print_pixel(debug_message: &str, image: &RgbImage, x: u32, y: u32) {
    let pixel: &Rgb<u8> = image.get_pixel(x, y);
    debug!(
        "{}, RGB : {}, {}, {}",
        debug_message, pixel[0], pixel[1], pixel[2]
    );
}

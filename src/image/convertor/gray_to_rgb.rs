use image::{GrayImage, Luma, Rgb, RgbImage};

/// convert from gray image to rgb image

pub fn convert(image: &GrayImage) -> RgbImage {
    let width = image.width();
    let height = image.height();
    let mut rgb_image = RgbImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            rgb_image.put_pixel(i, j, convert_gray_to_rgb_pixel(&pixel))
        }
    }
    rgb_image
}

fn convert_gray_to_rgb_pixel(pixel: &Luma<u8>) -> Rgb<u8> {
    Rgb([pixel[0], pixel[0], pixel[0]])
}

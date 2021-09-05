extern crate image;

/// convert from gray image to rgb image

pub fn convert(image: &image::GrayImage) -> image::RgbImage {
    let width = image.width();
    let height = image.height();
    let mut rgb_image = image::RgbImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            rgb_image.put_pixel(i, j, convert_gray_to_rgb_pixel(&pixel))
        }
    }
    rgb_image
}

fn convert_gray_to_rgb_pixel(pixel: &image::Luma<u8>) -> image::Rgb<u8> {
    image::Rgb([pixel[0], pixel[0], pixel[0]])
}

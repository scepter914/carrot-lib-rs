extern crate image;

pub fn convert(rgb_image: &image::RgbImage) -> image::GrayImage {
    let width = rgb_image.width();
    let height = rgb_image.height();
    let mut gray_image = image::GrayImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = rgb_image.get_pixel(i, j);
            let gray_pixel = [((pixel[0] as f32 * 0.2126) as u32
                + (pixel[1] as f32 * 0.7152) as u32
                + (pixel[2] as f32 * 0.0722) as u32) as u8; 1];
            gray_image.put_pixel(i, j, image::Luma(gray_pixel));
        }
    }
    gray_image
}

/// convert R layer of RGB image to gray image

pub fn convert_r_to_gray_image(rgb_image: &image::RgbImage) -> image::GrayImage {
    convert_channel_to_gray_image(rgb_image, 0)
}

/// convert G layer of RGB image to gray image

pub fn convert_g_to_gray_image(rgb_image: &image::RgbImage) -> image::GrayImage {
    convert_channel_to_gray_image(rgb_image, 1)
}

/// convert B layer of RGB image to gray image

pub fn convert_b_to_gray_image(rgb_image: &image::RgbImage) -> image::GrayImage {
    convert_channel_to_gray_image(rgb_image, 2)
}

fn convert_channel_to_gray_image(rgb_image: &image::RgbImage, channel: usize) -> image::GrayImage {
    let width = rgb_image.width();
    let height = rgb_image.height();
    let mut binary_image = image::GrayImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = rgb_image.get_pixel(i, j);
            binary_image.put_pixel(i, j, image::Luma([pixel[channel]; 1]));
        }
    }
    binary_image
}

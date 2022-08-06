use image::{GrayImage, Luma, Rgb, RgbImage};

/// convert from gray image to rgb image
pub fn convert_to_rgb(image: &GrayImage) -> RgbImage {
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

pub fn convert_to_r(image: &GrayImage) -> RgbImage {
    let width = image.width();
    let height = image.height();
    let mut rgb_image = RgbImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            rgb_image.put_pixel(i, j, convert_gray_to_r_pixel(&pixel))
        }
    }
    rgb_image
}

pub fn convert_to_g(image: &GrayImage) -> RgbImage {
    let width = image.width();
    let height = image.height();
    let mut rgb_image = RgbImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            rgb_image.put_pixel(i, j, convert_gray_to_g_pixel(&pixel))
        }
    }
    rgb_image
}

pub fn convert_to_b(image: &GrayImage) -> RgbImage {
    let width = image.width();
    let height = image.height();
    let mut rgb_image = RgbImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            rgb_image.put_pixel(i, j, convert_gray_to_b_pixel(&pixel))
        }
    }
    rgb_image
}

fn convert_gray_to_rgb_pixel(pixel: &Luma<u8>) -> Rgb<u8> {
    Rgb([pixel[0], pixel[0], pixel[0]])
}

fn convert_gray_to_r_pixel(pixel: &Luma<u8>) -> Rgb<u8> {
    Rgb([pixel[0], 0, 0])
}

fn convert_gray_to_g_pixel(pixel: &Luma<u8>) -> Rgb<u8> {
    Rgb([0, pixel[0], 0])
}

fn convert_gray_to_b_pixel(pixel: &Luma<u8>) -> Rgb<u8> {
    Rgb([0, 0, pixel[0]])
}

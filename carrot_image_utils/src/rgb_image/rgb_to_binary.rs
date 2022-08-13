use crate::gray_image::{gray_to_binary, gray_to_rgb};
use crate::rgb_image::rgb_to_gray;
use image::{GrayImage, Luma, Rgb, RgbImage};

/// RGB threshold to convert from RGB image to binary image by high and low threshold
///  high_threshold : [r, g, b]
///  low_threshold : [r, g, b]

pub struct RGBThreshold {
    pub high_threshold: Rgb<u8>,
    pub low_threshold: Rgb<u8>,
}

impl RGBThreshold {
    pub fn r_high_threshold(&self) -> u8 {
        self.high_threshold[0]
    }
    pub fn g_high_threshold(&self) -> u8 {
        self.high_threshold[1]
    }
    pub fn b_high_threshold(&self) -> u8 {
        self.high_threshold[2]
    }
    pub fn r_low_threshold(&self) -> u8 {
        self.low_threshold[0]
    }
    pub fn g_low_threshold(&self) -> u8 {
        self.low_threshold[1]
    }
    pub fn b_low_threshold(&self) -> u8 {
        self.low_threshold[2]
    }
}

/// - Convert a rgb image to binary image
/// - If below condition satisfy, then it return 255 (white) and the others return 0 (block).
///     - R threshold low < pixel.r < R threshold high
///     - G threshold low < pixel.g < G threshold high
///     - B threshold low < pixel.b < B threshold high

pub fn convert_by_threshold(image: &RgbImage, rgb_threshold: &RGBThreshold) -> GrayImage {
    let width = image.width();
    let height = image.height();
    let mut binarized_image = GrayImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            let value = convert_to_binary_pixel_by_threshold(pixel, rgb_threshold);
            binarized_image.put_pixel(i, j, Luma(value));
        }
    }
    binarized_image
}

/// - If below condition satisfy, then it return 255 (white) and the others return 0 (block).
///     - R threshold low < pixel.r < R threshold high
///     - G threshold low < pixel.g < G threshold high
///     - B threshold low < pixel.b < B threshold high

fn convert_to_binary_pixel_by_threshold(pixel: &Rgb<u8>, rgb_threshold: &RGBThreshold) -> [u8; 1] {
    let binary_pixel: [u8; 1] = if rgb_threshold.low_threshold[0] <= pixel[0]
        && pixel[0] <= rgb_threshold.high_threshold[0]
        && rgb_threshold.low_threshold[1] <= pixel[1]
        && pixel[1] <= rgb_threshold.high_threshold[1]
        && rgb_threshold.low_threshold[2] <= pixel[2]
        && pixel[2] <= rgb_threshold.high_threshold[2]
    {
        [255; 1]
    } else {
        [0; 1]
    };
    binary_pixel
}

/// - Debug image consist of 8 image
///     - input rgb image, binary image
///     - R gray image, binary image from R image
///     - G gray image, binary image from G image
///     - B gray image, binary image from B image
/// - Example image
///     - <https://github.com/scepter914/highspeed_cv/docs/result_rgb_layer.png>

pub fn get_rgb_threshold_debug_image(image: &RgbImage, rgb_threshold: &RGBThreshold) -> RgbImage {
    let width = image.width();
    let height = image.height();
    let mut combined_image = RgbImage::new(2 * width, 4 * height);

    let binarized_image_by_rgb_threshold = convert_by_threshold(image, rgb_threshold);
    let rgb_binarized_image_by_rgb_threshold =
        gray_to_rgb::convert_to_rgb(&binarized_image_by_rgb_threshold);

    // r
    let gray_r_image = rgb_to_gray::convert_r_to_gray_image(&image);
    let binary_r_image = gray_to_binary::convert_by_threshold(
        &gray_r_image,
        rgb_threshold.r_high_threshold(),
        rgb_threshold.r_low_threshold(),
    );
    let rgb_converted_gray_r_image = gray_to_rgb::convert_to_r(&gray_r_image);
    let rgb_converted_binary_r_image = gray_to_rgb::convert_to_rgb(&binary_r_image);

    // g
    let gray_g_image = rgb_to_gray::convert_g_to_gray_image(&image);
    let binary_g_image = gray_to_binary::convert_by_threshold(
        &gray_g_image,
        rgb_threshold.g_high_threshold(),
        rgb_threshold.g_low_threshold(),
    );
    let rgb_converted_gray_g_image = gray_to_rgb::convert_to_g(&gray_g_image);
    let rgb_converted_binary_g_image = gray_to_rgb::convert_to_rgb(&binary_g_image);

    // b
    let gray_b_image = rgb_to_gray::convert_b_to_gray_image(&image);
    let binary_b_image = gray_to_binary::convert_by_threshold(
        &gray_b_image,
        rgb_threshold.b_high_threshold(),
        rgb_threshold.b_low_threshold(),
    );
    let rgb_converted_gray_b_image = gray_to_rgb::convert_to_b(&gray_b_image);
    let rgb_converted_binary_b_image = gray_to_rgb::convert_to_rgb(&binary_b_image);

    for i in 0..width {
        for j in 0..height {
            // input image
            let pixel = image.get_pixel(i, j);
            combined_image.put_pixel(i, j, *pixel);

            // binalized image
            let pixel = rgb_binarized_image_by_rgb_threshold.get_pixel(i, j);
            combined_image.put_pixel(width + i, j, *pixel);

            // r, r thres
            let pixel = rgb_converted_gray_r_image.get_pixel(i, j);
            combined_image.put_pixel(i, height + j, *pixel);

            let pixel = rgb_converted_binary_r_image.get_pixel(i, j);
            combined_image.put_pixel(width + i, height + j, *pixel);

            // g, g thres
            let pixel = rgb_converted_gray_g_image.get_pixel(i, j);
            combined_image.put_pixel(i, 2 * height + j, *pixel);

            let pixel = rgb_converted_binary_g_image.get_pixel(i, j);
            combined_image.put_pixel(width + i, 2 * height + j, *pixel);

            // b, b thres
            let pixel = rgb_converted_gray_b_image.get_pixel(i, j);
            combined_image.put_pixel(i, 3 * height + j, *pixel);

            let pixel = rgb_converted_binary_b_image.get_pixel(i, j);
            combined_image.put_pixel(width + i, 3 * height + j, *pixel);
        }
    }
    combined_image
}

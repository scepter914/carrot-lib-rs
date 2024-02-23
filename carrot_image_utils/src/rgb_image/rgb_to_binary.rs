use crate::gray_image::gray_to_binary;
use crate::gray_image::gray_to_rgb::convert_to_rgb;
use crate::rgb_image::{rgb_to_gray, rgb_to_rgb};

use image::{GrayImage, Luma, Rgb, RgbImage};

use super::concat::concat_matrix_images;

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

    /// - Convert a rgb image to binary image
    /// - If below condition satisfy, then it return 255 (white) and the others return 0 (block).
    ///     - R threshold low < pixel.r < R threshold high
    ///     - G threshold low < pixel.g < G threshold high
    ///     - B threshold low < pixel.b < B threshold high
    pub fn convert_by_threshold(&self, image: &RgbImage) -> GrayImage {
        let width = image.width();
        let height = image.height();
        let mut binarized_image = GrayImage::new(width, height);
        for i in 0..width {
            for j in 0..height {
                let pixel = image.get_pixel(i, j);
                let value = self.convert_to_binary_pixel_by_threshold(pixel);
                binarized_image.put_pixel(i, j, Luma(value));
            }
        }
        binarized_image
    }

    /// - If below condition satisfy, then it return 255 (white) and the others return 0 (block).
    ///     - R threshold low < pixel.r < R threshold high
    ///     - G threshold low < pixel.g < G threshold high
    ///     - B threshold low < pixel.b < B threshold high
    fn convert_to_binary_pixel_by_threshold(&self, pixel: &Rgb<u8>) -> [u8; 1] {
        let binary_pixel: [u8; 1] = if self.low_threshold[0] <= pixel[0]
            && pixel[0] <= self.high_threshold[0]
            && self.low_threshold[1] <= pixel[1]
            && pixel[1] <= self.high_threshold[1]
            && self.low_threshold[2] <= pixel[2]
            && pixel[2] <= self.high_threshold[2]
        {
            [255; 1]
        } else {
            [0; 1]
        };
        binary_pixel
    }
    /// Get binarized images for each rgb value to debug
    pub fn get_binarized_images_for_each_rgb(&self, image: &RgbImage) -> Vec<RgbImage> {
        // r
        let gray_r_image = rgb_to_gray::convert_r_to_gray_image(&image);
        let binary_r_image = gray_to_binary::convert_by_threshold(
            &gray_r_image,
            self.r_high_threshold(),
            self.r_low_threshold(),
        );
        let rgb_converted_binary_r_image = convert_to_rgb(&binary_r_image);

        // g
        let gray_g_image = rgb_to_gray::convert_g_to_gray_image(&image);
        let binary_g_image = gray_to_binary::convert_by_threshold(
            &gray_g_image,
            self.g_high_threshold(),
            self.g_low_threshold(),
        );
        let rgb_converted_binary_g_image = convert_to_rgb(&binary_g_image);

        // b
        let gray_b_image = rgb_to_gray::convert_b_to_gray_image(&image);
        let binary_b_image = gray_to_binary::convert_by_threshold(
            &gray_b_image,
            self.b_high_threshold(),
            self.b_low_threshold(),
        );
        let rgb_converted_binary_b_image = convert_to_rgb(&binary_b_image);

        vec![
            rgb_converted_binary_r_image,
            rgb_converted_binary_g_image,
            rgb_converted_binary_b_image,
        ]
    }

    pub fn get_rgb_threshold_debug_image(
        &self,
        image: &RgbImage,
        width: u32,
        height: u32,
    ) -> RgbImage {
        // input and result images
        let output_image = convert_to_rgb(&self.convert_by_threshold(image));
        let dummy_image = RgbImage::new(width, height);
        let input_and_results = vec![image.clone(), output_image, dummy_image];

        let rgb_each_images = rgb_to_rgb::divide_channel(image);
        let binarized_images = self.get_binarized_images_for_each_rgb(image);

        let image_matrix = vec![input_and_results, rgb_each_images, binarized_images];
        concat_matrix_images(&image_matrix, width, height)
    }
}

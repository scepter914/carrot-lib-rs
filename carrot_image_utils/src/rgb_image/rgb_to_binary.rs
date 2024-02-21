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

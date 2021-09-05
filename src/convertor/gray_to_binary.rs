/// - Convert gray image to binary image by threshold
///     - If low_threshold <= pixel_value <= high_threshold then, output pixel is 255 (white color)
/// - Use Case
///     - If you want to a get binary image from pixel above threshold of a gray image, you should use convert_gray_to_binary_within_threshold(gray_image, threshold, 255)

pub fn convert_by_threshold(
    gray_image: &image::GrayImage,
    low_threshold: u8,
    high_threshold: u8,
) -> image::GrayImage {
    let width = gray_image.width();
    let height = gray_image.height();
    let mut binary_image = image::GrayImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = gray_image.get_pixel(i, j);
            let value = binarize_pixel_by_threshold(pixel, high_threshold, low_threshold);
            binary_image.put_pixel(i, j, value);
        }
    }
    binary_image
}

fn binarize_pixel_by_threshold(
    pixel: &image::Luma<u8>,
    low_threshold: u8,
    high_threshold: u8,
) -> image::Luma<u8> {
    let value: [u8; 1];
    if low_threshold <= pixel[0] && pixel[0] <= high_threshold {
        value = [255; 1];
    } else {
        value = [0; 1];
    }
    image::Luma(value)
}

/// - Convert gray image to binary image by otsu method

pub fn convert_by_otsu(gray_image: &image::GrayImage) -> image::GrayImage {
    let otsu_level = imageproc::contrast::otsu_level(&gray_image);
    imageproc::contrast::threshold(&gray_image, otsu_level)
}

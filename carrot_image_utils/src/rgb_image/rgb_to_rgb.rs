use image::{Rgb, RgbImage};

pub fn divide_channel(rgb_image: &RgbImage) -> Vec<RgbImage> {
    let width = rgb_image.width();
    let height = rgb_image.height();

    let mut r_image = RgbImage::new(width, height);
    let mut g_image = RgbImage::new(width, height);
    let mut b_image = RgbImage::new(width, height);

    for i in 0..width {
        for j in 0..height {
            let pixel = rgb_image.get_pixel(i, j);
            r_image.put_pixel(i, j, Rgb([pixel[0], 0, 0]));
            g_image.put_pixel(i, j, Rgb([0, pixel[1], 0]));
            b_image.put_pixel(i, j, Rgb([0, 0, pixel[2]]));
        }
    }
    vec![r_image, g_image, b_image]
}

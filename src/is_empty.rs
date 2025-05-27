use screenshots::Image;

pub fn is_image_empty(image: &Image) -> bool {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let buffer = image.buffer();
    
    if width == 0 || height == 0 {
        return true; // Empty image is considered empty
    }
    
    // Check the first pixel to determine if we're checking for all black or all white
    if buffer.len() < 4 {
        return true;
    }
    
    let first_pixel = &buffer[0..4];
    let is_black = first_pixel[0] == 0 && first_pixel[1] == 0 && first_pixel[2] == 0;
    let is_white = first_pixel[0] == 255 && first_pixel[1] == 255 && first_pixel[2] == 255;
    
    if !is_black && !is_white {
        return false;
    }
    
    // Check all other pixels match the first one
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;
            if idx + 2 >= buffer.len() {
                continue;
            }
            
            let r = buffer[idx];
            let g = buffer[idx + 1];
            let b = buffer[idx + 2];
            
            if is_black && (r != 0 || g != 0 || b != 0) {
                return false;
            }
            if is_white && (r != 255 || g != 255 || b != 255) {
                return false;
            }
        }
    }
    
    true // All pixels match the first one (all black or all white)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{RgbaImage, Rgba}; 

    // Helper to create a screenshots::Image from RgbaImage for testing is_image_empty
    fn rgba_to_screenshots_image(rgba_img: &RgbaImage) -> Image {
        Image::new(rgba_img.width(), rgba_img.height(), rgba_img.as_raw().to_vec())
    }

    #[test]
    fn test_empty_black_image() {
        let mut img = RgbaImage::new(10, 10);
        for pixel in img.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 255]);
        }
        assert!(is_image_empty(&rgba_to_screenshots_image(&img)));
    }

    #[test]
    fn test_empty_white_image() {
        let mut img = RgbaImage::new(10, 10);
        for pixel in img.pixels_mut() {
            *pixel = Rgba([255, 255, 255, 255]);
        }
        assert!(is_image_empty(&rgba_to_screenshots_image(&img)));
    }

    #[test]
    fn test_non_empty_image() {
        let mut img = RgbaImage::new(10, 10);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            if x == 5 && y == 5 {
                *pixel = Rgba([128, 128, 128, 255]); // A gray pixel
            } else {
                *pixel = Rgba([0, 0, 0, 255]);
            }
        }
        assert!(!is_image_empty(&rgba_to_screenshots_image(&img)));
    }

    #[test]
    fn test_zero_dimension_image() {
        let img_zero_width = RgbaImage::new(0, 10);
        assert!(is_image_empty(&rgba_to_screenshots_image(&img_zero_width)));
        let img_zero_height = RgbaImage::new(10, 0);
        assert!(is_image_empty(&rgba_to_screenshots_image(&img_zero_height)));
    }
}

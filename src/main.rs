extern crate num;
extern crate image;

use num::Complex;

fn main() {
    let max_iter: u32 = 255;

    let imgx = 500;
    let imgy = 500;

    let scalex = 2.0 / imgx as f32;
    let scaley = 2.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::GrayImage::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cy = y as f32 * scaley - 1.0;
        let cx = x as f32 * scalex - 1.5;

        let mut z = Complex::new(0.0, 0.0);
        let c = Complex::new(cx, cy);

        let mut i = 0;

        for t in 0..max_iter {
            if z.norm_sqr() > 4.0 {
                break
            }
            z = z * z + c;
            i = t;
        }
        // Create an 8bit pixel of type Luma and value i
        // and assign in to the pixel at position (x, y)
        *pixel = image::Luma([255 - i as u8]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}

extern crate num;
extern crate image;
extern crate piston_window;

use num::Complex;
use piston_window::*;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn escape_time(c: Complex<f64>, limit: u8) -> Option<u8> {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i)
        }
    }
    None
}

fn pixel_to_point(bounds: (u32, u32),
                  pixel: (u32, u32),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>)
    -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn initial_draw(bounds: (u32, u32), max_iter: u8) -> image::RgbaImage {
    let upper_left = Complex::new(-2.0, 1.4);
    let lower_right = Complex::new(0.8, -1.4);
    let mut imgbuf = image::RgbaImage::new(bounds.0, bounds.1);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let point = pixel_to_point(bounds, (x, y),
                                   upper_left, lower_right);
        let intensity = match escape_time(point, max_iter) {
            None => 0,
            Some(count) => (max_iter - count) as u8,
        };

        *pixel = image::Rgba([intensity; 4]);
    }
    imgbuf
}

fn _initial_draw(imgx: u32, imgy: u32, max_iter: u8) -> image::RgbaImage {
    let scalex = 1.0 / imgx as f32;
    let scaley = 1.0 / imgy as f32;

    let mut imgbuf = image::RgbaImage::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = x as f32 * scalex * 2.0 - 1.5;
        let cy = y as f32 * scaley * 2.0 - 1.0;

        let mut z = Complex::new(0.0, 0.0);
        let c = Complex::new(cx, cy);

        let mut i: u8 = 0;

        for t in 0..max_iter {
            if z.norm_sqr() > 4.0 {
                break
            }
            z = z * z + c;
            i = t;
        }

        let intensity = (max_iter - i) as u8;
        *pixel = image::Rgba([intensity; 4]);
    }
    
    imgbuf
}

fn main() {
    let max_iter: u8 = 255;

    let mut bounds = (500u32, 500u32);

    let mut window: PistonWindow = WindowSettings::new(
        "Mandelbrot", 
        [WIDTH, HEIGHT])
        .samples(4)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut imgbuf = initial_draw(bounds, max_iter);
    // let mut imgbuf = _initial_draw(bounds.0, bounds.1, max_iter);

    let imgtexture: G2dTexture = Texture::from_image(
            &mut window.factory,
            &imgbuf,
            &TextureSettings::new()
            ).unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g);
            image(&imgtexture, c.transform, g);
        });
    }

}

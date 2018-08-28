extern crate num;
extern crate image;
extern crate piston_window;

use num::Complex;
use piston_window::*;

fn initial_draw(imgx: u32, imgy: u32, max_iter: u32) -> image::RgbaImage {
    let scalex = 1.0 / imgx as f32;
    let scaley = 1.0 / imgy as f32;

    let mut imgbuf = image::RgbaImage::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = x as f32 * scalex * 2.0 - 1.5;
        let cy = y as f32 * scaley * 2.0 - 1.0;

        let mut z = Complex::new(0.0, 0.0);
        let c = Complex::new(cx, cy);

        let mut i: u32 = 0;

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
    let max_iter: u32 = 255;

    let imgx: u32 = 800;
    let imgy: u32 = 800;

    let mut window: PistonWindow = WindowSettings::new("Mandelbrot", [imgx, imgy])
        .samples(4)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut imgbuf = initial_draw(imgx, imgy, max_iter);

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

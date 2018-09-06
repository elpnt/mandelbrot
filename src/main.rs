extern crate num;
extern crate image;
extern crate piston_window;

use piston_window::*;

<<<<<<< HEAD
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
=======
fn main() {

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let mut window: PistonWindow = WindowSettings::new(
        "Draw Rectangle",
        [WIDTH, HEIGHT]
        ).vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut first_draw = false;
    let mut drawing = false;
    let mut released = false;

    let mut first_pos: Option<[f64; 2]> = None;
    let mut last_pos: Option<[f64; 2]> = None;
>>>>>>> 381098ce0d9afe7d54835b64aa8895505b724f79

    let mut rect_x0: f64 = 0.0;
    let mut rect_y0: f64 = 0.0;
    let mut rect_x1: f64 = 0.0;
    let mut rect_y1: f64 = 0.0;

<<<<<<< HEAD
        let mut i: u8 = 0;
=======
    let mut cursor_move_count: u32 = 0;
>>>>>>> 381098ce0d9afe7d54835b64aa8895505b724f79

    while let Some(e) = window.next() {
       
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                first_draw = true;
                drawing = true;
            }
<<<<<<< HEAD
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
=======
        };

        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                drawing = false;
                cursor_move_count = 0;
                released = true;
            }
        };
>>>>>>> 381098ce0d9afe7d54835b64aa8895505b724f79

        if let Some(cursor) = e.mouse_cursor_args() {
            if drawing {
                cursor_move_count += 1;
                if cursor_move_count == 1 {
                    rect_x0 = cursor[0] as f64;
                    rect_y0 = cursor[1] as f64;
                    // println!("{}: Pressed at ({}, {})", cursor_move_count, rect_x0, rect_y0);
                }
            } 
            
            rect_x1 = cursor[0] as f64;
            rect_y1 = cursor[1] as f64;
            // println!("{}: Released at ({}, {})", cursor_move_count, rect_x1, rect_y1);
        };

        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);

            if first_draw {
                rectangle([1.0, 0.0, 0.0, 0.2],
                          [rect_x0, rect_y0, rect_x1-rect_x0, rect_y1-rect_y0],
                          c.transform, g);
            }
        });
    }

}

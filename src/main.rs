extern crate num;
extern crate image;
extern crate piston_window;

use piston_window::*;

fn main() {

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let mut window: PistonWindow = WindowSettings::new("Draw Rectangle", [WIDTH, HEIGHT])
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut first_draw = false;
    let mut drawing = false;
    let mut released = false;

    let mut first_pos: Option<[f64; 2]> = None;
    let mut last_pos: Option<[f64; 2]> = None;

    let mut rect_x0: f64 = 0.0;
    let mut rect_y0: f64 = 0.0;
    let mut rect_x1: f64 = 0.0;
    let mut rect_y1: f64 = 0.0;

    let mut cursor_move_count: u32 = 0;

    while let Some(e) = window.next() {
        
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                first_draw = true;
                drawing = true;
            }
        };

        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                drawing = false;
                cursor_move_count = 0;
                released = true;
            }
        };

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

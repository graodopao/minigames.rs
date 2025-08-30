use std::cmp::min;
use std::ptr::null;
use raylib::prelude::*;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 540;

fn main() {
    // Palette from: https://colorhunt.co/palette/e62727f3f2ecdcdcdc1e93ab
    let col_highlight   = Color::from_hex("E62727").unwrap();
    let col_white       = Color::from_hex("F3F2EC").unwrap();
    let col_grey        = Color::from_hex("DCDCDC").unwrap();
    let col_blue        = Color::from_hex("1E93AB").unwrap();

    let (mut raylib, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("minigames.rs")
        .build();

    let mut ellapsed_time = 0.0f32;
    let default_font = raylib.get_font_default();
    while !raylib.window_should_close() {
        ellapsed_time += raylib.get_frame_time();
      
        let mut draw_handle = raylib.begin_drawing(&thread);
        draw_handle.clear_background(col_grey);


        let smooth_transition = f32::min(1.0, (ellapsed_time * 1.5).powf(5.0));
        let reverse_smooth_transition = 1.0 - smooth_transition;

        let bar_height = (HEIGHT as f32 * 0.5 * (1.0 - smooth_transition)) as i32;
        draw_handle.draw_triangle(
            Vector2::new(WIDTH as f32 * smooth_transition, 0.0),
            Vector2::new(WIDTH as f32, HEIGHT as f32 * reverse_smooth_transition * 2.0),
            Vector2::new(WIDTH as f32, 0.0),
            col_highlight);
        draw_handle.draw_triangle(
            Vector2::new(0.0, HEIGHT as f32 * smooth_transition),
            Vector2::new(0.0, HEIGHT as f32),
            Vector2::new(WIDTH as f32 * reverse_smooth_transition * 2.0, HEIGHT as f32),
            col_highlight);

        //let title_position = Vector2::new(WIDTH as f32 / 2.0, HEIGHT as f32 * 0.2);
        //draw_handle.draw_text_pro(&default_font, "MINIGAMES.RS", title_position, Vector2::new(0.0, 0.0), 0.0, 25.0 * smooth_transition, 5.0, Color::BLACK);
    }
}
use raylib::prelude::*;

fn main() {
    let (mut raylib, thread) = raylib::init()
        .size(640, 480)
        .title("graodopao's minigames")
        .build();

    while !raylib.window_should_close() {
        let mut draw_handle = raylib.begin_drawing(&thread);

        draw_handle.clear_background(Color::WHITE);
        draw_handle.draw_text("Test", 12, 12, 20, Color::BLACK);
    }
}
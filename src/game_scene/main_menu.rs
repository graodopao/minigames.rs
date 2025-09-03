use raylib::{RaylibHandle, RaylibThread};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::ffi::KeyboardKey::KEY_SPACE;
use raylib::math::Vector2;

use crate::game_scene::{GameScene, SceneData};
use crate::{GameColors, HEIGHT, WIDTH};

pub struct MainMenu {
    pub data: SceneData,
    pub elapsed_time: f32,
    start_initiated: bool,
    start_tick: f32,
}

impl MainMenu {
    pub const fn new() -> Self {
        Self{ data:SceneData::new(), elapsed_time: 0.0, start_initiated: false, start_tick: -0.25}
    }
}
impl GameScene for MainMenu {

    fn start(&mut self) {
        self.elapsed_time = 0.0;
        self.data.has_started = true;
    }

    fn update(&mut self, raylib: &mut raylib::RaylibHandle, thread: &RaylibThread) {
        self.elapsed_time += raylib.get_frame_time();

        if (!self.start_initiated) {
            self.start_initiated = raylib.is_key_pressed(KEY_SPACE);
            return;
        }

        self.start_tick += raylib.get_frame_time();
        if (self.start_tick < 1.0) { return };
        self.data.flag_for_finish();
    }
    
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {


        let font =  draw_handle.gui_get_font();
        draw_handle.draw_text_pro(
            &font,
            "Galactic\nWare.rs",
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            0.0,
            130.0,
            5.0,
            GameColors::CONTRAST);

        if ((self.elapsed_time * if (!self.start_initiated) { 5.0 } else { 40.0 }).sin() < 0.0) {
            let text = "- space to start -";
            let text_len = draw_handle.measure_text(text, 30) as f32;
            draw_handle.draw_text_pro(
                &font,
                text,
                Vector2::new(WIDTH as f32 / 2.0, HEIGHT as f32 * 0.6),
                Vector2::new(text_len / 2.0, 0.0),
                0.0,
                30.0,
                5.0,
                GameColors::CONTRAST);
        }

        let smooth_transition = (if (!self.start_initiated) {
            f32::min(self.elapsed_time * 1.2, 1.0) } else {
            f32::clamp(self.start_tick * 1.1, 0.0, 1.0)
        }).powf(5.0);

        let reverse_smooth_transition = 1.0 - smooth_transition;
        let bar_height = (HEIGHT as f32 * 0.5 * (1.0 - smooth_transition)) as i32;

        if (!self.start_initiated) {
            draw_handle.draw_triangle(
                Vector2::new(WIDTH as f32 * smooth_transition, 0.0),
                Vector2::new(WIDTH as f32, HEIGHT as f32 * reverse_smooth_transition * 2.0),
                Vector2::new(WIDTH as f32, 0.0),
                GameColors::DARK);
            draw_handle.draw_triangle(
                Vector2::new(0.0, HEIGHT as f32 * smooth_transition),
                Vector2::new(0.0, HEIGHT as f32),
                Vector2::new(WIDTH as f32 * reverse_smooth_transition * 2.0, HEIGHT as f32),
                GameColors::DARK);
        } else {
            draw_handle.draw_triangle(
                Vector2::new(WIDTH as f32 * smooth_transition, 0.0),
                Vector2::new(0.0, 0.0),
                Vector2::new(WIDTH as f32 * smooth_transition * 2.0, HEIGHT as f32 * smooth_transition * 2.0),
                GameColors::DARK);
            draw_handle.draw_triangle(
                Vector2::new(0.0, 0.0),
                Vector2::new(0.0, HEIGHT as f32 * smooth_transition ),
                Vector2::new(WIDTH as f32 * smooth_transition * 2.0, HEIGHT as f32 * smooth_transition * 2.0),
                GameColors::DARK);


        }
    }

    fn stop(&mut self) {
        self.data.flag_for_finish();
        self.data.has_started = false;
    }

    fn has_started(&self) -> bool {
        self.data.has_started
    }

    fn is_flagged_for_finish(&self) -> bool {
        self.data.flagged_for_finish
    }
}
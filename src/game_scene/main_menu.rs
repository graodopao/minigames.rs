use raylib::{RaylibHandle, RaylibThread};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;

use crate::game_scene::{GameScene, SceneData};
use crate::{GameColors, HEIGHT, WIDTH};

pub struct MainMenu {
    data: SceneData,
    pub(crate) elapsed_time: f32,
}

impl MainMenu {
    pub const fn new() -> Self {
        Self{ data:SceneData::new(), elapsed_time: 0.0}
    }
}
impl GameScene for MainMenu {

    fn start(&mut self) {
        self.elapsed_time = 0.0;
    }

    fn update(&mut self, raylib: &RaylibHandle, thread: &RaylibThread) {
        self.elapsed_time += raylib.get_frame_time();
    }
    
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        let smooth_transition = f32::min(1.0, (self.elapsed_time * 1.5).powf(5.0));
        let reverse_smooth_transition = 1.0 - smooth_transition;

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

        if ((self.elapsed_time * 5.0).sin() < 0.0) {
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

        let bar_height = (HEIGHT as f32 * 0.5 * (1.0 - smooth_transition)) as i32;
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
    }

    fn stop(&mut self) {
        todo!()
    }
}
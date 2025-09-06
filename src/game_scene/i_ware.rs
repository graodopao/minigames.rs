use std::ops::Range;
use std::thread::current;
use raylib::{RaylibHandle, RaylibThread};
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::*;
use crate::game_scene::{GameScene, SceneData};
use crate::{GameColors, HEIGHT, WIDTH};

pub struct IWare {
    data: SceneData,
    current_letter: char,
    letter_options: [char; 26],
    timeout_timer: f32,
}

impl IWare  {
    pub fn new() -> Self {
        Self {
            data: SceneData::new(),
            current_letter: 'i',
            letter_options: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
            timeout_timer: -0.5,
        }
    }

    fn random_letter(&mut self, raylib: &mut RaylibHandle) -> char {
        self.letter_options[raylib.get_random_value::<i32>(Range{start:0, end:25}) as usize]
    }
}

impl GameScene for IWare {
    fn start(&mut self) {
        self.data.has_started = true;
        self.timeout_timer = -0.5;
    }

    fn update(&mut self, raylib: &mut raylib::RaylibHandle, thread: &RaylibThread) {
        self.timeout_timer += raylib.get_frame_time();

        let char = raylib.get_char_pressed();
        if char.is_some() {
            println!("{}", char.unwrap().to_string().as_str());
            if char.unwrap() == self.current_letter {
                self.current_letter = self.random_letter(raylib);
                self.timeout_timer = -0.5;
            }
        }

        if (self.timeout_timer > 1.0) {
            self.stop();
        }
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.clear_background(GameColors::CLEAR.lerp(GameColors::DARK, self.timeout_timer.powf(3.0).min(1.0)));

        let font = draw_handle.gui_get_font();
        let width = draw_handle.measure_text(self.current_letter.to_string().to_uppercase().as_str(), 130) as f32;
        draw_handle.draw_text_ex(
            font,
            self.current_letter.to_string().to_uppercase().as_str(),
            Vector2{x:WIDTH as f32 / 2.0 - width / 2.0, y: HEIGHT as f32 / 2.0 - 80.0},
            130.0,
            20.0,
            GameColors::DARK);
    }

    fn stop(&mut self) {
        self.data.flag_for_finish();
        if (self.timeout_timer > 1.0) {
            self.data.lose();
        }
        self.data.has_started = false;
    }

    fn has_started(&self) -> bool {
        self.data.has_started
    }

    fn is_flagged_for_finish(&self) -> bool {
        self.data.flagged_for_finish
    }

    fn lost(&self) -> bool { self.data.lost }

    fn data(&self) -> &SceneData {
        &self.data
    }

    fn new_data(&mut self) {
        self.data = SceneData::new();
    }
}
pub mod i_ware;
pub mod main_menu;

use raylib::prelude::*;

struct SceneData {
    score:u64,
    difficulty:u8,
}

impl SceneData {
    const fn new() -> Self {
        Self { score: 0, difficulty: 0 }
    }

    fn score_up(&mut self, gain: u64) {
        self.score += gain;
    }

    fn set_difficulty(&mut self, to: u8) {
        self.difficulty = to;
    }
}

pub trait GameScene {
    //fn new() -> Self;
    fn start(&mut self);
    fn update(&mut self, raylib: &RaylibHandle, thread: &RaylibThread);
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
    fn stop(&mut self);
}
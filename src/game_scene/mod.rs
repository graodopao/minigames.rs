pub mod main_menu;
pub mod i_ware;
pub mod game_over;

use raylib::prelude::*;

pub struct SceneData {
    score:u64,
    difficulty:u8,
    has_started:bool,
    flagged_for_finish: bool,
    lost: bool,
}

impl SceneData {
    const fn new() -> Self {
        Self { score: 0, difficulty: 0, flagged_for_finish: false, lost: false, has_started: false }
    }

    fn score_up(&mut self, gain: u64) {
        self.score += gain;
    }

    fn set_difficulty(&mut self, to: u8) {
        self.difficulty = to;
    }

    fn flag_for_finish(&mut self) {
        self.flagged_for_finish = true;
    }
    
    fn lose(&mut self) {
        self.lost = true;
    }
    pub fn clear_flag(&mut self) {
        self.flagged_for_finish = false;
    }
}

pub trait GameScene {
    //fn new() -> Self;
    fn start(&mut self);
    fn update(&mut self, raylib: &mut RaylibHandle, thread: &RaylibThread);
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
    fn stop(&mut self);

    fn has_started(&self) -> bool;
    fn is_flagged_for_finish(&self) -> bool;
    fn lost(&self) -> bool;

    fn data(&self) -> &SceneData;

    fn new_data(&mut self);
}
mod game_scene;

use std::ptr::null;
use raylib::prelude::*;
use crate::game_scene::GameScene;
use crate::game_scene::main_menu::MainMenu;

static WIDTH: i32 = 720;
static HEIGHT: i32 = 540;

pub struct GameColors;
impl GameColors { // Palette from: https://colorhunt.co/palette/e62727f3f2ecdcdcdc1e93ab
    pub const DARK: Color       = Color { r:230, g:39,  b:39,  a:255 };
    pub const CLEAR: Color      = Color { r:243, g:242, b:236, a:255 };
    pub const CLEAR_TINT: Color = Color { r:220, g:220, b:220, a:255 };
    pub const CONTRAST: Color   = Color { r:30,  g:147, b:171, a:255 };
}

pub struct Scenes;
impl Scenes {
    pub const MAIN_MENU: MainMenu = MainMenu::new();
}

fn main() {
    let wow = MainMenu::new();
    let (mut raylib, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("minigames.rs")
        .build();

    let font = raylib.load_font_ex(&thread, "assets/Renogare-Regular.otf", 130, None).unwrap();
    raylib.gui_set_font(&font);

    let mut current_scene = Scenes::MAIN_MENU;
    current_scene.start();
    
    let default_font = raylib.get_font_default();
    while !raylib.window_should_close() {
        current_scene.update(&raylib, &thread);

        let mut draw_handle = raylib.begin_drawing(&thread);
        draw_handle.clear_background(GameColors::CLEAR_TINT);

        current_scene.draw(&mut draw_handle)
    }
}
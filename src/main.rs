mod game_scene;

use std::ptr::null;
use std::thread::current;
use raylib::prelude::*;
use crate::game_scene::GameScene;
use crate::game_scene::main_menu::MainMenu;
use crate::game_scene::i_ware::IWare;

static WIDTH: i32 = 720;
static HEIGHT: i32 = 540;

pub struct GameColors;
impl GameColors { // Palette from: https://colorhunt.co/palette/e62727f3f2ecdcdcdc1e93ab
    pub const DARK: Color       = Color { r:230, g:39,  b:39,  a:255 };
    pub const CLEAR: Color      = Color { r:243, g:242, b:236, a:255 };
    pub const CLEAR_TINT: Color = Color { r:220, g:220, b:220, a:255 };
    pub const CONTRAST: Color   = Color { r:30,  g:147, b:171, a:255 };
}

fn main() {
    let wow = MainMenu::new();
    let (mut raylib, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("minigames.rs")
        .build();

    let font = raylib.load_font_ex(&thread, "assets/Renogare-Regular.otf", 130, None).unwrap();
    raylib.gui_set_font(&font);

    // Creating the game list
    let mut minigame_example: MainMenu = MainMenu::new();
    let mut i_ware: IWare = IWare::new();
    let scene_list: [&mut dyn GameScene; 2] = [&mut minigame_example, &mut i_ware];

    let mut current_scene_index = 1;

    while !raylib.window_should_close() {

        if (scene_list[current_scene_index].is_flagged_for_finish()) {
            todo!("Place change minigames logic here");
            break;
        }

        scene_list[current_scene_index].update(&mut raylib, &thread);

        let mut draw_handle = raylib.begin_drawing(&thread);
        draw_handle.clear_background(GameColors::CLEAR_TINT);

        scene_list[current_scene_index].draw(&mut draw_handle)
    }
}
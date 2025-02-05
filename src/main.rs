use raylib::prelude::*;
mod scene_manager;
use scene_manager::{Scene, SceneManager};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

const BTN_WIDTH: f32 = 128.0;
const BTN_HEIGHT: f32 = 64.0;
const BTN_SPACING: f32 = 25.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    let font = rl
        .load_font(&thread, "assets/font_arcade_classic/ARCADECLASSIC.TTF")
        .expect("Failed to load font");

    let center_x = (SCREEN_WIDTH as f32 - BTN_WIDTH) / 2.0;
    let center_y = (SCREEN_HEIGHT as f32 - BTN_HEIGHT) / 2.0;

    let start_btn_rec = Rectangle {
        x: center_x,
        y: center_y - BTN_HEIGHT - BTN_SPACING,
        width: BTN_WIDTH,
        height: BTN_HEIGHT,
    };

    let exit_btn_rec = Rectangle {
        x: center_x,
        y: center_y,
        width: BTN_WIDTH,
        height: BTN_HEIGHT,
    };

    rl.set_target_fps(60);

    let mut should_close = false;
    //let mut screen = GameScreen::Logo;
    let logo_rec: Rectangle = Rectangle {
        x: (SCREEN_WIDTH as f32 - 200.0) / 2.0,  // Center horizontally
        y: (SCREEN_HEIGHT as f32 - 200.0) / 2.0, // Center vertically
        width: 200.0,
        height: 200.0,
    };

    let mut scene_manager = SceneManager::new();
    scene_manager.init();
    while !rl.window_should_close() && !should_close {
        let delta_time = rl.get_frame_time();
        // Update logic
        scene_manager.update(delta_time);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // Draw scene
        scene_manager.draw(&mut d);

        // Scene switch example
        if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            scene_manager.push(scene_manager::SceneID::MainMenu);
        }

        //match screen {
        //    GameScreen::Logo => {
        //        d.draw_rectangle_lines_ex(logo_rec, 1.25, Color::BLACK);
        //        d.draw_text_ex(
        //            &font,
        //            "Fancy Logo",
        //            Vector2 {
        //                x: logo_rec.x + 20.0,
        //                y: logo_rec.y + 20.0,
        //            },
        //            24.0,
        //            1.0,
        //            Color::BLACK,
        //        );
        //
        //        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        //            screen = GameScreen::Title;
        //        }
        //    }
        //    GameScreen::Title => {
        //        if draw_btn(&mut d, &font, &start_btn_rec, "Start") {
        //            screen = GameScreen::Gameplay;
        //        }
        //        if draw_btn(&mut d, &font, &exit_btn_rec, "Exit") {
        //            should_close = true;
        //        }
        //    }
        //    GameScreen::Gameplay => {
        //        d.draw_text_ex(
        //            &font,
        //            "Peak Gameplay!",
        //            Vector2 { x: 20.0, y: 20.0 },
        //            24.0,
        //            1.0,
        //            Color::BLACK,
        //        );
        //
        //        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        //            screen = GameScreen::Ending;
        //        }
        //    }
        //    GameScreen::Ending => {
        //        d.draw_text_ex(
        //            &font,
        //            "Ended!",
        //            Vector2 { x: 20.0, y: 20.0 },
        //            24.0,
        //            1.0,
        //            Color::BLACK,
        //        );
        //
        //        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        //            screen = GameScreen::Logo;
        //        }
        //    }
        //}
    }
}

/// Draws a button and returns `true` if it was clicked.
fn draw_btn(d: &mut RaylibDrawHandle, font: &Font, rect: &Rectangle, text: &str) -> bool {
    let mouse_pos = d.get_mouse_position();
    let is_hovered = rect.check_collision_point_rec(mouse_pos);

    let text_size = font.measure_text(text, 24.0, 1.0);
    let new_width = text_size.x + 25.0;
    let final_width = new_width.max(rect.width);

    // Adjust x to keep the button centered
    let adjusted_x = rect.x - (final_width - rect.width) / 2.0;

    let adjusted_rect = Rectangle {
        x: adjusted_x,
        y: rect.y,
        width: final_width,
        height: rect.height,
    };

    d.draw_rectangle_lines_ex(
        adjusted_rect,
        1.5,
        if is_hovered { Color::RED } else { Color::BLACK },
    );

    d.draw_text_ex(
        font,
        text,
        Vector2 {
            x: adjusted_rect.x + (adjusted_rect.width - text_size.x) / 2.0,
            y: adjusted_rect.y + (adjusted_rect.height - text_size.y) / 2.0,
        },
        24.0,
        1.0,
        Color::BLACK,
    );

    is_hovered && d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
}

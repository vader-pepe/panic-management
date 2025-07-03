use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

const BTN_WIDTH: f32 = 128.0;
const BTN_HEIGHT: f32 = 64.0;
const BTN_SPACING: f32 = 25.0;

const TILE_WIDTH: f32 = 32.0;
const TILE_HEIGHT: f32 = 32.0;

#[derive(Debug)]
enum GameScreen {
    Logo,
    MainMenu,
    Gameplay,
    PauseMenu,
    Ending,
}
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Panic Management")
        .build();

    let font = rl
        .load_font(&thread, "assets/font_arcadeclassic/ARCADECLASSIC.TTF")
        .expect("Failed to load font");

    let tiles_texture = rl
        .load_texture(&thread, "isometric_tileset/spritesheet.png")
        .expect("Failed to load image");

    let creature_1 = rl
        .load_texture(&thread, "character/Sprites/IDLE/idle_down.png")
        .expect("Failed to load image");

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

    let mut should_close = false;
    let logo_rec: Rectangle = Rectangle {
        x: (SCREEN_WIDTH as f32 - 200.0) / 2.0,  // Center horizontally
        y: (SCREEN_HEIGHT as f32 - 200.0) / 2.0, // Center vertically
        width: 200.0,
        height: 200.0,
    };
    let mut pause_btn = Button {
        rect: Rectangle {
            x: (SCREEN_WIDTH - (BTN_WIDTH as i32) - 20) as f32,
            y: 25.0,
            height: BTN_HEIGHT,
            width: BTN_WIDTH,
        },
        text: "Pause",
        visible: false,
    };
    let mut resume_btn = Button {
        rect: Rectangle { ..pause_btn.rect },
        text: "Resume",
        visible: false,
    };
    let mut end_btn = Button {
        rect: Rectangle {
            y: pause_btn.rect.y + 75.0,
            ..pause_btn.rect
        },
        text: "End",
        visible: false,
    };
    let mut exit_btn = Button {
        rect: exit_btn_rec,
        text: "Exit",
        visible: false,
    };
    let mut start_btn = Button {
        rect: start_btn_rec,
        text: "Start",
        visible: false,
    };

    let mut creature_1_rect = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 32.0,
        height: 32.0,
    };

    let mut screen = GameScreen::Logo;

    rl.set_target_fps(60);
    let mut dt: f32;

    let determinant = 0.5 * TILE_WIDTH * TILE_HEIGHT;
    let inv_det = 1.0 / determinant;

    while !rl.window_should_close() && !should_close {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        dt = d.get_frame_time();

        let mouse_pos = d.get_mouse_position();

        match screen {
            GameScreen::Logo => {
                d.draw_rectangle_lines_ex(logo_rec, 1.25, Color::BLACK);
                d.draw_text_ex(
                    &font,
                    "Fancy Logo",
                    Vector2 {
                        x: logo_rec.x + 20.0,
                        y: logo_rec.y + 20.0,
                    },
                    24.0,
                    1.0,
                    Color::BLACK,
                );

                if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    screen = GameScreen::MainMenu;
                }
            }
            GameScreen::MainMenu => {
                start_btn.draw(&mut d, &font);
                start_btn.handle_click(&mut d, || {
                    screen = GameScreen::Gameplay;
                });
                exit_btn.draw(&mut d, &font);
                exit_btn.handle_click(&mut d, || {
                    should_close = true;
                });
            }
            GameScreen::Gameplay => {
                // WARNING: only updates things here!
                // unless it's not gameplay related!

                if d.is_key_down(KeyboardKey::KEY_LEFT) && d.is_key_down(KeyboardKey::KEY_DOWN) {
                    println!("Diagonal left down");
                }

                if d.is_key_down(KeyboardKey::KEY_RIGHT) && d.is_key_down(KeyboardKey::KEY_DOWN) {
                    println!("Diagonal right down");
                }

                if d.is_key_down(KeyboardKey::KEY_RIGHT) && d.is_key_down(KeyboardKey::KEY_UP) {
                    println!("Diagonal right up");
                }

                if d.is_key_down(KeyboardKey::KEY_LEFT) && d.is_key_down(KeyboardKey::KEY_UP) {
                    println!("Diagonal left up");
                }

                if d.is_key_down(KeyboardKey::KEY_UP) {
                    creature_1_rect.y -= 32.0 * dt;
                }

                if d.is_key_down(KeyboardKey::KEY_DOWN) {
                    creature_1_rect.y += 32.0 * dt;
                }

                if d.is_key_down(KeyboardKey::KEY_LEFT) {
                    creature_1_rect.x -= 32.0 * dt;
                }

                if d.is_key_down(KeyboardKey::KEY_RIGHT) {
                    creature_1_rect.x += 32.0 * dt;
                }

                // loop over X axis
                for x in 0..35 {
                    // loop over Y axis
                    for y in 0..35 {
                        // WARNING: START TRASH
                        let gx = (0.25 * TILE_HEIGHT * (mouse_pos.x - x as f32)
                            + 0.5 * TILE_WIDTH * (mouse_pos.y - y as f32))
                            * inv_det;
                        let gy = (-0.25 * TILE_HEIGHT * (mouse_pos.x - x as f32)
                            + 0.5 * TILE_WIDTH * (mouse_pos.y - y as f32))
                            * inv_det;
                        if x == gx.floor() as i32 && y == gy.floor() as i32 {
                            d.draw_texture_pro(
                                &tiles_texture,
                                Rectangle {
                                    x: 0.0,
                                    y: 2.0,
                                    width: TILE_WIDTH,
                                    height: TILE_HEIGHT,
                                },
                                Rectangle {
                                    // use here
                                    x: ((x as f32) * 0.5 * TILE_WIDTH
                                        + (y as f32) * -0.5 * TILE_WIDTH),
                                    y: ((x as f32) * 0.25 * TILE_HEIGHT
                                        + (y as f32) * 0.25 * TILE_HEIGHT),
                                    width: TILE_WIDTH,
                                    height: TILE_HEIGHT,
                                },
                                Vector2 { x: 0.0, y: 0.0 },
                                0.0,
                                Color::WHITE,
                            );
                            continue;
                        }
                        // WARNING: END TRASH
                        d.draw_texture_pro(
                            &tiles_texture,
                            Rectangle {
                                x: 0.0,
                                y: 0.0,
                                width: TILE_WIDTH,
                                height: TILE_HEIGHT,
                            },
                            Rectangle {
                                // use here
                                x: ((x as f32) * 0.5 * TILE_WIDTH + (y as f32) * -0.5 * TILE_WIDTH),
                                y: ((x as f32) * 0.25 * TILE_HEIGHT
                                    + (y as f32) * 0.25 * TILE_HEIGHT),
                                width: TILE_WIDTH,
                                height: TILE_HEIGHT,
                            },
                            Vector2 { x: 0.0, y: 0.0 },
                            0.0,
                            Color::WHITE,
                        );
                    }
                }

                // draw creature
                d.draw_texture_pro(
                    &creature_1,
                    Rectangle {
                        x: 36.0,
                        y: 24.0,
                        width: 32.0,
                        height: 32.0,
                    },
                    Rectangle { ..creature_1_rect },
                    Vector2 { x: 0.0, y: 0.0 },
                    0.0,
                    Color::WHITE,
                );

                pause_btn.draw(&mut d, &font);
                pause_btn.handle_click(&mut d, || {
                    println!("Paused!");
                    screen = GameScreen::PauseMenu;
                });
                end_btn.draw(&mut d, &font);
                end_btn.handle_click(&mut d, || {
                    screen = GameScreen::Ending;
                });
            }
            GameScreen::Ending => {
                d.draw_text_ex(
                    &font,
                    "Ended!",
                    Vector2 { x: 20.0, y: 20.0 },
                    24.0,
                    1.0,
                    Color::BLACK,
                );
                if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    screen = GameScreen::Logo;
                }
            }
            GameScreen::PauseMenu => {
                // Darken the gameplay background
                d.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0, 0, 0, 180));

                resume_btn.draw(&mut d, &font);
                resume_btn.handle_click(&mut d, || {
                    screen = GameScreen::Gameplay;
                });
                // probably should move? idk
                exit_btn.rect = Rectangle {
                    x: end_btn.rect.x,
                    y: end_btn.rect.y + 75.0,
                    ..exit_btn.rect
                };
                exit_btn.draw(&mut d, &font);
                exit_btn.handle_click(&mut d, || {
                    should_close = true;
                });
                end_btn.draw(&mut d, &font);
                end_btn.handle_click(&mut d, || {
                    screen = GameScreen::Ending;
                });
            }
        }
    }
}

struct Button<'a> {
    rect: Rectangle,
    text: &'a str,
    visible: bool,
}

impl<'a> Button<'a> {
    /// Draws the button.
    fn draw(&mut self, d: &mut RaylibDrawHandle, font: &Font) {
        self.visible = true;
        if self.visible {
            let mouse_pos = d.get_mouse_position();
            let is_hovered = self.rect.check_collision_point_rec(mouse_pos);
            let text_size = font.measure_text(self.text, 24.0, 1.0);
            let new_width = text_size.x + 25.0;
            let final_width = new_width.max(self.rect.width);
            // Adjust x to keep the button centered.
            let adjusted_x = self.rect.x - (final_width - self.rect.width) / 2.0;
            let adjusted_rect = Rectangle {
                x: adjusted_x,
                y: self.rect.y,
                width: final_width,
                height: self.rect.height,
            };

            d.draw_rectangle_lines_ex(
                adjusted_rect,
                1.5,
                if is_hovered { Color::RED } else { Color::BLACK },
            );
            d.draw_text_ex(
                font,
                self.text,
                Vector2 {
                    x: adjusted_rect.x + (adjusted_rect.width - text_size.x) / 2.0,
                    y: adjusted_rect.y + (adjusted_rect.height - text_size.y) / 2.0,
                },
                24.0,
                1.0,
                Color::BLACK,
            );
        }
    }
    /// Executes the provided callback if the button is clicked.
    fn handle_click<F>(&self, d: &mut RaylibDrawHandle, callback: F)
    where
        F: FnOnce(),
    {
        let mouse_pos = d.get_mouse_position();
        if self.rect.check_collision_point_rec(mouse_pos)
            && d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            callback();
        }
    }
}

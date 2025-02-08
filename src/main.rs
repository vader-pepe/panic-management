use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

const BTN_WIDTH: f32 = 128.0;
const BTN_HEIGHT: f32 = 64.0;
const BTN_SPACING: f32 = 25.0;

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

    let mut screen = GameScreen::Logo;
    let mut velocity: f32 = 1.0;
    let mut peasant_steps = SCREEN_WIDTH as f32;
    let mut last_valid_peasant_steps = SCREEN_WIDTH as f32;

    rl.set_target_fps(60);
    while !rl.window_should_close() && !should_close {
        // WARNING: if you want an immutable reference to RaylibHandle,
        // please please do it before this warning line
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

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
                last_valid_peasant_steps -= velocity * 1.0;
                peasant_steps -= velocity * 1.0;
                d.draw_rectangle_lines_ex(
                    Rectangle {
                        x: 20.0,
                        y: 20.0,
                        height: 150.0,
                        width: 25.0,
                    },
                    1.25,
                    Color::BLACK,
                );
                d.draw_rectangle_lines_ex(
                    Rectangle {
                        x: 20.0,
                        y: (SCREEN_HEIGHT - 150 - 20) as f32,
                        height: 150.0,
                        width: 25.0,
                    },
                    1.25,
                    Color::BLACK,
                );
                d.draw_circle_lines(peasant_steps as i32, 150, 25.0, Color::BLACK);
                let rec1 = Rectangle {
                    x: 20.0,
                    y: 20.0,
                    height: 150.0,
                    width: 25.0,
                };
                if rec1.check_collision_circle_rec(
                    Vector2 {
                        x: peasant_steps,
                        y: 150.0,
                    },
                    25.0,
                ) {
                    peasant_steps = last_valid_peasant_steps;
                    velocity = 0.0;
                } else {
                    println!("not collided");
                }
                let text_size = font.measure_text("Peak Gameplay!", 24.0, 1.0);
                let pos = Vector2 {
                    x: 25.0,
                    y: (SCREEN_HEIGHT / 2) as f32,
                };
                let origin = Vector2 {
                    x: text_size.x / 2.0,
                    y: text_size.y / 2.0,
                };
                let rotation = 90.0;
                d.draw_text_pro(
                    &font,
                    "Peak Gameplay!",
                    pos,
                    origin,
                    rotation,
                    24.0,
                    1.0,
                    Color::BLACK,
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

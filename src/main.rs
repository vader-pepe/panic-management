use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;
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
    let mut should_change_scene_to_main = true;

    while !rl.window_should_close() && !should_close {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // start btn
        draw_btn(
            &mut d,
            &font,
            &start_btn_rec,
            "Start",
            start,
            &mut should_change_scene_to_main,
        );

        // exit btn
        draw_btn(
            &mut d,
            &font,
            &exit_btn_rec,
            "Exit",
            exit,
            &mut should_close,
        );
    }
}

fn start(c: &mut bool) {
    println!("Start clicked");
}

fn exit(c: &mut bool) {
    println!("Exit clicked");
    *c = true;
}

fn draw_btn<T, F: Fn(T)>(
    d: &mut RaylibDrawHandle,
    font: &Font,
    rect: &Rectangle,
    text: &str,
    callback: F,
    var: T,
) {
    let mouse_pos = d.get_mouse_position();
    let is_hovered = rect.check_collision_point_rec(mouse_pos);

    d.draw_rectangle_lines_ex(
        rect,
        1.5,
        if is_hovered { Color::RED } else { Color::BLACK },
    );

    let text_size = font.measure_text(text, 24.0, 1.0);
    d.draw_text_ex(
        font,
        text,
        Vector2 {
            x: rect.x + (rect.width - text_size.x) / 2.0,
            y: rect.y + (rect.height - text_size.y) / 2.0,
        },
        24.0,
        1.0,
        Color::BLACK,
    );

    if is_hovered && d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        callback(var);
    }
}

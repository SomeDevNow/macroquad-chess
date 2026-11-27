
mod game_handler;
mod utils;
use game_handler::GameHandler;
use macroquad::prelude::*;

const WINDOW_TITLE: &str = "Blank Window Template";
const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 360;
const BACKGROUND_COLOR: Color = Color::from_hex(0x5479b0);

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let render_target = render_target(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest); // Use Nearest for pixel art and Linear for other non-pixel art projects
    let mut render_target_cam = Camera2D::from_display_rect(Rect::new(
        0.0,
        0.0,
        WINDOW_WIDTH as f32,
        WINDOW_HEIGHT as f32,
    ));
    render_target_cam.render_target = Some(render_target.clone());
    let mut fullscreen: bool = false;

    // create custom variables

    let mut game_handler = GameHandler::new();

    loop {
        {  // do update logic here
            game_handler.update();

            if is_key_pressed(KeyCode::F) { fullscreen = if fullscreen {false} else {true}; set_fullscreen(fullscreen); }

            if is_key_pressed(KeyCode::Escape) { break; }            
        }

        let w = WINDOW_WIDTH as f32;
        let h = WINDOW_HEIGHT as f32;
        let scale = f32::min(screen_width() / w, screen_height() / h);

        set_camera(&render_target_cam);
        clear_background(BACKGROUND_COLOR);

        {  // start drawing here
            game_handler.draw();
        }

        set_default_camera();
        clear_background(BLACK);

        draw_texture_ex(
            &render_target.texture,
            (screen_width() - (w * scale)) * 0.5,
            (screen_height() - (h * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w * scale, h * scale)),
                flip_y: true,
                ..Default::default()
            },
        );

        next_frame().await;
    }
}

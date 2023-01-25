use macroquad::prelude::*;
use soclose::{colors, game::PlayField};

fn window_conf() -> Conf {
    Conf {
        window_title: "SoClose".to_owned(),
        fullscreen: false,
        window_width: 256,
        window_height: 320,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = PlayField::init();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        game.update();

        clear_background(colors::BACKGROUD);
        game.draw();
        next_frame().await
    }
}

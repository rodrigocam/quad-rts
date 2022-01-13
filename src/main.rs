use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;
use macroquad_tiled::load_map;

mod clock;
mod game;

use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "shotcaller".to_owned(),
        fullscreen: false,
        window_width: 400,
        window_height: 160,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map_data: String = std::fs::read_to_string("assets/tiled_map.json")?;
    let map_texture = load_texture("assets/level.png").await?;
    map_texture.set_filter(FilterMode::Nearest);
    let textures = &[("level.png", map_texture)];
    let map = load_map(&map_data, textures, &[])?;

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 400.0, 160.0));
    let mut game = Game::new();

    game.start();
    loop {
        let delta = get_frame_time();
        game.update(delta)?;

        clear_background(BLACK);

        set_camera(&camera);

        map.draw_tiles("ground", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("road", Rect::new(0.0, 0.0, 400.0, 160.0), None);

        map.draw_tiles("trees1", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees1_2", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees1_3", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees1_4", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees1_5", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees1_6", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees1_7", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees1_8", Rect::new(0.0, 0.0, 400.0, 160.0), None);

        map.draw_tiles("trees2", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees2_2", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees2_3", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees2_4", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees2_5", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees2_6", Rect::new(0.0, 0.0, 400.0, 160.0), None);
        map.draw_tiles("trees2_7", Rect::new(0.0, 0.0, 400.0, 160.0), None);

        map.draw_tiles("tower", Rect::new(0.0, 0.0, 400.0, 160.0), None);

        next_frame().await
    }
}

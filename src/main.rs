mod game;
mod score;
mod snake;
mod utils;

use game::Game;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake Game", game::WINDOW_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();
    let assets = utils::find_assets_folder().expect("Failed to find 'assets' directory");
    let font = assets.join("FiraSans-Regular.ttf");
    let mut glyphs = Glyphs::new(
        &font,
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .expect("Failed to load font");

    let mut last_update_time = std::time::Instant::now();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_press(key);
        }

        window.draw_2d(&event, |c, g, device| {
            game.draw(&c, g, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });

        if last_update_time.elapsed().as_secs_f64() > game.get_speed() {
            game.update();
            last_update_time = std::time::Instant::now();
        }
    }
}

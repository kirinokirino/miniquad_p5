#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::missing_const_for_fn,
    clippy::cast_possible_wrap,
    clippy::missing_panics_doc
)]
use simple_pixels::{rgb::RGBA8, start, Config, Context, KeyCode, State};

mod clock;
mod common;
mod sprite;

use clock::Clock;
use common::{Size, Vec2};
use sprite::Sprite;

fn main() {
    let config = Config {
        window_title: "game".to_string(),
        window_width: 200,
        window_height: 200,
        fullscreen: false,
        icon: None,
    };

    let game = Game::new();
    start(config, game);
}

struct Game {
    clock: Clock,
    sprite: Sprite,
}

impl Game {
    pub fn new() -> Self {
        let mut pixels = Vec::with_capacity(20 * 20);
        for col in 0..20 {
            let red = if col % 5 == 0 { 255 } else { 0 };
            for row in 0..20 {
                let green = if row % 5 == 0 { 255 } else { 0 };
                pixels.push(RGBA8::new(red, green, 5 * row + col, 255));
            }
        }
        let sprite = Sprite::new(Vec2::new(10.0, 10.0), Size::new(20, 20), pixels);
        let clock = Clock::new();
        Self { clock, sprite }
    }
}

impl State for Game {
    fn update(&mut self, ctx: &mut Context) {
        if ctx.is_key_down(KeyCode::Escape) {
            ctx.quit();
        }

        let mouse = ctx.get_mouse_pos();
        self.sprite.origin = Vec2::new(mouse.0, mouse.1);

        self.clock.sleep();
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear();
        self.sprite.draw(ctx);
    }
}

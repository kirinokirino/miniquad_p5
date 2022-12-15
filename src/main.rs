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

mod cli;
mod clock;
mod common;
mod ppt;
mod sprite;

use cli::Arguments;
use clock::Clock;
use common::{circle, constrain, line, Size, Vec2};
use ppt::load_sprite;
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
    mouse_pos: Vec2,
    width: u32,
    height: u32,
}

impl Game {
    pub fn new() -> Self {
        let clock = Clock::new();
        Self {
            clock,
            mouse_pos: Vec2::new(0.0, 0.0),
            width: 200,
            height: 200,
        }
    }
}

impl State for Game {
    fn update(&mut self, ctx: &mut Context) {
        if ctx.is_key_down(KeyCode::Escape) {
            ctx.quit();
        }

        let mouse = ctx.get_mouse_pos();
        self.mouse_pos = Vec2::new(
            constrain(mouse.0, 0.0, self.width as f32),
            constrain(mouse.1, 0.0, self.height as f32),
        );

        self.clock.sleep();
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear();
        let circle = circle(self.mouse_pos, 10.0);
        let line = circle.iter().filter(|point| {
            point.x > 0.0
                && point.x < self.width as f32
                && point.y > 0.0
                && point.y < self.height as f32
        });
        for point in line {
            ctx.draw_pixel(
                point.x as i32,
                point.y as i32,
                RGBA8::new(150, 100, 100, 255),
            );
        }
    }
}

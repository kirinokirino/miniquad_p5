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
    cursor: Vec<Vec2>,
}

impl Game {
    pub fn new() -> Self {
        let clock = Clock::new();
        let (width, height) = (200, 200);
        let mouse_pos = Vec2::new(0.0, 0.0);
        let mut circles: Vec<Vec2> = Vec::new();
        for radius in 1..5 {
            let circle = circle(mouse_pos, radius as f32 * 5.0);
            circles.extend(circle.iter())
        }
        Self {
            clock,
            mouse_pos,
            width,
            height,
            cursor: circles,
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
        for point in self.cursor.iter().filter_map(|point| {
            let point = *point + self.mouse_pos;
            if point.x > 0.0
                && point.x < self.width as f32
                && point.y > 0.0
                && point.y < self.height as f32
            {
                return Some(point);
            }
            None
        }) {
            let (x, y) = (point.x as i32, point.y as i32);
            ctx.draw_pixel(
                x ,
                y ,
                dither(x, y, RGBA8::new(150, 100, 100, 255), RGBA8::new(80, 70, 90, 255), 0.5)
            );
        }
    }
}

const DISPERSION_MATRIX_SIZE: u8 = 9;
const DISPERSED: [u8; DISPERSION_MATRIX_SIZE as usize] = [1, 7, 4, 5, 8, 3, 6, 2, 9];

pub fn dither(x: i32, y: i32, main_color: RGBA8, alternative_color: RGBA8, mix: f32) -> RGBA8 {
    let idx_in_dispersion_matrix = ((x - y * 3).abs() % DISPERSION_MATRIX_SIZE as i32) as usize;
    let color_threshold = DISPERSED[idx_in_dispersion_matrix] as f32 / DISPERSION_MATRIX_SIZE as f32;

    if mix < color_threshold {
        main_color
    } else {
        alternative_color
    }
}
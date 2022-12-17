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

use clock::Clock;
use common::{
    circle, constrain, empty_triangle, point_is_in_triangle, triangle, triangle2, Size, Vec2,
};

use sprite::Sprite;

fn main() {
    let (width, height) = (400, 400);
    let config = Config {
        window_title: "game".to_string(),
        window_width: width,
        window_height: height,
        fullscreen: false,
        icon: None,
    };

    let game = Game::new(width, height);
    start(config, game);
}

struct Game {
    clock: Clock,
    mouse_pos: Vec2,
    width: u32,
    height: u32,
    sprites: Vec<Sprite>,
    stuff: Vec<Vec2>,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let clock = Clock::new();
        let mouse_pos = Vec2::new(0.0, 0.0);
        let mut sprites: Vec<Sprite> = Vec::new();
        sprites.push(Sprite::from_vec2(
            circle(Vec2::new(0.0, 0.0), 15.0), //mouse_pos + Vec2::new(25.0, 25.0)),
            RGBA8::new(20, 200, 100, 255),
        ));

        let mut stuff = triangle(
            Vec2::from_angle(std::f32::consts::TAU / 3.0) * 100.0,
            Vec2::from_angle((std::f32::consts::TAU / 3.0) * 2.0) * 100.0,
            Vec2::from_angle((std::f32::consts::TAU / 3.0) * 3.0) * 100.0,
        );
        Self {
            clock,
            mouse_pos,
            width,
            height,
            sprites,
            stuff,
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

        self.sprites[0].origin = self.mouse_pos;

        let (p0, p1, p2) = (
            Vec2::new(200.0, 10.0),
            Vec2::new(20.0, 200.0),
            Vec2::new(350.0, 250.0),
        );

        self.stuff = triangle2(p0, p1, p2);
        self.stuff
            .extend(bleh(Vec2::new(250.0, 200.0), self.clock.now() * 3.0, 4.0));
        self.stuff
            .extend(bleh(Vec2::new(200.0, 150.0), self.clock.now() * 0.2, 9.0));

        self.clock.sleep();
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear();
        for sprite in &self.sprites {
            sprite.draw(ctx);
        }
        for point in self
            .stuff
            .iter()
            .filter(|p| p.inside(self.width as i32, self.height as i32))
        {
            ctx.draw_pixel(
                point.x as i32,
                point.y as i32,
                dither(
                    point.x as i32,
                    point.y as i32,
                    RGBA8::new(0, 100, 255, 50),
                    RGBA8::new(0x6a, 0xc0, 0xbd, 50),
                    (self.clock.now() / 10.0).sin(),
                ),
            );
        }
    }
}

pub fn bleh(p: Vec2, t: f32, b: f32) -> Vec<Vec2> {
    triangle(
        p + Vec2::from_angle((t / 3.0) % b / 3.0) * 100.0,
        p + Vec2::from_angle(((t / 2.0 + 1.5) % b / 3.0) * 2.0) * 100.0,
        p + Vec2::from_angle(((t / 2.5 + 1.0) % b / 3.0) * 3.0) * 100.0,
    )
}
const DISPERSION_MATRIX_SIZE: u8 = 9;
const DISPERSED: [u8; DISPERSION_MATRIX_SIZE as usize] = [1, 7, 4, 5, 8, 3, 6, 2, 9];

#[must_use]
pub fn dither(x: i32, y: i32, main_color: RGBA8, alternative_color: RGBA8, mix: f32) -> RGBA8 {
    let idx_in_dispersion_matrix = ((x - y * 3).abs() % i32::from(DISPERSION_MATRIX_SIZE)) as usize;
    let color_threshold =
        f32::from(DISPERSED[idx_in_dispersion_matrix]) / f32::from(DISPERSION_MATRIX_SIZE);

    if mix < color_threshold {
        main_color
    } else {
        alternative_color
    }
}

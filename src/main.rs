#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::missing_const_for_fn,
    clippy::cast_possible_wrap,
    clippy::missing_panics_doc
)]
use common::{constrain, lerp_u8};
use geometry::{Line, Triangle};
use graphics::circle;
use math::Vec2;
use simple_pixels::{rgb::RGBA8, start, Config, Context, KeyCode, State};

mod cli;
mod clock;
mod common;
mod geometry;
mod graphics;
mod math;
mod ppt;
mod sprite;

use clock::Clock;

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
    triangle: Triangle,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let clock = Clock::new();
        let mouse_pos = Vec2::new(0.0, 0.0);
        let mut sprites: Vec<Sprite> = Vec::new();
        // sprites.push(Sprite::from_vec2(
        //     circle(Vec2::new(0.0, 0.0), 15.0), //mouse_pos + Vec2::new(25.0, 25.0)),
        //     RGBA8::new(20, 200, 100, 255),
        // ));

        let center = Vec2::new((width / 2) as f32, (height / 2) as f32);

        let triangle = Triangle::new(
            center + Vec2::from_angle(0.0_f32.to_radians()) * 100.0,
            center + Vec2::from_angle(120.0_f32.to_radians()) * 100.0,
            center + Vec2::from_angle(240.0_f32.to_radians()) * 100.0,
        );
        Self {
            clock,
            mouse_pos,
            width,
            height,
            sprites,
            triangle,
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

        //self.sprites[0].origin = self.mouse_pos;

        self.clock.sleep();
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear();
        for sprite in &self.sprites {
            sprite.draw(ctx);
        }
        let points: Vec<Vec2> = self
            .triangle
            .solid_color()
            .into_iter()
            .filter(|p| p.inside(self.width as i32, self.height as i32))
            .collect();

        let shading = Shading::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(self.width as f32, self.height as f32),
            RGBA8::new(255, 0, 0, 255),
            RGBA8::new(0, 0, 255, 255),
        );

        shading.draw_shaded(ctx, &points);
        //shading.draw_dithered(ctx, &points);
    }
}

struct Shading {
    line: Line,
    start_color: RGBA8,
    end_color: RGBA8,
}

impl Shading {
    pub fn new(start_pos: Vec2, end_pos: Vec2, start_color: RGBA8, end_color: RGBA8) -> Self {
        let line = Line::new(start_pos, end_pos);
        Self {
            line,
            start_color,
            end_color,
        }
    }

    pub fn draw_dithered(&self, ctx: &mut Context, points: &[Vec2]) {
        for point in points {
            let (x, y) = (point.x as i32, point.y as i32);
            let mix = self.line.todo_name(*point);
            ctx.draw_pixel(x, y, dither(x, y, self.start_color, self.end_color, mix))
        }
    }

    pub fn draw_shaded(&self, ctx: &mut Context, points: &[Vec2]) {
        let RGBA8 { r, g, b, a } = self.start_color;
        let (sr, sg, sb) = (r, g, b);
        let RGBA8 { r, g, b, a } = self.end_color;
        for point in points {
            let (x, y) = (point.x as i32, point.y as i32);
            let mix = self.line.todo_name(*point);
            ctx.draw_pixel(
                x,
                y,
                RGBA8::new(
                    lerp_u8(sr, r, mix),
                    lerp_u8(sg, g, mix),
                    lerp_u8(sb, b, mix),
                    255,
                ),
            )
            //ctx.draw_pixel(x, y, dither(x, y, self.start_color, self.end_color, mix))
        }
        // {
        //     ctx.draw_pixel(
        //         point.x as i32,
        //         point.y as i32,
        //         dither(
        //             point.x as i32,
        //             point.y as i32,
        //             RGBA8::new(0, 100, 255, 50),
        //             RGBA8::new(0x6a, 0xc0, 0xbd, 50),
        //             (self.clock.now() / 10.0).sin(),
        //         ),
        //     );
        // }
    }
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

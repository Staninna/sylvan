#![allow(dead_code)]
// This is a file containing a verlet solver that renders balls on the VGA buffer.

use crate::os::vga::{Color, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};

pub struct Ball {
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    x_acc: f64,
    y_acc: f64,
    mass: f64,
    drag: f64,
    color: Color,
    radius: usize,
}

impl Ball {
    pub fn new(
        pos: (f64, f64),
        vel: (f64, f64),
        acc: (f64, f64),
        mass: f64,
        drag: f64,
        color: Color,
        radius: usize,
    ) -> Self {
        Self {
            x_pos: pos.0,
            y_pos: pos.1,
            x_vel: vel.0,
            y_vel: vel.1,
            x_acc: acc.0,
            y_acc: acc.1,
            mass,
            drag,
            color,
            radius,
        }
    }

    pub fn new_random(rng: &mut impl Rng, pos: (f64, f64), radius: usize) -> Self {
        let vel = (rng.gen_range(-10.0..=10.0), rng.gen_range(-10.0..=10.0));
        let acc = (rng.gen_range(-10.0..=10.0), rng.gen_range(-10.0..=10.0));
        let mass = rng.gen_range(1.0..=10.0);
        let drag = rng.gen_range(0.0..=1.0);
        let color = rng.gen_range(0..=15);

        Self::new(pos, vel, acc, mass, drag, Color::from(color), radius)
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        let dx = self.x_pos - other.x_pos;
        let dy = self.y_pos - other.y_pos;

        let dist_squared = dx * dx + dy * dy;
        let mut radii_sum_squared = (self.radius + other.radius) as f64;
        radii_sum_squared *= radii_sum_squared;

        dist_squared <= radii_sum_squared
    }

    pub fn draw(&self) {
        draw_ball(self);
    }

    pub fn update(&mut self) {
        update(self, 0.01);
    }
}

/// Update pos and vel using "Velocity Verlet" integration
/// @param dt DeltaTime / time step [eg: 0.01]
pub fn update(ball: &mut Ball, dt: f64) {
    let new_x_pos = ball.x_pos + ball.x_vel * dt + ball.x_acc * (dt * dt * 0.5);
    let new_y_pos = ball.y_pos + ball.y_vel * dt + ball.y_acc * (dt * dt * 0.5);
    // let new_x_acc = apply_forces(ball);
    // let new_y_acc = apply_forces(ball);
    let new_x_acc = 0.25;
    let new_y_acc = 0.25;
    let new_x_vel = ball.x_vel + (ball.x_acc + new_x_acc) * (dt * 0.5);
    let new_y_vel = ball.y_vel + (ball.y_acc + new_y_acc) * (dt * 0.5);

    ball.x_pos = new_x_pos;
    ball.y_pos = new_y_pos;
    ball.x_vel = new_x_vel;
    ball.y_vel = new_y_vel;
    ball.x_acc = new_x_acc;
    ball.y_acc = new_y_acc;
}

pub fn apply_forces(ball: &Ball) -> f64 {
    let grav_acc = 0.0; // 9.81 m/s² down in the z-axis
    let drag_force = 0.5 * ball.drag * (ball.x_vel * ball.x_vel); // D = 0.5 * (rho * C * Area * vel^2)
    let drag_acc = drag_force / ball.mass; // a = F/m
    grav_acc - drag_acc
}

pub fn draw_ball(ball: &Ball) {
    let x_pos = ball.x_pos as usize;
    let y_pos = ball.y_pos as usize;
    let radius = ball.radius;
    let color = ball.color;

    let mut writer = WRITER.lock();

    for y in (y_pos - radius)..=(y_pos + radius) {
        for x in (x_pos - radius)..=(x_pos + radius) {
            let dist = ((x_pos - x) * (x_pos - x) + (y_pos - y) * (y_pos - y)) as f64;
            if dist <= (radius * radius) as f64 {
                writer.buffer.chars[y][x].write(crate::os::vga::ScreenChar {
                    ascii: b' ',
                    color_code: crate::os::vga::ColorCode::new(color, color),
                });
            }
        }
    }
}

use arrayvec::ArrayVec;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub static mut SEED: u64 = 1;
pub fn create_balls() -> ArrayVec<Ball, 25> {
    let mut rng = SmallRng::seed_from_u64(unsafe { SEED });
    let mut balls = ArrayVec::<Ball, 25>::new();

    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 1000;

    while balls.len() < 25 && attempts < MAX_ATTEMPTS {
        let pos = (
            rng.gen_range(0.0 + 5.0..=BUFFER_WIDTH as f64 - 5.0),
            rng.gen_range(0.0 + 5.0..=BUFFER_HEIGHT as f64 - 5.0),
        );

        let radius = rng.gen_range(1..=10);
        let ball = Ball::new_random(&mut rng, pos, radius);

        if !balls
            .iter()
            .any(|existing_ball| ball.collides_with(existing_ball))
        {
            balls.push(ball);
        }

        attempts += 1;
    }

    balls
}

use super::constraint::Constraint;
use super::vec2::Vec2;
use crate::os::vga::{Color, Writer, BUFFER_HEIGHT, BUFFER_WIDTH};
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Copy, Clone, Debug)]
pub struct Blob {
    pos_cur: Vec2,
    pos_old: Vec2,
    acc: Vec2,
    color: Color,
    size: f32,
}

impl Blob {
    pub fn new(pos: Vec2, size: f32, color: Color) -> Self {
        Self {
            pos_cur: pos,
            pos_old: pos,
            acc: Vec2::ZERO,
            color,
            size,
        }
    }

    pub fn random(rng: &mut SmallRng, constraint: Constraint) -> Self {
        // Generate random position within the constraint
        let max_tries = 1000;
        let mut pos = Vec2::ZERO;
        for _ in 0..max_tries {
            pos = Vec2::new(
                rng.gen_range(0.0..=BUFFER_WIDTH as f32),
                rng.gen_range(0.0..=BUFFER_HEIGHT as f32),
            );
            if (pos - constraint.position).length() < constraint.radius {
                break;
            }
        }

        let size = rng.gen_range(1.0..=5.0);
        let color = Color::from(rng.gen_range(1..=15));

        Self::new(pos, size, color)
    }

    pub fn update_position(&mut self, time: f32) {
        let vel = self.pos_cur - self.pos_old;
        self.pos_old = self.pos_cur;
        self.pos_cur = self.pos_cur + vel + self.acc * time * time;
        self.acc = Vec2::ZERO;
    }

    pub fn update_acceleration(&mut self, acc: Vec2) {
        self.acc += acc;
    }

    pub fn update_constraint(&mut self, constraints_pos: Vec2, constraints_radius: f32) {
        let mut to_next = constraints_pos - self.pos_cur;
        if to_next.length() > constraints_radius {
            to_next = to_next * (constraints_radius / to_next.length());
        }
        self.pos_cur = constraints_pos - to_next;
    }

    pub fn update_collision(&mut self, other: &mut Blob) {
        let collision_axis = self.pos_cur - other.pos_cur;
        let distance = collision_axis.length();
        let minimum_distance = self.size / 2.0 + other.size / 2.0;
        if distance < minimum_distance {
            let n = collision_axis / distance;
            let delta = minimum_distance - distance;
            self.pos_cur += 0.5 * delta * n;
            other.pos_cur -= 0.5 * delta * n;
        }
    }

    pub fn draw(&self) {
        let x_pos = self.pos_cur.x as usize;
        let y_pos = self.pos_cur.y as usize;

        if x_pos < (self.size / 2.0) as usize
            || x_pos > (BUFFER_WIDTH as f32 - (self.size / 2.0)) as usize
            || y_pos < (self.size / 2.0) as usize
            || y_pos > (BUFFER_HEIGHT as f32 - (self.size / 2.0)) as usize
        {
            return;
        }

        let radius = (self.size / 2.0) as usize;
        let color = self.color;

        for y in (y_pos - radius)..=(y_pos + radius) {
            for x in (x_pos - radius)..=(x_pos + radius) {
                let dist = ((x_pos - x) * (x_pos - x) + (y_pos - y) * (y_pos - y)) as f32;
                if dist <= (radius * radius) as f32 {
                    Writer::write_char(' ', x, y, color, color);
                }
            }
        }
    }
}

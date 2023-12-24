use super::{blob::Blob, constraint::Constraint, vec2::Vec2};
use arrayvec::ArrayVec;

pub const MAX_BLOBS: usize = 10;

pub struct Solver {
    blobs: ArrayVec<Blob, MAX_BLOBS>,
    gravity: Vec2,
    constraint: Constraint,
    iterations: i32,
    time: f32,
}

impl Solver {
    // Make new solver object
    pub fn new(
        blobs: ArrayVec<Blob, MAX_BLOBS>,
        gravity: Vec2,
        constraint: Constraint,
        iterations: i32,
        time: f32,
    ) -> Self {
        Self {
            blobs,
            gravity,
            constraint,
            iterations,
            time,
        }
    }

    // Update all blobs in the solver
    pub fn update(&mut self) {
        let sub_time_step = self.time / self.iterations as f32;
        for _ in 0..self.iterations {
            self.solve_acceleration();
            self.solve_constraint();
            self.solve_collision();
            self.solve_position(sub_time_step);
        }
    }

    fn solve_acceleration(&mut self) {
        for blob in &mut self.blobs {
            blob.update_acceleration(self.gravity);
        }
    }

    fn solve_constraint(&mut self) {
        for blob in &mut self.blobs {
            blob.update_constraint(self.constraint.position, self.constraint.radius);
        }
    }

    fn solve_collision(&mut self) {
        let mut blobs = self.blobs.as_mut_slice();
        while let [first, tail @ ..] = blobs {
            for second in tail.iter_mut() {
                first.update_collision(second);
            }
            blobs = tail
        }
    }

    fn solve_position(&mut self, time: f32) {
        for blob in &mut self.blobs {
            blob.update_position(time);
        }
    }

    pub fn draw(&self) {
        for blob in &self.blobs {
            blob.draw();
        }
    }

    pub fn add_blob(&mut self, blob: Blob) {
        self.blobs.push(blob);
    }

    pub fn remove_blob(&mut self, index: usize) {
        self.blobs.remove(index);
    }
}

use super::vec2::Vec2;

#[derive(Copy, Clone, Debug)]
pub struct Constraint {
    pub position: Vec2,
    pub radius: f32,
}

impl Constraint {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Self { position, radius }
    }
}

use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn clamp_length_max(&self, max: f32) -> Vec2 {
        let mut vec = *self;
        let len = vec.length();
        if len > max * max {
            vec = vec * (max / len);
        }
        vec
    }

    pub fn clamp(&self, position: Vec2, radius: f32) -> Vec2 {
        let mut vec = *self;
        let len = (vec - position).length();
        if len > radius * radius {
            vec = position + (vec - position) * (radius / len);
        }
        vec
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vec2
where
    Self: Sub<Output = Self> + Copy,
{
    fn sub_assign(&mut self, other: Vec2) {
        *self = *self - other;
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = *self + other;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f32) -> Vec2 {
        Vec2::new(self.x * other, self.y * other)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2::new(self * other.x, self * other.y)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, other: f32) -> Vec2 {
        Vec2::new(self.x / other, self.y / other)
    }
}

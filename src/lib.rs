
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2D {
    pub x: i64,
    pub y: i64,
}

impl Vec2D {
    pub fn rotate_right(&self) -> Self {
        Vec2D { x: -self.y, y: self.x }
    }

    pub fn add(&self, other: &Vec2D) -> Vec2D {
        Vec2D { x: self.x + other.x, y: self.y + other.y }
    }
}

impl std::ops::Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, other: Vec2D) -> Vec2D {
        Vec2D { x: self.x + other.x, y: self.y + other.y }
    }
}
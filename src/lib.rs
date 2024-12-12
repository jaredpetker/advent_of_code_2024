#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vec2D {
    pub x: i64,
    pub y: i64,
}

impl Vec2D {

    pub fn new(x: i64, y: i64) -> Self {
        Vec2D { x, y }
    }

    pub fn up() -> Self {
        Vec2D { x: 0, y: -1 }
    }

    pub fn down() -> Self {
        Vec2D { x: 0, y: 1 }
    }

    pub fn left() -> Self {
        Vec2D { x: -1, y: 0 }
    }

    pub fn right() -> Self {
        Vec2D { x: 1, y: 0 }
    }

    pub fn rotate_left(&self) -> Self {
        Vec2D { x: self.y, y: -self.x }
    }

    pub fn rotate_right(&self) -> Self {
        Vec2D { x: -self.y, y: self.x }
    }

    pub fn add(&self, other: &Vec2D) -> Vec2D {
        Vec2D { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn prod_vec(&self, other: &Vec2D) -> Vec2D {
        Vec2D { x: self.x * other.x, y: self.y * other.y }
    }

}

impl std::ops::Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, other: Vec2D) -> Vec2D {
        Vec2D { x: self.x + other.x, y: self.y + other.y }
    }
}

impl std::ops::AddAssign<Vec2D> for Vec2D {
    fn add_assign(&mut self, other: Vec2D) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Sub<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn sub(self, other: Vec2D) -> Vec2D {
        Vec2D { x: self.x - other.x, y: self.y - other.y }
    }
}

impl std::ops::SubAssign<Vec2D> for Vec2D {
    fn sub_assign(&mut self, other: Vec2D) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Mul<i64> for Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: i64) -> Vec2D {
        Vec2D { x: self.x * rhs, y: self.y * rhs }
    }
}

impl std::ops::MulAssign<i64> for Vec2D {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Div<i64> for Vec2D {
    type Output = Vec2D;

    fn div(self, rhs: i64) -> Vec2D {
        Vec2D { x: self.x / rhs, y: self.y / rhs }
    }
}

impl std::ops::DivAssign<i64> for Vec2D {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl std::ops::Neg for Vec2D {
    type Output = Vec2D;

    fn neg(self) -> Vec2D {
        Vec2D { x: -self.x, y: -self.y }
    }
}

pub struct Rect2D {
    pub top_left: Vec2D,
    pub bottom_right: Vec2D,
}

impl Rect2D {
    pub fn new(top_left: Vec2D, bottom_right: Vec2D) -> Self {
        Rect2D { top_left, bottom_right }
    }

    pub fn contains(&self, point: Vec2D) -> bool {
        point.x >= self.top_left.x && point.x < self.bottom_right.x &&
        point.y >= self.top_left.y && point.y < self.bottom_right.y
    }
}


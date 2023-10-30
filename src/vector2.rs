use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::vector3::Vector3;

pub const ZERO: Vector2 = Vector2 {
    x: 0.,
    y: 0.
};
pub const IDENTITY: Vector2 = Vector2 {
    x: 1.,
    y: 1.
};
pub const RIGHT: Vector2 = Vector2 {
    x: 1.,
    y: 0.
};
pub const LEFT: Vector2 = Vector2 {
    x: -1.,
    y: 0.
};
pub const UP: Vector2 = Vector2 {
    x: 0.,
    y: 1.
};
pub const DOWN: Vector2 = Vector2 {
    x: 0.,
    y: -1.
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn xy(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    
    pub fn normalized(&self) -> Option<Vector2> {
        if self.len_squared() == 0. {
            return None;
        }

        Some(self / self.len())
    }

    pub fn normalized_or_zero(&self) -> Vector2 {
        self.normalized().unwrap_or(ZERO)
    }

    pub fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn angle_between(&self, other: &Vector2) -> f32 {
        self.cos_between(&other).acos()
    }

    pub fn cos_between(&self, other: &Vector2) -> f32 {
        self.dot(&other) / self.len() / other.len()
    }

    pub fn sin_between(&self, other: &Vector2) -> f32 {
        (1. - self.cos_between(&other).powi(2)).sqrt()
    }

    pub fn projected_onto(&self, other: &Vector2) -> Vector2 {
        other * (other.dot(&self) / other.len_squared())
    }

    pub fn projection_of(&self, other: &Vector2) -> Vector2 {
        self * (self.dot(&other) / self.len_squared())
    }

    pub fn rejection_from(&self, other: &Vector2) -> Vector2 {
        self - &self.projected_onto(&other)
    }

    pub fn rejection_of(&self, other: &Vector2) -> Vector2 {
        other - &other.projected_onto(&self)
    }
    
    pub fn xyz(&self, z: f32) -> Vector3 {
        Vector3 { x: self.x, y: self.y, z }
    }
    
    pub fn xy0(&self) -> Vector3 {
        Vector3 { x: self.x, y: self.y, z: 0. }
    }
    
    pub fn yx(&self) -> Vector2 {
        Vector2 { x: self.y, y: self.x }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Add for &Vector2 {
    type Output = Vector2;

    fn add(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Vector2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl AddAssign<&Vector2> for Vector2 {
    fn add_assign(&mut self, other: &Vector2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Sub for &Vector2 {
    type Output = Vector2;

    fn sub(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Vector2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl SubAssign<&Vector2> for Vector2 {
    fn sub_assign(&mut self, other: &Vector2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }
}

impl Mul<f32> for &Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }
}

impl Mul<&f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: &f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }
}

impl Mul<&f32> for &Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: &f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, vector: Vector2) -> Vector2 {
        Vector2 {
            x: vector.x * self,
            y: vector.y * self
        }
    }
}

impl Mul<&Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, vector: &Vector2) -> Vector2 {
        Vector2 {
            x: vector.x * self,
            y: vector.y * self
        }
    }
}

impl Mul<Vector2> for &f32 {
    type Output = Vector2;

    fn mul(self, vector: Vector2) -> Vector2 {
        Vector2 {
            x: vector.x * self,
            y: vector.y * self
        }
    }
}

impl Mul<&Vector2> for &f32 {
    type Output = Vector2;

    fn mul(self, vector: &Vector2) -> Vector2 {
        Vector2 {
            x: vector.x * self,
            y: vector.y * self
        }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl MulAssign<&f32> for Vector2 {
    fn mul_assign(&mut self, scalar: &f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x / scalar,
            y: self.y / scalar
        }
    }
}

impl Div<f32> for &Vector2 {
    type Output = Vector2;

    fn div(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x / scalar,
            y: self.y / scalar
        }
    }
}

impl Div<&f32> for Vector2 {
    type Output = Vector2;

    fn div(self, scalar: &f32) -> Vector2 {
        Vector2 {
            x: self.x / scalar,
            y: self.y / scalar
        }
    }
}

impl Div<&f32> for &Vector2 {
    type Output = Vector2;

    fn div(self, scalar: &f32) -> Vector2 {
        Vector2 {
            x: self.x / scalar,
            y: self.y / scalar
        }
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl DivAssign<&f32> for Vector2 {
    fn div_assign(&mut self, scalar: &f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Neg for &Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y
        }
    }
}

impl ToString for Vector2 {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl ToString for &Vector2 {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}
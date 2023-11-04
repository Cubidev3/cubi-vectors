use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use mint::IntoMint;
use crate::vector3::Vector3;
use crate::vector_base;

vector_base!(Vector2<f32> {x,y});

impl Vector2 {
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

    pub fn from_xy(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
    pub fn from_value(value: f32) -> Vector2 {
        Vector2 { x: value, y: value } }
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalized(&self) -> Option<Vector2> {
        if self.len_squared() == 0.0 {
            return None;
        }

        Some(*self / self.len())
    }

    pub fn normalized_or_zero(&self) -> Vector2 {
        self.normalized().unwrap_or(Vector2::ZERO)
    }

    pub fn dot(&self, other: Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn angle_between(&self, other: Vector2) -> f32 {
        self.cos_between(other).acos()
    }

    pub fn cos_between(&self, other: Vector2) -> f32 {
        self.dot(other) / self.len() / other.len()
    }

    pub fn sin_between(&self, other: Vector2) -> f32 {
        (1. - self.cos_between(other).powi(2)).sqrt()
    }

    pub fn projected_onto(&self, other: Vector2) -> Vector2 {
        other * (other.dot(*self) / other.len_squared())
    }

    pub fn projection_of(&self, other: Vector2) -> Vector2 {
        *self * (self.dot(other) / self.len_squared())
    }

    pub fn rejection_from(&self, other: Vector2) -> Vector2 {
        *self - self.projected_onto(other)
    }

    pub fn rejection_of(&self, other: Vector2) -> Vector2 {
        other - other.projected_onto(*self)
    }

    pub fn is_almost_zero(&self) -> bool {
        self.len_squared() <= f32::EPSILON * f32::EPSILON
    }

    pub fn is_almost_equal_to(&self, other: Vector2) -> bool { (*self - other).is_almost_zero() }

    pub fn rotated_by(&self, rotation_radians: f32) -> Vector2 {
        Vector2 {
            x: self.x * rotation_radians.cos() - self.y * rotation_radians.sin(),
            y: self.x * rotation_radians.sin() + self.y * rotation_radians.cos()
        }
    }

    pub fn abs(&self) -> Vector2 {
        Vector2 { x: self.x.abs(), y: self.y.abs() }
    }

    pub fn sign(&self) -> Vector2 {
        Vector2 { x: self.x.signum(), y: self.y.signum() }
    }

    pub fn element_wise_product(&self, other: Vector2) -> Vector2 {
        Vector2 { x: self.x * other.x, y: self.y * other.y }
    }

    pub fn with_sign_of(&self, other: Vector2) -> Vector2 {
        Vector2 { x: self.x.abs() * other.x.signum(), y: self.y.abs() * other.y.signum() }
    }

    pub fn is_x_inside_interval(&self, min: f32, max: f32) -> bool {
        min <= self.x && self.x <= max
    }

    pub fn is_y_inside_interval(&self, min: f32, max: f32) -> bool {
        min <= self.y && self.y <= max
    }

    pub fn flipped_x(&self) -> Vector2 {
        Vector2 { x: -self.x, y: self.y }
    }

    pub fn flipped_y(&self) -> Vector2 {
        Vector2 { x: self.x, y: -self.y }
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

impl Default for Vector2 {
    fn default() -> Vector2 {
        Vector2::ZERO
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

impl Into<mint::Vector2<f32>> for Vector2 {
    fn into(self) -> mint::Vector2<f32> {
        mint::Vector2 { x: self.x, y: self.y }
    }
}

impl IntoMint for Vector2 {
    type MintType = mint::Vector2<f32>;
}
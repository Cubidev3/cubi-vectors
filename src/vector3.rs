use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use mint::IntoMint;

use crate::vector2::Vector2;
use crate::vector_base;

vector_base!(Vector3<f32> {x,y,z});

impl Vector3 {
    pub const ZERO: Vector3 = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    pub const IDENTITY: Vector3 = Vector3 {
        x: 1.,
        y: 1.,
        z: 1.,
    };
    pub const RIGHT: Vector3 = Vector3 {
        x: 1.,
        y: 0.,
        z: 0.,
    };
    pub const LEFT: Vector3 = Vector3 {
        x: -1.,
        y: 0.,
        z: 0.,
    };
    pub const UP: Vector3 = Vector3 {
        x: 0.,
        y: 1.,
        z: 0.,
    };
    pub const DOWN: Vector3 = Vector3 {
        x: 0.,
        y: -1.,
        z: 0.,
    };
    pub const FORWARD: Vector3 = Vector3 {
        x: 0.,
        y: 0.,
        z: 1.,
    };
    pub const BACKWARD: Vector3 = Vector3 {
        x: 0.,
        y: 0.,
        z: 1.,
    };

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn from_value(value: f32) -> Vector3 {
        Vector3 { x: value, y: value, z: value }
    }

    pub fn from_xy(xy: Vector2, z: f32) -> Vector3 {
        Vector3 {
            x: xy.x,
            y: xy.y,
            z
        }
    }

    pub fn from_xz(xz: Vector2, y: f32) -> Vector3 {
        Vector3 {
            x: xz.x,
            y,
            z: xz.y
        }
    }

    pub fn from_yz(yz: Vector2, x: f32) -> Vector3 {
        Vector3 {
            x,
            y: yz.x,
            z: yz.y
        }
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn normalized(&self) -> Option<Vector3> {
        if self.len_squared() == 0.0 {
            return None;
        }
        
        Some(*self / self.len())
    }
    
    pub fn normalized_or_zero(&self) -> Vector3 {
        self.normalized().unwrap_or(Vector3::ZERO)
    }

    pub fn dot(&self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn angle_between(&self, other: Vector3) -> f32 {
        self.cos_between(other).acos()
    }

    pub fn cos_between(&self, other: Vector3) -> f32 {
        self.dot(other) / self.len() / other.len()
    }

    pub fn sin_between(&self, other: Vector3) -> f32 {
        self.cross(other).len() / self.len() / other.len()
    }

    pub fn projected_onto(&self, other: Vector3) -> Vector3 {
        other * (other.dot(*self) / other.len_squared())
    }

    pub fn projection_of(&self, other: Vector3) -> Vector3 {
        *self * (self.dot(other) / self.len_squared())
    }

    pub fn rejection_from(&self, other: Vector3) -> Vector3 {
        *self - self.projected_onto(other)
    }

    pub fn rejection_of(&self, other: Vector3) -> Vector3 {
        other - other.projected_onto(*self)
    }

    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn cross_in_direction_of(&self, other: Vector3, direction: Vector3) -> Vector3 {
        let cross = self.cross(other);

        if direction.dot(cross).is_sign_negative() {
            return -cross;
        }

        cross
    }

    pub fn rotated_around_x_by(&self, rotation_radians: f32) -> Vector3 {
        Vector3::from_yz(self.yz().rotated_by(rotation_radians), self.x)
    }

    pub fn rotated_around_y_by(&self, rotation_radians: f32) -> Vector3 {
        Vector3::from_xz(self.xz().rotated_by(rotation_radians), self.y)
    }

    pub fn rotated_around_z_by(&self, rotation_radians: f32) -> Vector3 {
        Vector3::from_xy(self.xy().rotated_by(rotation_radians), self.z)
    }

    pub fn abs(&self) -> Vector3 {
        Vector3 { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }

    pub fn sign(&self) -> Vector3 {
        Vector3 { x: self.x.signum(), y: self.y.signum(), z: self.z.signum() }
    }

    pub fn element_wise_product(&self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }

    pub fn with_sign_of(&self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x.abs() * other.x.signum(), y: self.y.abs() * other.y.signum(), z: self.z.abs() * other.z.signum()}
    }

    pub fn is_almost_zero(&self) -> bool {
        self.len_squared() <= f32::EPSILON * f32::EPSILON
    }

    pub fn is_x_inside_interval(&self, min: f32, max: f32) -> bool {
        min <= self.x && self.x <= max
    }

    pub fn is_y_inside_interval(&self, min: f32, max: f32) -> bool {
        min <= self.y && self.y <= max
    }

    pub fn is_z_inside_interval(&self, min: f32, max: f32) -> bool {
        min <= self.z && self.z <= max
    }

    pub fn flipped_x(&self) -> Vector3 {
        Vector3 { x: -self.x, y: self.y, z: self.z }
    }

    pub fn flipped_y(&self) -> Vector3 {
        Vector3 { x: self.x, y: -self.y, z: self.z }
    }

    pub fn flipped_z(&self) -> Vector3 {
        Vector3 { x: self.x, y: self.y, z: -self.z }
    }

    pub fn flipped_xy(&self) -> Vector3 {
        Vector3 { x: -self.x, y: -self.y, z: self.z }
    }

    pub fn flipped_xz(&self) -> Vector3 {
        Vector3 { x: -self.x, y: self.y, z: -self.z }
    }

    pub fn flipped_yz(&self) -> Vector3 {
        Vector3 { x: self.x, y: -self.y, z: -self.z }
    }
    
    pub fn xy(&self) -> Vector2 {
        Vector2 { x: self.x, y: self.y }
    }
    
    pub fn xz(&self) -> Vector2 {
        Vector2 { x: self.x, y: self.z }
    }
    
    pub fn yz(&self) -> Vector2 {
        Vector2 { x: self.y, y: self.z }
    }
    
    pub fn yx(&self) -> Vector2 {
        Vector2 { x: self.y, y: self.x }
    }

    pub fn zx(&self) -> Vector2 {
        Vector2 { x: self.z, y: self.x }
    }

    pub fn zy(&self) -> Vector2 {
        Vector2 { x: self.z, y: self.y }
    }
}

impl Default for Vector3 {
    fn default() -> Vector3 {
        Vector3::ZERO
    }
}

impl ToString for Vector3 {
    fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ToString for &Vector3 {
    fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Into<mint::Vector3<f32>> for Vector3 {
    fn into(self) -> mint::Vector3<f32> {
        mint::Vector3 { x: self.x, y: self.y, z: self.z }
    }
}

impl IntoMint for Vector3 {
    type MintType = mint::Vector3<f32>;
}

impl Into<Vector3> for mint::Vector3<f32> {
    fn into(self) -> Vector3 {
        Vector3 { x: self.x, y: self.y, z: self.z }
    }
}
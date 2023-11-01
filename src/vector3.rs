use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::vector2::Vector2;

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

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn xyz(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
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
        if self.len_squared() == 0. {
            return None;
        }
        
        Some(self / self.len())
    }
    
    pub fn normalized_or_zero(&self) -> Vector3 {
        self.normalized().unwrap_or(ZERO)
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn angle_between(&self, other: &Vector3) -> f32 {
        self.cos_between(&other).acos()
    }

    pub fn cos_between(&self, other: &Vector3) -> f32 {
        self.dot(&other) / self.len() / other.len()
    }

    pub fn sin_between(&self, other: &Vector3) -> f32 {
        self.cross(&other).len() / self.len() / other.len()
    }

    pub fn projected_onto(&self, other: &Vector3) -> Vector3 {
        other * (other.dot(&self) / other.len_squared())
    }

    pub fn projection_of(&self, other: &Vector3) -> Vector3 {
        self * (self.dot(&other) / self.len_squared())
    }

    pub fn rejection_from(&self, other: &Vector3) -> Vector3 {
        self - self.projected_onto(&other)
    }

    pub fn rejection_of(&self, other: &Vector3) -> Vector3 {
        other - other.projected_onto(&self)
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn cross_in_direction_of(&self, other: &Vector3, direction: &Vector3) -> Vector3 {
        let cross = self.cross(&other);

        if direction.dot(&cross).is_sign_negative() {
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

    pub fn is_almost_zero(&self) -> bool {
        self.len_squared() <= f32::EPSILON * f32::EPSILON
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
        ZERO
    }
}
impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    fn add(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, other: &Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl SubAssign<&Vector3> for Vector3 {
    fn sub_assign(&mut self, other: &Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<&f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: &f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<&f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: &f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Vector3 {
        Vector3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl Mul<&Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, vector: &Vector3) -> Vector3 {
        Vector3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl Mul<Vector3> for &f32 {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Vector3 {
        Vector3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl Mul<&Vector3> for &f32 {
    type Output = Vector3;

    fn mul(self, vector: &Vector3) -> Vector3 {
        Vector3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl MulAssign<&f32> for Vector3 {
    fn mul_assign(&mut self, scalar: &f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Div<&f32> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: &f32) -> Vector3 {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Div<&f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, scalar: &f32) -> Vector3 {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl DivAssign<&f32> for Vector3 {
    fn div_assign(&mut self, scalar: &f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
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
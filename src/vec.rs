use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

#[rustfmt::skip]
impl Vec3 {
    pub fn x(self) -> f32 { self.0 }
    pub fn y(self) -> f32 { self.1 }
    pub fn z(self) -> f32 { self.2 }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn squared_length(self) -> f32 { self.dot(self) }
    pub fn length(self) -> f32 { self.squared_length().sqrt() }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3( self.1 * other.2 - self.2 * other.1,
            -(self.0 * other.2 - self.2 * other.0),
              self.0 * other.1 - self.1 * other.0)
    }

    pub fn to_unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, scalar: f32) -> Vec3 {
        Vec3(self.0 + scalar, self.1 + scalar, self.2 + scalar)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3(self * vector.0, self * vector.1, self * vector.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        scalar * self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3(self.0 * vector.0, self.1 * vector.1, self.2 * vector.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        (1.0 / scalar) * self
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

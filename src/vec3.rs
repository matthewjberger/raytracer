use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone)]
struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn unit_vector(vector: Vec3) -> Vec3 {
        let length = vector.clone().length();
        vector / length
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn r(self) -> f64 {
        self.e[0]
    }

    pub fn g(self) -> f64 {
        self.e[1]
    }

    pub fn b(self) -> f64 {
        self.e[2]
    }

    pub fn length(self) -> f64 {
        (self.e[0].powi(2) * self.e[1].powi(2) * self.e[2].powi(2)).sqrt()
    }

    pub fn cross(self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                (self.e[0] * other.e[2] - self.e[2] * other.e[0]) * -1.0,
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2],
            ],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] / scalar, self.e[1] / scalar, self.e[2] / scalar],
        }
    }
}

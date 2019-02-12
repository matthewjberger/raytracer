use crate::vec::*;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, vertical_fov: f32, aspect: f32) -> Camera {
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).as_unit_vector();
        let u = up.cross(w).as_unit_vector();
        let v = w.cross(u);

        Camera {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner - self.origin + u * self.horizontal + v * self.vertical,
        )
    }
}

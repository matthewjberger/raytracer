use crate::random::*;
use crate::vec::*;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f32,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub shutter_opened_time: f32,
    pub shutter_closed_time: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct CameraConfiguration {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub vertical_fov: f32,
    pub aspect: f32,
    pub aperture: f32,
    pub focus_dist: f32,
    pub shutter_opened_time: f32,
    pub shutter_closed_time: f32,
}

impl Camera {
    pub fn new(configuration: CameraConfiguration) -> Camera {
        let theta = configuration.vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = configuration.aspect * half_height;

        let w = (configuration.look_from - configuration.look_at).as_unit_vector();
        let u = configuration.up.cross(w).as_unit_vector();
        let v = w.cross(u);

        Camera {
            lower_left_corner: configuration.look_from
                - configuration.focus_dist * (half_width * u + half_height * v + w),
            horizontal: 2.0 * half_width * configuration.focus_dist * u,
            vertical: 2.0 * half_height * configuration.focus_dist * v,
            origin: configuration.look_from,
            lens_radius: configuration.aperture / 2.0,
            u,
            v,
            w,
            shutter_opened_time: configuration.shutter_opened_time,
            shutter_closed_time: configuration.shutter_closed_time,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * crate::material::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let time = self.shutter_opened_time
            + drand48() * (self.shutter_closed_time - self.shutter_opened_time);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}

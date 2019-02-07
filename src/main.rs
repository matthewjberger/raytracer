use std::fs::File;
use std::io::Write;
use crate::vec::{Vec3, Ray};

mod vec;

fn color(r: Ray) -> Vec3 {
    let unit_direction = r.direction.to_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    let filename = "output.ppm";
    let mut output = File::create(filename).unwrap();

    let width = 200;
    let height = 100;
    writeln!(output, "P3\n{} {}\n255", width, height).unwrap();

    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    for y in (0..height).rev() {
        for x in 0..width {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;

            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);

            let color = color(r) * 255.99;;
            writeln!(output, "{} {} {}", color.x() as i32, color.y() as i32, color.z() as i32).unwrap();
        }
    }
}

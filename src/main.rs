extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::Write;

use crate::camera::*;
use crate::material::*;
use crate::model::*;
use crate::vec::*;

mod camera;
mod material;
mod model;
mod vec;

fn color(r: Ray, world: &[Box<Model>], depth: i32) -> Vec3 {
    const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
    const SKY_BLUE: Vec3 = Vec3(0.5, 0.7, 1.0);

    let interval = TimeInterval::new(0.001, i32::max_value() as f32);
    if let Some(hit) = world.hit(&r, &interval) {
        let scatter = hit.material.scatter(&r, &hit);
        if let Some(ray) = scatter.ray {
            if depth < 50 {
                return scatter.color * color(ray, world, depth + 1);
            }
        }
        return Vec3(0.0, 0.0, 0.0);
    }
    let unit_direction = r.direction.as_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * WHITE + t * SKY_BLUE
}

fn main() {
    let filename = "output.ppm";
    let mut output = File::create(filename).unwrap();

    let width = 200;
    let height = 100;
    let samples_per_pixel = 100;
    writeln!(output, "P3\n{} {}\n255", width, height).unwrap();

    let world: Vec<Box<Model>> = vec![
        Box::new(Sphere::new(
            Vec3(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(Vec3(0.1, 0.2, 0.5))),
        )),
        Box::new(Sphere::new(
            Vec3(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::new(Vec3(0.8, 0.8, 0.0))),
        )),
        Box::new(Sphere::new(
            Vec3(1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.3)),
        )),
        Box::new(Sphere::new(
            Vec3(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3(-1.0, 0.0, -1.0),
            -0.45,
            Box::new(Dielectric::new(1.5)),
        )),
    ];

    let camera = Camera::new(
        Vec3(-2.0, 2.0, 1.0),
        Vec3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        45.0,
        width as f32 / height as f32,
    );

    for y in (0..height).rev() {
        for x in 0..width {
            let mut blended_color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f32 + rand::thread_rng().gen_range(0.0, 1.0)) / width as f32;
                let v = (y as f32 + rand::thread_rng().gen_range(0.0, 1.0)) / height as f32;
                let r = camera.get_ray(u, v);
                blended_color = blended_color + color(r, &world, 0);
            }
            blended_color = blended_color / (samples_per_pixel as f32);
            let final_color = Vec3(
                blended_color.x().sqrt(),
                blended_color.y().sqrt(),
                blended_color.z().sqrt(),
            ) * 255.99;

            writeln!(
                output,
                "{} {} {}",
                final_color.x() as i32,
                final_color.y() as i32,
                final_color.z() as i32
            )
            .unwrap();
        }
    }
}

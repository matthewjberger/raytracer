extern crate image;
extern crate num_cpus;
extern crate threadpool;

use image::{ImageBuffer, Pixel, Rgb};
use std::sync::{mpsc::channel, Arc, Mutex};
use threadpool::ThreadPool;

use std::vec::Vec;

use crate::camera::*;
use crate::material::*;
use crate::model::*;
use crate::random::*;
use crate::vec::*;

mod camera;
mod material;
mod model;
mod random;
mod vec;

#[allow(dead_code)]
fn sphere_scene() -> Vec<Box<dyn Model + Send>> {
    vec![
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
    ]
    .into_iter()
    .map(|s| s as Box<dyn Model + Send>)
    .collect()
}

fn random_scene() -> Vec<Box<dyn Model + Send>> {
    let mut models: Vec<Box<dyn Model + Send>> = Vec::new();

    models.push(Box::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3(0.5, 0.5, 0.5))),
    )));

    for x in -11..11 {
        for y in -11..11 {
            let choose_mat = drand48();
            let center = Vec3(x as f32 + 0.9 * drand48(), 0.2, y as f32 + 0.9 * drand48());
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    models.push(Box::new(MovingSphere::new(
                        0.0,
                        1.0,
                        center + Vec3(0.0, 0.5 * drand48(), 0.0),
                        Sphere::new(
                            center,
                            0.2,
                            Box::new(Lambertian::new(Vec3(
                                drand48() * drand48(),
                                drand48() * drand48(),
                                drand48() * drand48(),
                            ))),
                        ),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    models.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(
                            0.5 * Vec3(1.0 + drand48(), 1.0 + drand48(), 1.0 + drand48()),
                            0.5 * drand48(),
                        )),
                    )));
                } else {
                    // glass
                    models.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }
    models.push(Box::new(Sphere::new(
        Vec3(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));

    models.push(Box::new(Sphere::new(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Vec3(0.4, 0.2, 0.1))),
    )));

    models.push(Box::new(Sphere::new(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0)),
    )));

    models
        .into_iter()
        .map(|s| s as Box<dyn Model + Send>)
        .collect()
}

fn color(r: Ray, world: &[Box<dyn Model + Send>], depth: i32) -> Vec3 {
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
    let (width, height) = (800, 400);
    let samples_per_pixel = 100;

    let world = Arc::new(Mutex::new(random_scene()));

    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(CameraConfiguration {
        look_from,
        look_at,
        up: Vec3(0.0, 1.0, 0.0),
        vertical_fov: 20.0,
        aspect: width as f32 / height as f32,
        aperture,
        focus_dist,
        shutter_opened_time: 0.0,
        shutter_closed_time: 1.0,
    });

    let mut img = ImageBuffer::new(width, height);
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in (0..height).rev() {
        let (world, tx) = (Arc::clone(&world), tx.clone());
        pool.execute(move || {
            for x in 0..width {
                let mut blended_color = Vec3(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (x as f32 + drand48()) / width as f32;
                    let v = (y as f32 + drand48()) / height as f32;
                    let r = camera.get_ray(u, v);
                    let world = world.lock().unwrap();
                    blended_color = blended_color + color(r, &*world, 0);
                }
                blended_color = blended_color / (samples_per_pixel as f32);
                let final_color = Vec3(
                    blended_color.x().sqrt(),
                    blended_color.y().sqrt(),
                    blended_color.z().sqrt(),
                ) * 255.99;
                let pixel = Rgb::from_channels(
                    final_color.x() as u8,
                    final_color.y() as u8,
                    final_color.z() as u8,
                    0,
                );
                tx.send((x, y, pixel)).expect("Could not send data!");
            }
        });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv().unwrap();
        img.put_pixel(x, y, pixel)
    }

    image::imageops::flip_vertical(&img);
    let _ = img.save("output.png");
}

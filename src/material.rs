use crate::model::Hit;
use crate::vec::{Ray, Vec3};
use rand::Rng;

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p =
            2.0 * Vec3(
                rand::thread_rng().gen_range(0.0, 1.0),
                rand::thread_rng().gen_range(0.0, 1.0),
                rand::thread_rng().gen_range(0.0, 1.0),
            ) - Vec3(1.0, 1.0, 1.0);
        if p.squared_length() >= 1.0 {
            return p;
        };
    }
}

pub struct Scatter {
    pub color: Vec3,
    pub ray: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Scatter;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &Hit) -> Scatter {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        Scatter {
            color: self.albedo,
            ray: Some(Ray::new(hit.p, target - hit.p)),
        }
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 0.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Scatter {
        let reflected = r_in.direction.as_unit_vector().reflect(hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
        Scatter {
            color: self.albedo,
            ray: if scattered.direction.dot(hit.normal) > 0.0 {
                Some(scattered)
            } else {
                None
            },
        }
    }
}

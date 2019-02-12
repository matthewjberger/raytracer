use crate::model::Hit;
use crate::vec::{Ray, Vec3};
use rand::Rng;

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p =
            2.0 * Vec3(
                rand::thread_rng().gen_range(0.0, 1.0),
                rand::thread_rng().gen_range(0.0, 1.0),
                0.0,
            ) - Vec3(1.0, 1.0, 0.0);
        if p.squared_length() >= 1.0 {
            return p;
        };
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
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

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Scatter {
        let reflected = r_in.direction.reflect(hit.normal);

        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;
        let reflect_prob: f32;

        if r_in.direction.dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction.dot(hit.normal) / r_in.direction.length();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction.dot(hit.normal) / r_in.direction.length();
        }

        let refracted = r_in.direction.refract(outward_normal, ni_over_nt);

        reflect_prob = if refracted.is_some() {
            schlick(cosine, self.ref_idx)
        } else {
            1.0
        };

        let ray = if rand::thread_rng().gen_range(0.0, 1.0) < reflect_prob {
            Ray::new(hit.p, reflected)
        } else {
            Ray::new(hit.p, refracted.unwrap())
        };

        Scatter {
            color: Vec3(1.0, 1.0, 1.0),
            ray: Some(ray),
        }
    }
}

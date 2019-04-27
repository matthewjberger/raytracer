use crate::material::Material;
use crate::vec::{Ray, Vec3};

#[derive(Clone, Copy)]
pub struct Hit<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub struct TimeInterval {
    pub min: f32,
    pub max: f32,
}

impl TimeInterval {
    pub fn new(min: f32, max: f32) -> TimeInterval {
        TimeInterval { min, max }
    }
}

pub trait Model {
    fn hit(&self, r: &Ray, interval: &TimeInterval) -> Option<Hit>;
}

pub struct MovingSphere {
    pub start_time: f32,
    pub end_time: f32,
    pub end_center: Vec3,
    pub geometry: Sphere,
}

impl MovingSphere {
    pub fn new(start_time: f32, end_time: f32, end_center: Vec3, geometry: Sphere) -> MovingSphere {
        MovingSphere {
            start_time,
            end_time,
            end_center,
            geometry,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.geometry.center
            + ((time - self.start_time) / (self.end_time - self.start_time))
                * (self.end_center - self.geometry.center)
    }
}

impl Model for MovingSphere {
    fn hit(&self, r: &Ray, interval: &TimeInterval) -> Option<Hit> {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.geometry.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp > interval.min && temp < interval.max {
                let point = r.point_at_parameter(temp);
                let normal = (point - self.center(r.time)) / self.geometry.radius;
                return Some(Hit {
                    t: temp,
                    p: point,
                    normal,
                    material: &*self.geometry.material,
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp > interval.min && temp < interval.max {
                let point = r.point_at_parameter(temp);
                let normal = (point - self.center(r.time)) / self.geometry.radius;
                return Some(Hit {
                    t: temp,
                    p: point,
                    normal,
                    material: &*self.geometry.material,
                });
            }
        }
        None
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material + Send>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material + Send>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Model for Sphere {
    fn hit(&self, r: &Ray, interval: &TimeInterval) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp > interval.min && temp < interval.max {
                let point = r.point_at_parameter(temp);
                let normal = (point - self.center) / self.radius;
                return Some(Hit {
                    t: temp,
                    p: point,
                    normal,
                    material: &*self.material,
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp > interval.min && temp < interval.max {
                let point = r.point_at_parameter(temp);
                let normal = (point - self.center) / self.radius;
                return Some(Hit {
                    t: temp,
                    p: point,
                    normal,
                    material: &*self.material,
                });
            }
        }
        None
    }
}

impl Model for [Box<dyn Model + Send>] {
    fn hit(&self, r: &Ray, interval: &TimeInterval) -> Option<Hit> {
        let mut closest = None;
        for child in self {
            if let Some(hit) = child.hit(r, interval) {
                match closest {
                    None => closest = Some(hit),
                    Some(previous) => {
                        if hit.t < previous.t {
                            closest = Some(hit)
                        }
                    }
                }
            }
        }
        closest
    }
}

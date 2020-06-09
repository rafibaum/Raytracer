use crate::ray::Ray;
use crate::vec::Vec3;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub face_hit: Face,
}

pub enum Face {
    Front,
    Back,
}

impl HitRecord {
    pub fn new(ray: &Ray, point: &Vec3, t: f64, outward_normal: &Vec3) -> HitRecord {
        let face = if ray.direction.dot(outward_normal) < 0.0 {
            Face::Front
        } else {
            Face::Back
        };

        let normal = match face {
            Face::Front => *outward_normal,
            Face::Back => -(*outward_normal),
        };

        HitRecord {
            point: *point,
            normal,
            t,
            face_hit: face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest = t_max;

        for object in self {
            if let Some(hit) = object.hit(ray, t_min, closest) {
                closest = hit.t;
                temp_rec = Some(hit);
            }
        }

        temp_rec
    }
}

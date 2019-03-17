use super::Ray;
use super::super::vec::{Vec3, dot};

pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(cx: f32, cy: f32, cz: f32, r: f32) -> Sphere {
        Sphere {
            center: Vec3::new(cx, cy, cz),
            radius: r,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        
        let a = dot(&r.direction(), &r.direction());
        let b = 2.0 * dot(&oc, &r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        
        let discriminant = b * b - 4.0*a*c;
        let mut t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t < tmax && t > tmin {
            let p = r.point_at_parameter(t);
            let normal = (p - self.center).unit_vector();
            return Some(HitRecord{
                t, p, normal
            });
        }
        t = (-b + discriminant.sqrt()) / (2.0 * a);
        if t < tmax && t > tmin {
            let p = r.point_at_parameter(t);
            let normal = (p - self.center).unit_vector();
            return Some(HitRecord{
                t, p, normal
            });
        }
        None
    }
}

pub struct HitableList {
    hitable_list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(hitable_list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList {
            hitable_list,
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = tmax;
        for hitable in &self.hitable_list {
            if let Some(hr) = hitable.hit(r, tmin, closest_so_far) {
                closest_so_far = hr.t;
                hit_record = Some(hr);
            }
        }

        hit_record
    }
}
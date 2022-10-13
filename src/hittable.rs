use std::ops::RangeInclusive;
use std::sync::Arc;

use cgmath::{Point3, Vector3, InnerSpace};

use crate::material::Material;
use crate::ray::Ray;
use crate::types::Float;

pub struct HitRecord {
    pub point: Point3<Float>,
    pub normal: Vector3<Float>,
    pub material: Arc<dyn Material + Send + Sync>,
    pub front_face: bool,
    pub t: Float,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<Float>) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3<Float>,
    pub radius: Float,
    pub material: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(center: impl Into<Point3<Float>>, radius: Float, material: &Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center: center.into(),
            radius,
            material: Arc::clone(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<Float>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude2();
        let half_b = cgmath::dot(oc, ray.direction);
        let c = oc.magnitude2() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - (a * c);

        if discriminant < 0.0 { None }
        else {
            let sqrtdet = discriminant.sqrt();
            let root = (-half_b - sqrtdet) / a;
            if !t_range.contains(&root) {
                let root = (-half_b + sqrtdet) / a;
                if !t_range.contains(&root) { return None }
            }
            
            let point = ray.at(root);
            let normal = (point - self.center) / self.radius;
            let front_face = cgmath::dot(ray.direction, normal) < 0.0;
            Some(HitRecord {
                t: root,
                point,
                normal: if front_face { normal } else { -normal },
                material: Arc::clone(&self.material),
                front_face,
            })
        }
    }
}

pub struct Triangle {
    vertecies: [Point3<Float>; 3],
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<Float>) -> Option<HitRecord> {
        let s = self.vertecies[0];
        let o = ray.origin;
        let v = o - s;
        let d_1 = self.vertecies[1] - self.vertecies[0];
        let d_2 = self.vertecies[2] - self.vertecies[0];
        let n = d_1.cross(d_2);

        todo!()
    }
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }
    
    pub fn add(&mut self, obj: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(obj);
    }
    
    pub fn hit(&self, ray: &Ray, t_range: RangeInclusive<Float>) -> Option<HitRecord> {
        let range_start = *t_range.start();
        let mut closest = *t_range.end();
        self.objects.iter().fold(None, |acc, v| {
            match v.hit(ray, range_start..=closest).map(|v| {
                closest = v.t;
                v
            }) {
                Some(v) => Some(v),
                None => acc,
            }
        })
    }
}


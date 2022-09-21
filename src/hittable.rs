use std::ops::RangeInclusive;

use cgmath::{Point3, Vector3, InnerSpace};

use crate::material::Material;
use crate::ray::Ray;
use crate::types::Float;

pub struct HitRecord<'a> {
    pub point: Point3<Float>,
    pub normal: Vector3<Float>,
    pub material: &'a dyn Material,
    pub front_face: bool,
    pub t: Float,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<Float>) -> Option<HitRecord>;
}

pub struct Sphere<'a> {
    pub center: Point3<Float>,
    pub radius: Float,
    pub material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: impl Into<Point3<Float>>, radius: Float, material: &'a dyn Material) -> Self {
        Self {
            center: center.into(),
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
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
                material: self.material,
                front_face,
            })
        }
    }
}


pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }
    
    pub fn add<T: Hittable + 'a>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
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


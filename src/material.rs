use std::ops::RangeInclusive;

use cgmath::{Vector3, InnerSpace, vec3};
use rand::Rng;

use crate::{ray::Ray, hittable::HitRecord, types::Float};


pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<Float>)>;
}

pub struct Lambertian {
    albedo: Vector3<Float>,    
}

fn random_vector(range: RangeInclusive<Float>) -> Vector3<Float> {
    let mut rng = rand::thread_rng();
    let range_start = *range.start();
    vec3(rng.gen(), rng.gen(), rng.gen()) * (range.end() - range_start) + vec3(range_start, range_start, range_start)
}

fn random_unit_sphere() -> Vector3<Float> {
    loop {
        let res = random_vector(-1.0..=1.0);
        if res.magnitude2() < 1.0 { break res }
    }
}

fn is_near_zero(vec: &Vector3<Float>) -> bool {
    const EPSILON: Float = 1e-8;
    (vec.x.abs() < EPSILON) && (vec.y.abs() < EPSILON) && (vec.z.abs() < EPSILON)
}

impl Lambertian {
    pub fn new<T: Into<Vector3<Float>>>(albedo: T) -> Self {
        Self {
            albedo: albedo.into(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<Float>)> {
        let direction = hit.normal + random_unit_sphere().normalize();
        Some((
            Ray {
                origin: hit.point,
                direction: if is_near_zero(&direction) { hit.normal } else { direction },
            },
            self.albedo,
        ))
    }
}

pub struct Metalic {
    albedo: Vector3<Float>,
}

impl Metalic {
    pub fn new<A: Into<Vector3<Float>>>(albedo: A) -> Self {
        Self {
            albedo: albedo.into(),
        }
    }
}

fn reflect(vec: Vector3<Float>, n: Vector3<Float>) -> Vector3<Float> {
    vec - 2.0 * cgmath::dot(vec, n) * n
}

impl Material for Metalic {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<Float>)> {
        let reflected = reflect(ray_in.direction, hit.normal);
        let ray = Ray {
            direction: reflected,
            origin: hit.point,
        };
        if cgmath::dot(ray.direction, hit.normal) > 0.0 { Some((ray, self.albedo)) }
        else { None }
    }
}


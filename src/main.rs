use material::{Lambertian, Metalic};
use rand::prelude::*;

use cgmath::{Vector3, Vector2, Point3, InnerSpace, vec3, EuclideanSpace, Zero, vec2};
use hittable::{Sphere, HittableList};
use image::{ RgbImage, ImageBuffer, Rgb };
use ray::Ray;
use types::Float;

mod types;
mod ray;
mod material;
mod hittable;

const WIDTH: u32 = 800; // / 2;
const HEIGHT: u32 = 600; // / 2;
const ASPECT: Float = WIDTH as Float / HEIGHT as Float;

const SAMPELS_PER_PIXEL: usize = 100;
const MAX_BOUNCES: usize = 50;

fn main() {
    // let img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    
    let white_base = Lambertian::new((1.0, 1.0, 1.0));
    let red_base = Lambertian::new((0.8, 0.2, 0.5));
    
    let metal = Metalic::new((1.0, 1.0, 1.0));
    let metal_gold = Metalic::new((0.8, 0.6, 0.2));
    
    let mut list = HittableList::new();
    list.add(Sphere::new((0.2, 0.0, -1.0), 0.5, &white_base));
    list.add(Sphere::new((-0.75, -0.25, -0.8), 0.25, &metal));
    list.add(Sphere::new((-0.35, -0.35, -0.7), 0.125, &metal_gold));
    list.add(Sphere::new((0.7, -0.5, -0.8), 0.25, &red_base));

    // ground
    list.add(Sphere::new((0.0, -100.5, -1.0), 100.0, &red_base));

    
    let mut rng = rand::rngs::SmallRng::from_entropy(); // rand::thread_rng();
    
    let img: RgbImage = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        let mut color = Vector3::zero();
        for _i in 0..SAMPELS_PER_PIXEL {
            let (x, y) = ((x as Float + rng.gen::<Float>()) / WIDTH as Float, (y as Float + rng.gen::<Float>()) / HEIGHT as Float);
            let mut uv = (Vector2::new(x, y)) * 2.0 - Vector2::new(1.0, 1.0);
            uv.x *= ASPECT;
            uv.y *= -1.0;

            let res = pixel_color(uv, &list);
            color += res;
        }
        to_color(color, SAMPELS_PER_PIXEL)
    });
        
    img.save("render.png").expect("couldnt save image");
}

fn to_color(color: Vector3<Float>, spp: usize) -> Rgb<u8> {
    let spp = spp as Float;
    Rgb([
        ((color.x / spp).sqrt() * 255.0).clamp(0.0, 255.0) as u8, 
        ((color.y / spp).sqrt() * 255.0).clamp(0.0, 255.0) as u8, 
        ((color.z / spp).sqrt() * 255.0).clamp(0.0, 255.0) as u8
    ])
}

fn pixel_color(uv: Vector2<Float>, list: &HittableList) -> Vector3<Float> {
    let ray = Ray {
        origin: Point3::new(0.0, 0.0, 0.0),
        direction: Vector3::new(uv.x, uv.y, -1.0).normalize(),
    };

    ray_color(ray, list, MAX_BOUNCES)
}

fn lerp<T: std::ops::Mul<Float, Output = T> + std::ops::Add<T, Output = T>>(a: T, b: T, v: Float) -> T { a * (1.0 - v) + b * v }

fn mult_components(a: Vector3<Float>, b: Vector3<Float>) -> Vector3<Float> {
    vec3(a.x * b.x, a.y * b.y, a.z * b.z)
}

fn ray_color(ray: Ray, list: &HittableList, depth: usize) -> Vector3<Float> {
    if depth <= 0 { return vec3(0.0, 0.0, 0.0) }

    if let Some(hit) = list.hit(&ray, 0.001..=100000.0) {
        return if let Some((ray, attenuation)) = hit.material.scatter(&ray, &hit) {
            mult_components(ray_color(ray, list, depth - 1), attenuation)
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }

    lerp(
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(0.5, 0.7, 1.0),
        ray.direction.y
    )
}


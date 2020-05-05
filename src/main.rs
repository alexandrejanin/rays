extern crate rand;

use std::rc::Rc;

use rand::Rng;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, Hittables, Sphere};
use crate::materials::{Glass, Lambertian, Metal};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

mod camera;
mod color;
mod hittable;
mod materials;
mod ray;
mod vec3;

fn ray_color<T: Hittable>(ray: Ray, world: &T, bounces: u32) -> Color {
    if bounces <= 0 {
        return Color::white();
    }

    if let Some(hit) = world.hit(ray, 0.00001, f64::INFINITY) {
        return if let Some(scatter) = hit.material.scatter(ray, &hit) {
            scatter.attenuation * ray_color(scatter.ray, world, bounces - 1)
        } else {
            Color::black()
        }
    }

    let unit_direction = ray.direction.normalized();
    let t = 0.5 + unit_direction.y / 2.0;
    (1.0 - t) * Color::white() + t * Color { r: 0.5, g: 0.7, b: 1.0 }
}

fn write_color(mut color: Color) {
    color.r = color.r.sqrt();
    color.g = color.g.sqrt();
    color.b = color.b.sqrt();
    println!("{}", color);
}

fn main() {
    let mut rng = rand::thread_rng();

    let image_width = 1000;
    let image_height = 1000;

    let samples_per_pixel = 100;
    let max_bounces = 100;

    let camera = Camera {
        origin: Point3::zero(),
        lower_left_corner: Point3::new(-1, -1, -1),
        horizontal: Vec3::new(2, 0, 0),
        vertical: Vec3::new(0, 2, 0),
    };

    let world = Hittables::from(vec![
        Box::new(Sphere::new(Point3::new(-1, 0, -1), 0.5, Rc::new(Metal::new(Color::grey(0.9))))),
        Box::new(Sphere::new(Point3::new(0, 0, -3), 0.5, Rc::new(Lambertian::new(Color::rgb(250, 80, 80))))),
        Box::new(Sphere::new(Point3::new(0, 0, -1), 0.5, Rc::new(Glass::white(1.5)))),
        Box::new(Sphere::new(Point3::new(0, 0, -1), -0.49, Rc::new(Glass::white(1.5)))),
        Box::new(Sphere::new(Point3::new(1, 0, -1), 0.5, Rc::new(Metal::fuzzy(Color::grey(0.9), 0.1)))),
        Box::new(Sphere::new(Point3::new(0, -100.5, -1), 100, Rc::new(Lambertian::new(Color::rgb(60, 80, 100))))),
    ]);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for y in (0..image_height).rev() {
        eprintln!("{} lines left", y);
        for x in 0..image_width {
            let mut color = Color::black();

            if samples_per_pixel > 1 {
                for _ in 0..samples_per_pixel {
                    let u = (x as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                    let v = (y as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                    let ray = camera.ray(u, v);
                    color += ray_color(ray, &world, max_bounces) / samples_per_pixel as f64;
                }
            } else {
                let u = x as f64 / (image_width - 1) as f64;
                let v = y as f64 / (image_height - 1) as f64;
                let ray = camera.ray(u, v);
                color = ray_color(ray, &world, max_bounces);
            }

            write_color(color);
        }
    }
}

use crate::{
    color, interval,
    objects::Scene,
    point3,
    structs::{Color, Interval, Point3, Ray, Vec3},
    vec3,
};
use rand::Rng;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::{io::Write, os::windows::thread};

pub struct Camera {
    width: u16,
    height: u16,
    first_pixel: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Point3,
}

/*for j in 0..self.width {
    let mut color = color!(0, 0, 0);
    for _ in 0..(Self::SAMPLES) {
        let ray = self.get_ray(i, j);
        color += Camera::ray_color(ray, scene, Self::MAX_BOUNCES);
    }

    self.write_color(writer, color);
} */

impl Camera {
    const SAMPLES: u16 = 10;
    const MAX_BOUNCES: u8 = 10;

    pub fn render(&self, writer: &mut dyn Write, scene: &Scene) {
        //let thread_pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();

        for i in 0..self.height {
            println!("{} out of {}", i, self.height);
            //thread_pool.install(op)
        }
    }

    fn get_ray(&self, i: u16, j: u16) -> Ray {
        let pixel_center =
            self.first_pixel + (self.pixel_delta_u * j as f64) + (self.pixel_delta_v * i as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen_range(0.0..1.0);
        let py = -0.5 + rng.gen_range(0.0..1.0);

        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }

    fn write_color(&self, writer: &mut dyn Write, color: Color) {
        let r = color.r();
        let g = color.g();
        let b = color.b();

        let scale = 1.0 / Self::SAMPLES as f64;
        let r = scale * r as f64;
        let g = scale * g as f64;
        let b = scale * b as f64;

        writeln!(writer, "{} {} {}", r, g, b).unwrap();
    }

    fn ray_color(ray: Ray, scene: &Scene, bounces: u8) -> Color {
        if bounces <= 0 {
            return color!(0, 0, 0);
        }
        match scene.does_hit(ray, interval!(0.1, f64::INFINITY)) {
            Some(hit) => {
                let (mut ray, albedo) = hit.material.scatter(hit.clone(), ray);

                if ray.direction().near_zero() {
                    ray = Ray::new(hit.point(), hit.normal());
                }

                let ray_color = Self::ray_color(ray, scene, bounces - 1);
                return Color::new(
                    (ray_color.r() as f64 * (albedo.r() as f64) / 255.0) as i32,
                    (ray_color.g() as f64 * (albedo.g() as f64) / 255.0) as i32,
                    (ray_color.b() as f64 * (albedo.b() as f64) / 255.0) as i32,
                );
            }
            None => linear_interpolation(
                (ray.direction().unit_vec().y() + 1.0) * 0.5,
                Color::WHITE,
                Color::BLUE,
            ),
        }
    }

    pub fn new(screen: (u16, u16), viewport: (f64, f64), focal_length: f64) -> Self {
        let camera_center = point3!(0, 0, 0);

        let viewport_u = vec3!(viewport.0, 0, 0);
        let viewport_v = vec3!(0, -viewport.1, 0);

        let pixel_delta_u = viewport_u / screen.0;
        let pixel_delta_v = viewport_v / screen.1;

        let viewport_top_left =
            camera_center - vec3!(0, 0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);

        let first_pixel = viewport_top_left + (pixel_delta_u + pixel_delta_v) * 0.5_f64;

        Camera {
            width: screen.0,
            height: screen.1,
            pixel_delta_u,
            pixel_delta_v,
            first_pixel,
            center: camera_center,
        }
    }
}

fn linear_interpolation(step: f64, start: Color, end: Color) -> Color {
    start * (1.0 - step) + end * step
}

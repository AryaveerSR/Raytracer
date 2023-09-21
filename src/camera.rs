use crate::{
    color, interval, point3,
    structs::{Color, Interval, Point3, Ray, Vec3},
    vec3, SCENE,
};
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{
    collections::HashMap,
    io::Write,
    sync::{mpsc, Arc},
};

pub struct Camera {
    width: u16,
    height: u16,
    first_pixel: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Point3,
}

impl Camera {
    const SAMPLES: u16 = 10;
    const MAX_BOUNCES: u8 = 10;

    pub fn render(&self, writer: &mut dyn Write) {
        let (sender, receiver) = mpsc::channel::<(u16, Arc<Vec<Color>>)>();

        (0..(self.height))
            .into_par_iter()
            .for_each_with(sender, |s, i| {
                let mut pixels = vec![];

                for j in 0..(self.width) {
                    let mut color = color!(0, 0, 0);

                    for _ in 0..(Self::SAMPLES) {
                        let ray = self.get_ray(i, j);
                        color += Camera::ray_color(ray, Self::MAX_BOUNCES);
                    }

                    let color = Self::average_samples(color);
                    pixels.push(color);
                }

                s.send((i, Arc::new(pixels))).unwrap();
            });

        let mut current_pending_row: u16 = 0;
        let mut row_hashes: HashMap<u16, Arc<Vec<Color>>> = HashMap::new(); //todo! is hashmap the best structure to use ?

        while current_pending_row < self.height {
            let val = receiver.try_recv(); //todo! better variables

            if val.is_ok() {
                let (i, row) = val.unwrap();

                if i == current_pending_row {
                    Self::write_row(row, writer);
                    current_pending_row += 1;
                } else {
                    row_hashes.insert(i, row);
                }
            } else {
                if row_hashes.contains_key(&current_pending_row) {
                    let row = row_hashes.remove(&current_pending_row).unwrap();
                    Self::write_row(row, writer);
                    current_pending_row += 1;
                }
            }
        }
    }

    fn write_row(row: Arc<Vec<Color>>, writer: &mut dyn Write) {
        for pixel in row.iter() {
            writeln!(writer, "{} {} {}", pixel.r(), pixel.g(), pixel.b()).unwrap();
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

    fn average_samples(color: Color) -> Color {
        let scale = 1.0 / Self::SAMPLES as f64;

        let r = scale * color.r() as f64;
        let g = scale * color.g() as f64;
        let b = scale * color.b() as f64;

        Color::new(r as i32, g as i32, b as i32)
    }

    fn ray_color(ray: Ray, bounces: u8) -> Color {
        if bounces <= 0 {
            return color!(0, 0, 0);
        }
        match SCENE.does_hit(ray, interval!(0.1, f64::INFINITY)) {
            Some(hit) => {
                let (mut ray, albedo) = hit.material.scatter(hit.clone(), ray);

                if ray.direction().near_zero() {
                    ray = Ray::new(hit.point(), hit.normal());
                }

                let ray_color = Self::ray_color(ray, bounces - 1);
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

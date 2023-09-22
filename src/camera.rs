//! Implementation of camera (rendering the scene).

use crate::{
    interval,
    structs::{Color, Interval, Point3, Ray, Vec3},
    HEIGHT, SCENE, WIDTH,
};
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{
    collections::HashMap,
    io::Write,
    sync::{mpsc, Arc},
};

//todo! all constants user defineable ?
const VERTICAL_FOV: f64 = 90.0; // In Degrees

const LOOK_FROM: Point3 = Point3::new_const(-2.0, 1.0, 1.0);
const LOOK_TO: Point3 = Point3::new_const(0.0, 0.0, -1.0);
const VUP: Vec3 = Vec3::new_const(0.0, 1.0, 0.0); // Camera relative "up" direction

/// Struct representing a camera.
pub struct Camera {
    first_pixel: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    /// How many surrounding pixels are to be sampled ?
    const SAMPLES: u16 = 20;
    /// How many times should a ray bounce ?
    const MAX_BOUNCES: u8 = 20;

    /// The main render function.
    pub fn render(&self, writer: &mut dyn Write) {
        // a channel to send and receive completed rows.
        let (sender, receiver) = mpsc::channel::<(u16, Arc<Vec<Color>>)>();

        println!("Starting Computing.");
        // for every row..
        (0..HEIGHT).into_par_iter().for_each_with(sender, |s, i| {
            //? println!("Computing row {}", i);

            let mut pixels = vec![];

            // compute the pixels
            for j in 0..WIDTH {
                let mut color = Color::BLACK;

                for _ in 0..(Self::SAMPLES) {
                    let ray = self.get_ray(i, j);
                    color += Camera::ray_color(ray, Self::MAX_BOUNCES);
                }

                let color = Self::average_samples(color);
                pixels.push(color);
            }

            // and send them.
            s.send((i, Arc::new(pixels))).unwrap();
        });

        // the current row we are waiting on.
        let mut current_pending_row: u16 = 0;
        // if the rows are out of order, they wait their turn in this hashmap.
        let mut row_hashes: HashMap<u16, Arc<Vec<Color>>> = HashMap::new();

        println!("Starting Writing");
        while current_pending_row < HEIGHT {
            //? println!("Waiting for row {}", current_pending_row);

            let received_row = receiver.try_recv();

            if received_row.is_ok() {
                let (i, row) = received_row.unwrap();

                // if the row is the one we need, just write it and carry on
                if i == current_pending_row {
                    Self::write_row(row, writer);
                    current_pending_row += 1;
                } else {
                    // or put it in the waiting list
                    row_hashes.insert(i, row);
                }
            } else {
                // check if the row we need is now on the hashmap
                // the else is triggered when there are no more rows to receive,
                // which is either when all are computed or one's taking a long time.
                if row_hashes.contains_key(&current_pending_row) {
                    let row = row_hashes.remove(&current_pending_row).unwrap();
                    Self::write_row(row, writer);
                    current_pending_row += 1;
                }
            }
        }
    }

    /// write that damn row
    fn write_row(row: Arc<Vec<Color>>, writer: &mut dyn Write) {
        for pixel in row.iter() {
            writeln!(writer, "{} {} {}", pixel.r(), pixel.g(), pixel.b()).unwrap();
        }
    }

    fn get_ray(&self, i: u16, j: u16) -> Ray {
        let pixel_center =
            self.first_pixel + (self.pixel_delta_u * j as f64) + (self.pixel_delta_v * i as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_direction = pixel_sample - LOOK_FROM;
        Ray::new(LOOK_FROM, ray_direction)
    }

    // generate a random sample square
    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();

        let px = -0.5 + rng.gen_range(0.0..1.0);
        let py = -0.5 + rng.gen_range(0.0..1.0);

        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }

    // divide the accumulated color by the no. of samples
    fn average_samples(color: Color) -> Color {
        let scale = 1.0 / Self::SAMPLES as f64;

        let r = scale * color.r() as f64;
        let g = scale * color.g() as f64;
        let b = scale * color.b() as f64;

        Color::new(r as i32, g as i32, b as i32)
    }

    fn ray_color(ray: Ray, bounces: u8) -> Color {
        if bounces <= 0 {
            return Color::BLACK;
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
            // a nice sky
            None => linear_interpolation(
                (ray.direction().unit_vec().y() + 1.0) * 0.5,
                Color::WHITE,
                Color::BLUE,
            ),
        }
    }

    pub fn new() -> Self {
        let focal_length = (LOOK_FROM - LOOK_TO).length();

        let theta = VERTICAL_FOV.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (WIDTH / HEIGHT) as f64;

        let w = (LOOK_FROM - LOOK_TO).unit_vec();
        let u = VUP.cross(w).unit_vec();
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = v * viewport_height;

        let pixel_delta_u = viewport_u / WIDTH;
        let pixel_delta_v = viewport_v / HEIGHT;

        let viewport_top_left =
            LOOK_FROM - (w * focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);

        let first_pixel = viewport_top_left + (pixel_delta_u + pixel_delta_v) * 0.5_f64;

        Camera {
            pixel_delta_u,
            pixel_delta_v,
            first_pixel,
        }
    }
}

fn linear_interpolation(step: f64, start: Color, end: Color) -> Color {
    start * (1.0 - step) + end * step
}

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

const LOOK_FROM: Point3 = Point3::new_const(0.0, 0.0, 1.0); // The camera's assumed center.
const LOOK_TO: Point3 = Point3::new_const(0.0, 0.0, 0.0); // The point the camera is looking at.
const VUP: Vec3 = Vec3::new_const(0.0, 1.0, 0.0); // What direction is up, in this case positive y-axis.

const MAX_BOUNCES: u8 = 10; // Max. no of bounces a ray can have before it just turns black.
const SAMPLES: u16 = 20; // Max. no of samples. More samples look better but are more compute-intensive.

/// Struct representing a camera.
/// See constructor for what the fields are.
pub struct Camera {
    first_pixel: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    /// The main render function that does frankly everything.
    pub fn render(&self, writer: &mut dyn Write) {
        // A receiver and (about to be cloned a lot) sender to send back the results of each compute.
        //todo! WGPU: using compute for this.
        let (sender, receiver) = mpsc::channel::<(u16, Arc<Vec<Color>>)>();

        println!("Starting computing.");

        // Loop through every row of the image.
        (0..HEIGHT).into_par_iter().for_each_with(sender, |s, i| {
            //? println!("Computing row {}", i);

            // The entire row stored as a vector of color.
            let mut pixels = vec![];

            // For every pixel..
            for j in 0..WIDTH {
                let mut color = Color::BLACK;

                // ..go through every sample ray
                for _ in 0..(SAMPLES) {
                    // get the color
                    let ray = self.get_ray(i, j);
                    // add it to the `color` variable
                    color += Camera::ray_color(ray, MAX_BOUNCES);
                }

                // and just average it over the number of samples.
                pixels.push(color / SAMPLES);
            }

            // and send them.
            s.send((i, Arc::new(pixels))).unwrap();
        });

        // The following code receives the completed rows from each thread.
        //todo! this was designed to run in a thread parallel to the computing threads themselves,

        // The row we are waiting on
        let mut current_pending_row: u16 = 0;

        // Since the rows will come in out-of-order (some will complete before others), store them here
        // if its not their turn.
        let mut row_hashes: HashMap<u16, Arc<Vec<Color>>> = HashMap::new();

        println!("Starting Writing");

        while current_pending_row < HEIGHT {
            //? println!("Waiting for row {}", current_pending_row);

            // Try receiving a row.
            let received_row = receiver.try_recv();

            if received_row.is_ok() {
                let (i, row) = received_row.unwrap();

                // Check if the received row is the one we need.
                if i == current_pending_row {
                    // Write it to the file
                    Self::write_row(row, writer);
                    current_pending_row += 1;
                } else {
                    // Put it in the waiting list
                    row_hashes.insert(i, row);
                }
            } else {
                // Check if the row we need is now on the hashmap
                if row_hashes.contains_key(&current_pending_row) {
                    // Remove the row from the hashmap, since its turn has arrived..
                    let row = row_hashes.remove(&current_pending_row).unwrap();

                    // ..and write it.
                    Self::write_row(row, writer);
                    current_pending_row += 1;
                }
            }
        }
    }

    //todo! A general method to call for the file, instead of just assuming its a PPM
    /// Function to write a vector of colors to the file.
    fn write_row(row: Arc<Vec<Color>>, writer: &mut dyn Write) {
        for pixel in row.iter() {
            writeln!(writer, "{} {} {}", pixel.r(), pixel.g(), pixel.b())
                .expect("Writing a row failed.");
        }
    }

    /// Function to get the ray corresponding to the particular pixel.
    ///
    /// Each time its called (which should be equal to the no. of samples),
    /// it will randomize by a bit (see `pixel_sample_square()`)
    fn get_ray(&self, i: u16, j: u16) -> Ray {
        let pixel_center =
            self.first_pixel + (self.pixel_delta_u * j as f64) + (self.pixel_delta_v * i as f64);

        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_direction = pixel_sample - LOOK_FROM;
        Ray::new(LOOK_FROM, ray_direction)
    }

    /// Generate a random offset for a pixel to sample randomly.
    /// The range ensures that the sampled pixel would be between the original pixel and its surrounding pixels.
    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();

        let px = rng.gen_range(-0.5..0.5);
        let py = rng.gen_range(-0.5..0.5);

        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }

    /// Function that takes a ray, checks for hits and returns the appropriate color to display.
    fn ray_color(ray: Ray, bounces: u8) -> Color {
        // If it bounces eternally (the bounce threshold), just return black.
        if bounces <= 0 {
            return Color::BLACK;
        }

        // Check if the scene has any object that is in the path of this ray.
        //
        // The interval starts at 0.1 to prevent shadow-acne (see below), where if a ray
        // just bounced off a surface, the same surface might appear to be in the path again.
        //
        // PS: https://stackoverflow.com/questions/36908835/what-causes-shadow-acne
        match SCENE.does_hit(ray, interval!(0.1, f64::INFINITY)) {
            // If the ray does hit, get the hit data.
            Some(hit) => {
                // Call the scatter function on the material of the surface just hit.
                let (mut ray, albedo) = hit.material.scatter(hit.clone(), ray);

                // If the ray has near-zero direction after scattering, just send it back the way it came.
                if ray.direction().near_zero() {
                    ray = Ray::new(hit.point(), hit.normal());
                }

                // Call itself recursively for this ray until either it bounces a certain no. of times
                // or it goes off into the 'sky' (the light source).
                //
                // For every bounce off a surface, multiply it with the (`albedo` / 255) of the material
                // and the color from the next bounce/sky.
                let ray_color = Self::ray_color(ray, bounces - 1);
                (albedo * ray_color) / 255
            }
            // If the ray doesn't hit anything, this draws a sky.
            None => {
                // Linear interpolation. (fancy speak for gradient)
                let step = (ray.direction().unit_vec().y() + 1.0) * 0.5;

                // It goes from `WHITE` to `BLUE`, resulting in a pretty neat sky.
                Color::WHITE * (1.0 - step) + Color::BLUE * step
            }
        }
    }

    /// Constructor for a camera.
    pub fn new() -> Self {
        // Calculate focal length.
        // This is possible as `LOOK_TO` is a point and not a direction (which it normally should be),
        // so we can use it to encode focal length.
        //
        // The alternative approach would be to define `LOOK_TO` as a vector to get the direction
        // relative to `LOOK_FROM`, and then have a constant for `FOCAL_LENGTH`.
        let focal_length = (LOOK_FROM - LOOK_TO).length();

        // Calculate the viewport dimensions using focal length and the field of view.
        let theta = VERTICAL_FOV.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (WIDTH / HEIGHT) as f64;

        // The basis unit vectors to describe the camera's orientation.
        //
        // `v` is the vector pointing to camera's upwards.
        // (positive y-axis from camera's frame of reference, if `LOOK_TO` lies on the negative z-axis)
        //
        // `u` is the vector pointing to camera's right.
        // (positive x-axis in the above analogy)
        //
        // `w` is the vector pointing away from the direction the camera is looking.
        // (positive z-axis in above analogy)
        let w = (LOOK_FROM - LOOK_TO).unit_vec();
        let u = VUP.cross(w).unit_vec();
        let v = w.cross(u);

        // Vectors with camera's up/right as direction and viewport height/width as magnitude.
        let viewport_u = u * viewport_width;
        let viewport_v = v * viewport_height;

        // Distance between each pixel vertically and horizontally, also in vector form.
        let pixel_delta_u = viewport_u / WIDTH;
        let pixel_delta_v = viewport_v / HEIGHT;

        // The starting postion for the viewport.
        let viewport_top_left =
            LOOK_FROM - (w * focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);

        // The position of the first pixel's center (considering them as points on a grid instead of little squares)
        let first_pixel = viewport_top_left + (pixel_delta_u + pixel_delta_v) * 0.5_f64;

        Camera {
            pixel_delta_u,
            pixel_delta_v,
            first_pixel,
        }
    }
}

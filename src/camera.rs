//! Implementation of camera (rendering the scene).

use crate::{
    file::FileWriter,
    interval,
    structs::{Color, Interval, Point3, Ray, Vec3},
    FIELD_OF_VIEW, FOV, HEIGHT, LOOK_FROM, LOOK_TO, MAX_BOUNCES, SAMPLES, SCENE, VUP, WIDTH,
};
use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{
    collections::HashMap,
    sync::{mpsc, Arc},
};

/// Struct representing a camera.
/// See constructor for what the fields are.
pub struct Camera {
    first_pixel: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    /// The main render function that does frankly everything.
    pub fn render(&self, file_writer: &mut dyn FileWriter) {
        // A receiver and (about to be cloned a lot) sender to send back the results of each compute.
        let (sender, receiver) = mpsc::channel::<(u16, Arc<Vec<Color>>)>();

        #[cfg(debug_assertions)]
        println!("Starting computing.");

        //todo! WGPU: using compute for this.

        // Loop through every row of the image.
        (0..*HEIGHT.get().expect("OnceCell not initialized."))
            .into_par_iter()
            .for_each_with(sender, |s, i| {
                //? #[cfg(debug_assertions)]
                //? println!("Computing row {}", i);

                // The entire row stored as a vector of color.
                let mut pixels = vec![];

                let mut rng = rand::thread_rng();

                // For every pixel..
                for j in 0..*WIDTH.get().expect("OnceCell not initialized.") {
                    let mut color = Color::BLACK;

                    let samples = *SAMPLES.get().expect("OnceCell not initialized.");

                    // ..go through every sample ray
                    for _ in 0..(samples) {
                        // get the color
                        let ray = self.get_ray(i, j, &mut rng);
                        // add it to the `color` variable
                        color += Camera::ray_color(
                            ray,
                            *MAX_BOUNCES.get().expect("OnceCell not initialized."),
                        );
                    }

                    // and just average it over the number of samples.
                    pixels.push(color / samples);
                }

                // and send them.
                s.send((i, Arc::new(pixels))).unwrap();
            });

        // The following code receives the completed rows from each thread and writes to file.

        // The row we are waiting on
        let mut current_pending_row: u16 = 0;

        // Since the rows will come in out-of-order (some will complete before others), store them here
        // if its not their turn.
        let mut row_hashes: HashMap<u16, Arc<Vec<Color>>> = HashMap::new();

        #[cfg(debug_assertions)]
        println!("Starting writing");

        while current_pending_row < *HEIGHT.get().expect("OnceCell not initialized.") {
            //? #[cfg(debug_assertions)]
            //? println!("Waiting for row {}", current_pending_row);

            match receiver.try_recv() {
                Ok((i, row)) => {
                    // Check if the received row is the one we need.
                    if i == current_pending_row {
                        // Write it to the file
                        Self::write_row(row, file_writer);
                        current_pending_row += 1;
                    } else {
                        // Put it in the waiting list
                        row_hashes.insert(i, row);
                    }
                }
                Err(_) => {
                    // Check if the row we need is now on the hashmap
                    if row_hashes.contains_key(&current_pending_row) {
                        // Remove the row from the hashmap, since its turn has arrived..
                        let row = row_hashes.remove(&current_pending_row).unwrap();

                        // ..and write it.
                        Self::write_row(row, file_writer);
                        current_pending_row += 1;
                    }
                }
            };
        }
    }

    //todo! A general method to call for the file, instead of just assuming its a PPM
    /// Function to write a vector of colors to the file.
    fn write_row(row: Arc<Vec<Color>>, file_writer: &mut dyn FileWriter) {
        for pixel in row.iter() {
            file_writer.write(*pixel);
        }
    }

    /// Function to get the ray corresponding to the particular pixel.
    ///
    /// Each time its called (which should be equal to the no. of samples),
    /// it will randomize by a bit (see `pixel_sample_square()`)
    fn get_ray(&self, i: u16, j: u16, rng: &mut ThreadRng) -> Ray {
        let look_from = *LOOK_FROM.get().expect("OnceCell not initialized.");

        let pixel_center =
            self.first_pixel + (self.pixel_delta_u * j as f64) + (self.pixel_delta_v * i as f64);

        let pixel_sample = pixel_center + self.pixel_sample_square(rng);

        let ray_direction = pixel_sample - look_from;
        Ray::new(look_from, ray_direction)
    }

    /// Generate a random offset for a pixel to sample randomly.
    /// The range ensures that the sampled pixel would be between the original pixel and its surrounding pixels.
    fn pixel_sample_square(&self, rng: &mut ThreadRng) -> Vec3 {
        let px = rng.gen_range(-0.5..0.5);
        let py = rng.gen_range(-0.5..0.5);

        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }

    /// Function that takes a ray, checks for hits and returns the appropriate color to display.
    fn ray_color(ray: Ray, bounces: u8) -> Color {
        // If it bounces eternally (the bounce threshold), just return black.
        if bounces == 0 {
            return Color::BLACK;
        }

        // Check if the scene has any object that is in the path of this ray.
        //
        // The interval starts at 0.1 to prevent shadow-acne (see below), where if a ray
        // just bounced off a surface, the same surface might appear to be in the path again.
        //
        // PS: https://stackoverflow.com/questions/36908835/what-causes-shadow-acne
        match SCENE
            .get()
            .expect("OnceCell initialization failed")
            .does_hit(ray, interval!(0.1, f64::INFINITY))
        {
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
        // Static variables used repeatedly in this function.
        let width = WIDTH.get().expect("OnceCell not initialized.");
        let height = HEIGHT.get().expect("OnceCell not initialized.");
        let look_from = *LOOK_FROM.get().expect("OnceCell not initialized.");
        let look_to = *LOOK_TO.get().expect("OnceCell not initialized.");
        let vup = *VUP.get().expect("OnceCell not initialized.");

        // Calculate focal length.
        // This is possible as `LOOK_TO` is a point and not a direction (which it normally should be),
        // so we can use it to encode focal length.
        //
        // The alternative approach would be to define `LOOK_TO` as a vector to get the direction
        // relative to `LOOK_FROM`, and then have a constant for `FOCAL_LENGTH`.
        let focal_length = (look_from - look_to).length();

        // Calculate the vertical field-of-view from the passed `FIELD_OF_VIEW` enum.
        // The enum can contain the vertical fov, which is just what we want,
        // or it can have the horizontal fov, which can be multiplied by the aspect ratio to get the
        // vertical fov.
        let vertical_fov = {
            match *FIELD_OF_VIEW.get().expect("OnceCell not initialized.") {
                FOV::Vertical(fov) => fov,
                FOV::Horizontal(fov) => fov * (*width as f64) / (*height as f64),
            }
        };

        // Calculate the viewport dimensions using focal length and the field of view.
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (width / height) as f64;

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
        let w = (look_from - look_to).unit_vec();
        let u = vup.cross(w).unit_vec();
        let v = w.cross(u);

        // Vectors with camera's up/right as direction and viewport height/width as magnitude.
        let viewport_u = u * viewport_width;
        let viewport_v = v * viewport_height;

        // Distance between each pixel vertically and horizontally, also in vector form.
        let pixel_delta_u = viewport_u / *width;
        let pixel_delta_v = viewport_v / *height;

        // The starting postion for the viewport.
        let viewport_lower_left =
            look_from - (w * focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);

        // The position of the first pixel's center (considering them as points on a grid instead of little squares)
        let first_pixel = viewport_lower_left + (pixel_delta_u + pixel_delta_v) * 0.5_f64;

        Camera {
            pixel_delta_u,
            pixel_delta_v,
            first_pixel,
        }
    }
}

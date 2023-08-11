pub mod color;
pub mod image;
pub mod ray;
pub mod vec3;

use color::Color;
use image::PPM;
use ray::Ray;
use vec3::{Point3, Vec3};

const WIDTH: u16 = 400;
const HEIGHT: u16 = 400;

const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT;

pub fn do_stuff() {
    // Camera
    let camera_center = Point3::new(0, 0, 0);

    let viewport_u = vec3!(VIEWPORT_WIDTH, 0, 0);
    let viewport_v = vec3!(0, -VIEWPORT_HEIGHT, 0);

    let pixel_delta_u = viewport_u / WIDTH as f64;
    let pixel_delta_v = viewport_v / HEIGHT as f64;

    let viewport_top_left =
        camera_center - vec3!(0, 0, FOCAL_LENGTH) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let first_pixel = viewport_top_left + (pixel_delta_u + pixel_delta_v) * 0.5_f64;

    let mut data = vec![vec![Color::BLACK; WIDTH as usize]; HEIGHT as usize];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let pixel_center =
                first_pixel + (pixel_delta_u * j as f64) + (pixel_delta_v * i as f64);
            let ray_direction = pixel_center - camera_center;

            let color = ray_color(Ray::new(camera_center, ray_direction));
            data[i as usize][j as usize] = color;
        }
    }

    PPM::new(WIDTH, HEIGHT, data).write_to_file("output.ppm".to_string());
}

fn ray_color(ray: Ray) -> Color {
    let t = hits_sphere(point3!(0, 0, -1), 0.5, &ray);
    if t >= 0.0 {
        let normal = (ray.at(t) - vec3!(0, 0, -1)).unit_vec();
        Color::from_rgb(
            ((normal.x() + 1.0) * 128.0) as u8,
            ((normal.y() + 1.0) * 128.0) as u8,
            ((normal.z() + 1.0) * 128.0) as u8,
        ) * 0.5_f64 // TODO: 6.2 section
    } else {
        linear_interpolation(
            (ray.direction().unit_vec().y() + 1.0) * 0.5,
            Color::WHITE,
            Color::BLUE,
        )
    }
}

fn linear_interpolation(step: f64, start: Color, end: Color) -> Color {
    start * (1.0 - step) + end * step
}

fn hits_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let distance = ray.origin() - center;
    let a_coeff = ray.direction().dot(ray.direction());
    let b_coeff = distance.dot(ray.direction()) * 2.0;
    let c_coeff = distance.dot(distance) - radius.powi(2);
    let discriminant = b_coeff.powi(2) - a_coeff * c_coeff * 4.0;

    if discriminant >= 0.0 {
        (-b_coeff - discriminant.sqrt()) / (a_coeff * 2.0)
    } else {
        return -1.0;
    }
}

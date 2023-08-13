pub mod file;
pub mod object;
pub mod structs;

use file::{FileWriter, PPMFile};
use object::{Scene, Sphere};
use structs::{Color, Interval, Point3, Ray, Vec3};

const WIDTH: u16 = 400;
const HEIGHT: u16 = 400;

const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT;

pub fn run() {
    // Camera
    let camera_center = Point3::new(0, 0, 0);

    let viewport_u = vec3!(VIEWPORT_WIDTH, 0, 0);
    let viewport_v = vec3!(0, -VIEWPORT_HEIGHT, 0);

    let mut scene = Scene::new();
    scene.add(Box::new(Sphere::new(point3!(0, 0, -1), 0.5)));
    scene.add(Box::new(Sphere::new(point3!(0, -100.5, -1), 100)));

    let pixel_delta_u = viewport_u / WIDTH as f64;
    let pixel_delta_v = viewport_v / HEIGHT as f64;

    let viewport_top_left =
        camera_center - vec3!(0, 0, FOCAL_LENGTH) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let first_pixel = viewport_top_left + (pixel_delta_u + pixel_delta_v) * 0.5_f64;

    let mut file = PPMFile::new("output.ppm", WIDTH, HEIGHT);
    let file_writer = file.writer();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let pixel_center =
                first_pixel + (pixel_delta_u * j as f64) + (pixel_delta_v * i as f64);
            let ray_direction = pixel_center - camera_center;

            let color = ray_color(Ray::new(camera_center, ray_direction), &scene);

            writeln!(file_writer, "{}", color.as_string()).unwrap();
        }
    }
}

fn ray_color(ray: Ray, world: &Scene) -> Color {
    match world.does_hit(ray, Interval::new(0, f64::INFINITY)) {
        Some(hit) => {
            let normal = hit.normal();
            return Color::new(
                ((normal.x() + 1.0) * 128.0) as u8,
                ((normal.y() + 1.0) * 128.0) as u8,
                ((normal.z() + 1.0) * 128.0) as u8,
            ) * 0.5_f64;
        }
        None => {
            return linear_interpolation(
                (ray.direction().unit_vec().y() + 1.0) * 0.5,
                Color::WHITE,
                Color::BLUE,
            );
        }
    }
}

fn linear_interpolation(step: f64, start: Color, end: Color) -> Color {
    start * (1.0 - step) + end * step
}

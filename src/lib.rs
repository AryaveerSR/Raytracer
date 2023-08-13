pub mod camera;
pub mod file;
pub mod object;
pub mod structs;

use camera::Camera;
use file::{FileWriter, PPMFile};
use object::{Scene, Sphere};
use structs::Point3;

const WIDTH: u16 = 400;
const HEIGHT: u16 = 400;

const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT;

pub fn run() {
    let mut scene = Scene::new();
    scene.add(Box::new(Sphere::new(point3!(0, 0, -1), 0.5)));
    scene.add(Box::new(Sphere::new(point3!(0, -100.5, -1), 100)));

    let camera = Camera::new(
        (WIDTH, HEIGHT),
        (VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
        FOCAL_LENGTH,
    );

    let mut file = PPMFile::new("output.ppm", WIDTH, HEIGHT);
    let writer = file.writer();

    camera.render(writer, &scene);
}

pub mod camera;
pub mod file;
pub mod materials;
pub mod objects;
pub mod structs;

use camera::Camera;
use file::{FileWriter, PPMFile};
use materials::{Lambertian, Metal};
use objects::{Scene, Sphere};
use once_cell::sync::Lazy;
use std::sync::Arc;
use structs::Point3;

const WIDTH: u16 = 800;
const HEIGHT: u16 = 400;

const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * 2.0;

pub static SCENE: Lazy<Scene> = Lazy::new(|| {
    let mut scene = Scene::new();

    // scene
    scene.add(Box::new(Sphere::new(
        point3!(0, -100.5, -1),
        100,
        Arc::new(Lambertian::new(color!(205, 205, 0))),
    )));
    scene.add(Box::new(Sphere::new(
        point3!(0, 0, -1),
        0.5,
        Arc::new(Lambertian::new(color!(180, 77, 77))),
    )));
    scene.add(Box::new(Sphere::new(
        point3!(-1.0, 0, -1),
        0.5,
        Arc::new(Metal::new(color!(205, 205, 205), 0.3)),
    )));

    scene
});

pub fn run() {
    // init camera
    let camera = Camera::new(
        (WIDTH, HEIGHT),
        (VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
        FOCAL_LENGTH,
    );

    let mut file = PPMFile::new("output.ppm", WIDTH, HEIGHT);
    let writer = file.writer();

    camera.render(writer);
}

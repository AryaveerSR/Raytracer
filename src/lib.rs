pub mod camera;
pub mod file;
pub mod materials;
pub mod objects;
pub mod structs;

use camera::Camera;
use file::{FileWriter, PPMFile};
use materials::{Dielectric, Lambertian, Metal};
use objects::{Scene, Sphere};
use once_cell::sync::Lazy;
use std::sync::Arc;
use structs::Point3;

const WIDTH: u16 = 800;
const HEIGHT: u16 = 400;

//todo! load from file ??
/// A static object containing the scene that is to be rendered.
pub static SCENE: Lazy<Scene> = Lazy::new(|| {
    let mut scene = Scene::new();

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
        point3!(-1, 0, -1),
        0.5,
        Arc::new(Dielectric::new(1.5)),
    )));
    scene.add(Box::new(Sphere::new(
        point3!(1, 0, -1),
        0.5,
        Arc::new(Metal::new(color!(204, 204, 204), 0.1)),
    )));

    scene
});

pub fn run() {
    // init camera
    let camera = Camera::new();

    // init output file
    let mut file = PPMFile::new("output.ppm", WIDTH, HEIGHT);
    // get the writer (to be used with `write!` macro)
    let writer = file.writer();

    // render ahoy!
    camera.render(writer);
}

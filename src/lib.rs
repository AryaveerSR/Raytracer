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

//? A really good but compute-heavy scene.
/* pub static SCENE: Lazy<Scene> = Lazy::new(|| {
    let mut scene = Scene::new();

    scene.add(Box::new(Sphere::new(
        point3!(0, -1000, 0),
        1000,
        Arc::new(Lambertian::new(color!(130, 130, 130))),
    )));

    for i in -8..=8 {
        for j in -8..=8 {
            let mut rng = rand::thread_rng();

            let range = 0.0..=1.0;

            let center = point3!(
                i as f64 + 0.9 * rng.gen_range(range.clone()),
                0.2,
                j as f64 + 0.9 * rng.gen_range(range.clone())
            );

            if (center - point3!(4, 0.2, 0)).length() > 0.9 {
                let material: Arc<dyn Material + Send + Sync + 'static> = match rng.gen_range(0..3)
                {
                    0 => Arc::new(Lambertian::new(Color::random())),
                    1 => Arc::new(Metal::new(Color::random(), rng.gen_range(range.clone()))),
                    2 => Arc::new(Dielectric::new(rng.gen_range(1.1..=2.1))),
                    _ => unreachable!(),
                };

                scene.add(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    scene.add(Box::new(Sphere::new(
        point3!(-4, 1, 0),
        1.0,
        Arc::new(Lambertian::new(color!(180, 77, 77))),
    )));
    scene.add(Box::new(Sphere::new(
        point3!(0, 1, 0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    scene.add(Box::new(Sphere::new(
        point3!(4, 1, 0),
        1.0,
        Arc::new(Metal::new(color!(178, 153, 127), 0.0)),
    )));

    scene
}); */

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

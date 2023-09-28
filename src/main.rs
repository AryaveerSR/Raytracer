use raytracing::{
    self, color,
    file::{FileWriter, PPMFile},
    materials::{Dielectric, Lambertian, Metal},
    objects::{Scene, Sphere},
    point3, vec3, Options, FOV,
};
use std::{sync::Arc, time::Instant};

fn main() {
    let start_time = Instant::now();

    let scene = {
        let mut scene = Scene::new();

        //todo! macros to make this easier ??
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
    };

    let opts = Options {
        scene: scene,
        width: 800,
        height: 400,
        fov: FOV::Vertical(50.0),
        look_from: point3!(0.0, 0.0, 1.0),
        look_to: point3!(0.0, 0.0, 0.0),
        vup: vec3!(0.0, 1.0, 0.0),
        max_bounces: 20,
        samples: 20,
    };

    let mut file = PPMFile::new("output.ppm", 800, 400);
    let writer = file.writer();

    println!("Starting raytracer... ");

    raytracing::run(opts, writer);

    println!("Finished in {}s", start_time.elapsed().as_secs());
}

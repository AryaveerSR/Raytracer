use raytracing::{
    self, color,
    file::PPMFile,
    materials::{Dielectric, Lambertian, Metal},
    objects::Sphere,
    point3,
    structs::Scene,
    vec3, Options, FOV,
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
            vec3!(0, 0, 0),
        )));
        scene.add(Box::new(Sphere::new(
            point3!(0, 0, -1),
            0.5,
            Arc::new(Lambertian::new(color!(180, 77, 77))),
            vec3!(0, 3, 0),
        )));
        scene.add(Box::new(Sphere::new(
            point3!(-1, 0, -1),
            0.5,
            Arc::new(Dielectric::new(1.5)),
            vec3!(0, 0, 0),
        )));
        scene.add(Box::new(Sphere::new(
            point3!(1, 0, -1),
            0.5,
            Arc::new(Metal::new(color!(204, 204, 204), 0.1)),
            vec3!(0, 0, 0),
        )));

        scene
    };

    let opts = Options {
        scene,
        width: 800,
        height: 400,
        fov: FOV::Vertical(50.0),
        look_from: point3!(0.0, 0.0, 1.0),
        look_to: point3!(0.0, 0.0, 0.0),
        vup: vec3!(0.0, 1.0, 0.0),
        max_bounces: 20,
        samples: 20,
        shutter_open_duration: 1.0 / 24.0, // 24 FPS
    };

    let mut file = PPMFile::new("output.ppm", 800, 400);

    println!("Starting raytracer... ");

    raytracing::run(opts, &mut file);

    println!("Finished in {}s", start_time.elapsed().as_secs());
}

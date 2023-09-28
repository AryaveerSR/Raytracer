use criterion::{criterion_group, criterion_main, Criterion};
use raytracing::{
    self, color,
    materials::{Dielectric, Lambertian, Metal},
    objects::{Scene, Sphere},
    point3, vec3, Options, FOV,
};
use std::io::Write;
use std::sync::Arc;

struct DummyWriter {}

impl Write for DummyWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn render() {
    // ^ why is this here ? IDK
    let mut writer = DummyWriter {};
    let scene = {
        let mut scene = Scene::new();

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
            vec3!(0, 0, 0),
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
        scene: scene,
        width: 800,
        height: 400,
        fov: FOV::Vertical(50.0),
        look_from: point3!(0.0, 0.0, 1.0),
        look_to: point3!(0.0, 0.0, 0.0),
        vup: vec3!(0.0, 1.0, 0.0),
        max_bounces: 20,
        samples: 20,
        shutter_open_duration: 1.0 / 24.0,
    };

    raytracing::run(opts, &mut writer);
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("render spheres");
    group.sample_size(10);
    group.bench_function("render fn", |b| b.iter(|| render()));
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

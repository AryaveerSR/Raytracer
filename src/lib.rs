pub mod camera;
pub mod file;
pub mod materials;
pub mod objects;
pub mod structs;

use camera::Camera;
use file::{FileWriter, PPMFile};
use objects::Scene;
use once_cell::sync::OnceCell;
use structs::{Point3, Vec3};

/// A struct for the caller to pass all user-defined arguments.
pub struct Options {
    pub scene: Scene,
    pub width: u16,
    pub height: u16,
    pub fov: FOV,
    pub look_from: Point3,
    pub look_to: Point3,
    pub vup: Vec3,
    pub max_bounces: u8,
    pub samples: u16,
}

/// An enum for passing field-of-view in degrees in any axis we want.
/// The other axis would be calculated based on the aspect ratio, which in turn
/// is calculated from `WIDTH` and `HEIGHT`.
#[derive(Debug)]
pub enum FOV {
    Vertical(f64),
    Horizontal(f64),
}

// These constants are loaded in `run()` and used throughout the program.
// OnceCell allows us to let the caller to `run()` pass the variables,
// but still treat them as constants (makes multithreading simpler),
// and allows us to do benchmarks.

// Width and height of the rendered image.
// The dimensions of the viewport are calculated using field-of-view data.
static WIDTH: OnceCell<u16> = OnceCell::new();
static HEIGHT: OnceCell<u16> = OnceCell::new();

/// The scene itself.
static SCENE: OnceCell<Scene> = OnceCell::new();

/// Vertical or horizontal field of view in degrees.
static FIELD_OF_VIEW: OnceCell<FOV> = OnceCell::new();
/// Max. no of bounces a ray can have before it just turns black.
static MAX_BOUNCES: OnceCell<u8> = OnceCell::new();
/// Max. no of samples. More samples give a more "smooth" look but are more compute-intensive.
static SAMPLES: OnceCell<u16> = OnceCell::new();

/// The camera's assumed center.
static LOOK_FROM: OnceCell<Point3> = OnceCell::new();
/// The point the camera is looking at.
static LOOK_TO: OnceCell<Point3> = OnceCell::new();
/// What direction is up, usually case positive y-axis.
static VUP: OnceCell<Vec3> = OnceCell::new();

//? A really good but compute-heavy scene.
//todo! examples/ with this scene
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

pub fn run(opts: Options) {
    // Initialize the OnceCell statics.
    // These shouldn't fail (hopefully).
    WIDTH.set(opts.width).unwrap();
    HEIGHT.set(opts.height).unwrap();
    FIELD_OF_VIEW.set(opts.fov).unwrap();
    MAX_BOUNCES.set(opts.max_bounces).unwrap();
    SAMPLES.set(opts.samples).unwrap();
    LOOK_FROM.set(opts.look_from).unwrap();
    LOOK_TO.set(opts.look_to).unwrap();
    VUP.set(opts.vup).unwrap();
    SCENE.set(opts.scene).unwrap();

    // Init camera
    let camera = Camera::new();

    // Init output file
    let mut file = PPMFile::new("output.ppm");
    // Get the writer (to be used with `write!` macro)
    let writer = file.writer();

    // Render ahoy!
    camera.render(writer);
}

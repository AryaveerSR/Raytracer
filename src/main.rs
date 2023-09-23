use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    println!("Starting raytracer... ");

    raytracing::run();

    println!("Finished in {}s", start_time.elapsed().as_secs());
}

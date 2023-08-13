use std::time::Instant;

fn main() {
    let t = Instant::now();
    println!("Rendering... ");
    raytracing::run();
    println!("Finished in {}s", t.elapsed().as_secs());
}

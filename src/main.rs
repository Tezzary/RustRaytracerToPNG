mod raytracer;
mod image;
fn main() {
    
    let width = 400;
    let height = 200;

    let bounces = 10;
    let samples_per_pixel = 256;

    let threads = 7;

    let spheres = raytracer::generate_scene();
    let camera = raytracer::new_camera(width, height, vec![0.0, 0.0, -50.0]);

    raytracer::raytrace_image(width, height, &camera, &spheres, samples_per_pixel, bounces, threads);
}

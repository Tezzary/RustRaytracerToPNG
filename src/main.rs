mod image;
mod raytracer;
use std::thread;
use std::sync::mpsc;
use std::time::SystemTime;

#[derive(Clone)]

struct Cube {
    pixels: Vec<Vec<u8>>,
    x_index: u32,
    y_index: u32,
}

fn ray_trace_cube(x_index: u32, y_index: u32, width: u32, height: u32, camera: &raytracer::Camera, spheres: &Vec<raytracer::Sphere>, samples_per_pixel: u32, bounces: u32) -> Cube{
    let mut pixels: Vec<Vec<u8>> = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let y_index = height * y_index + y;
            let x_index = width * x_index + x;
            let color = raytracer::ray_trace_pixel(x_index, y_index, camera, spheres, samples_per_pixel, bounces);
            pixels.push(color);
        }
    }
    Cube {
        pixels: pixels,
        x_index: x_index,
        y_index: y_index,
    }
            
}


fn main() {
    let spheres = raytracer::generate_scene();
    let width = 1920;
    let height = 1080;
    let cube_width = 10;
    let cube_height = 10;
    let x_cubes = width / cube_width;
    let y_cubes = height / cube_height;
    
    let threads = 20;
    let bounces = 6;
    let samples_per_pixel = 512;
    let mut img = image::Image::blank(width, height);
    let camera = raytracer::new_camera(width, height, vec![0.0, 0.0, -50.0]);

    //let cube = ray_trace_cube(240, 0, 1, 1, &camera, &spheres, samples_per_pixel, bounces);    

    let (tx, rx) = mpsc::channel();

    for i in 0..threads {
        
        let camera = camera.clone();
        let spheres = spheres.clone();
        let tx = tx.clone();
        
        thread::spawn(move || {
            let camera = camera.clone();
            let spheres = spheres.clone();
            for j in 0..((x_cubes * y_cubes) / threads + 1) {
                let cube_number = (j * threads) + i;
                if cube_number >= x_cubes * y_cubes {
                    break;
                }
                let x_index = cube_number % x_cubes;
                let y_index = cube_number / x_cubes;

                let tx = tx.clone();

                let cube = ray_trace_cube(x_index, y_index, cube_width, cube_height, &camera, &spheres, samples_per_pixel, bounces);

                tx.send(cube).unwrap();
            }


        });
    }
    let mut count = 0;
    let start_time = SystemTime::now();
    for cube in rx {
        for y in 0..cube_height {
            for x in 0..cube_width {
                
                img.write_to_pixel(x + cube.x_index * cube_width, height - (y + cube.y_index * cube_height) - 1, [cube.pixels[(y * cube_width + x) as usize][0], cube.pixels[(y * cube_width + x) as usize][1], cube.pixels[(y * cube_width + x) as usize][2], 255]);
            }
        }
        count += 1;
        if count % (threads * 1) == 0 {
            let current_time = SystemTime::now();
            let elapsed = current_time.duration_since(start_time).unwrap();
            let eta = elapsed.as_secs() as f64 * (x_cubes * y_cubes) as f64 / count as f64 + (start_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as f64);
            //clear terminal
            print!("\x1B[2J\x1B[1;1H");
            println!("{}% done, {} seconds elapsed, {} seconds remaining", (count as f64 / (x_cubes * y_cubes) as f64 * 100.0).round() as u64, elapsed.as_secs(), (eta - (current_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()) as f64).round() as u64);
            image::Image::save_to_file(&mut img, "test12.png");
        }
        if count == x_cubes * y_cubes {
            image::Image::save_to_file(&mut img, "test12.png");
            break;
        }
    }
}

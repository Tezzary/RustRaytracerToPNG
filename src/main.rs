mod image;
mod raytracer;
mod objs;
use rand::prelude::*;
use std::thread;
use std::sync::mpsc;
#[derive(Clone)]

struct Cube {
    pixels: Vec<Vec<u8>>,
    x_index: u32,
    y_index: u32,
}



fn generate_scene() -> Vec<Sphere> {
    let colors: Vec<Vec<f64>> = vec![
        vec![1.0, 0.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 1.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 0.0, 1.0]
    ];
    let mut spheres = Vec::new();
    for i in 0..5 {
        spheres.push(Sphere {
            center: vec![i as f64 * 18.0 - 36.0, 0.0, 0.0],
            radius: 8.0,
            color: colors[i].clone(),
            light: 0.0,
            smoothness: 0.0,
        });
    }
    spheres[2].smoothness = 0.5;
    spheres.push(Sphere {
        center: vec![0.0, -1008.0, 0.0],
        radius: 1000.0,
        color: vec![0.5, 0.5, 0.0],
        light: 0.0,
        smoothness: 0.0,
    });
    spheres.push(Sphere {
        center: vec![30.0, 0.0, 100.0],
        radius: 50.0,
        color: vec![1.0, 1.0, 1.0],
        light: 5.0,
        smoothness: 0.0,
    }); 
    spheres.push(Sphere {
        center: vec![-30.0, 10.0, -10.0],
        radius: 5.0,
        color: vec![1.0, 1.0, 1.0],
        light: 5.0,
        smoothness: 0.0,
    });
    spheres.push(Sphere {
        center: vec![0.0, -12.0, -15.0],
        radius: 5.0,
        color: vec![1.0, 1.0, 1.0],
        light: 5.0,
        smoothness: 0.0,
    });
    return spheres;
}

fn ray_trace_cube(x_index: u32, y_index: u32, width: u32, height: u32, camera: Camera, spheres: Vec<Sphere>, samples_per_pixel: u32, bounces: u32) -> Cube{
    let mut pixels: Vec<Vec<u8>> = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let y_index = height * y_index + y;
            let x_index = width * x_index + x;
            let color = raytracer::ray_trace_pixel(x, y);
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
    let spheres = generate_scene();
    let width = 400;
    let height = 200;
    let cube_width = 10;
    let cube_height = 10;
    let x_cubes = width / cube_width;
    let y_cubes = height / cube_height;
    
    let threads = 20;
    let bounces = 10;
    let samples_per_pixel = 500;
    let mut img = image::Image::blank(width, height);
    let camera = Camera {
        origin: vec![0.0, 0.0, -50.0],
        yaw: 0.0,
        pitch: 0.0,
        fov: 90.0 * std::f64::consts::PI / 180.0,
        width: width,
        height: height,
    };

    let cube = ray_trace_cube(240, 0, 1, 1, camera.clone(), spheres.clone(), samples_per_pixel, bounces);

    print!("{} {} {}", cube.pixels[0][0], cube.pixels[0][1], cube.pixels[0][2]);
    

    let (tx, rx) = mpsc::channel();

    for i in 0..threads {
        
        let camera = camera.clone();
        let spheres = spheres.clone();
        let tx = tx.clone();
        
        thread::spawn(move || {
            for j in 0..((x_cubes * y_cubes) / threads + 1) {
                let cube_number = (j * threads) + i;
                if cube_number >= x_cubes * y_cubes {
                    break;
                }
                let x_index = cube_number % x_cubes;
                let y_index = cube_number / x_cubes;

                let camera = camera.clone();
                let spheres = spheres.clone();
                let tx = tx.clone();

                let cube = ray_trace_cube(x_index, y_index, cube_width, cube_height, camera, spheres, samples_per_pixel, bounces);

                tx.send(cube).unwrap();
            }


        });
    }
    let mut count = 0;
    for cube in rx {
        for y in 0..cube_height {
            for x in 0..cube_width {
                
                img.write_to_pixel(x + cube.x_index * cube_width, height - (y + cube.y_index * cube_height) - 1, [cube.pixels[(y * cube_width + x) as usize][0], cube.pixels[(y * cube_width + x) as usize][1], cube.pixels[(y * cube_width + x) as usize][2], 255]);
            }
        }
        count += 1;
        if count % (threads * 1) == 0 {
            image::Image::save_to_file(&mut img, "test9.png");
        }
        if count == x_cubes * y_cubes {
            image::Image::save_to_file(&mut img, "test9.png");
            break;
        }
    }
}

mod image;
use rand::prelude::*;
use std::thread;
use std::sync::mpsc;
#[derive(Clone)]
struct Sphere {
    center: Vec<f64>,
    radius: f64,
    color: Vec<f64>,
    light: f64,
}
#[derive(Clone)]
struct Camera {
    origin: Vec<f64>,
    yaw: f64,
    pitch: f64,
    fov: f64,
    width: u32,
    height: u32,
}
struct Ray {
    origin: Vec<f64>,
    direction: Vec<f64>,
}

struct Hit {
    distance: f64,
    point: Vec<f64>,
    normal: Vec<f64>,
    sphere: Sphere,
}

struct Cube {
    pixels: Vec<Vec<u8>>,
    x_index: u32,
    y_index: u32,
}

fn get_rand() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
fn gaussian_rand() -> f64 {
    let num1 = get_rand();
    let num2 = get_rand();
    (-2.0 * num1.ln()).sqrt() * (2.0 * std::f64::consts::PI * num2).cos()
}
fn get_random_bounce(normal: &Vec<f64>) -> Vec<f64> {
    let mut bounce = vec![
        gaussian_rand(),
        gaussian_rand(),
        gaussian_rand()
    ];
    let size = (bounce[0] * bounce[0] + bounce[1] * bounce[1] + bounce[2] * bounce[2]).sqrt();
    bounce[0] /= size;
    bounce[1] /= size;
    bounce[2] /= size;
    let dot_product = bounce[0] * normal[0] + bounce[1] * normal[1] + bounce[2] * normal[2];
    if dot_product < 0.0 {
        bounce[0] *= -1.0;
        bounce[1] *= -1.0;
        bounce[2] *= -1.0;
    }
    bounce
}
impl Ray {
    fn reset_direction(&mut self, camera: &Camera, x: u32, y: u32) {
        let f_x = x as f64;
        let f_y = y as f64;
        self.origin = camera.origin.clone();
        self.direction = vec![
            f64::sin(camera.yaw + ((f_x + 0.5) / camera.width as f64) * camera.fov - camera.fov / 2.0),
            f64::sin(camera.pitch + ((f_y + 0.5) / camera.height as f64) * camera.fov * camera.height as f64 / camera.width as f64 - camera.fov * camera.height as f64 / camera.width as f64 / 2.0),
            f64::cos(camera.yaw + ((f_x + 0.5) / camera.width as f64) * camera.fov - camera.fov / 2.0),
        ];
    }
    fn get_collision(&mut self, sphere: &Sphere) -> Hit {
        let delta_position = vec![
            sphere.center[0] - self.origin[0],
            sphere.center[1] - self.origin[1],
            sphere.center[2] - self.origin[2],
        ];

        let dot_product = delta_position[0] * self.direction[0] + delta_position[1] * self.direction[1] + delta_position[2] * self.direction[2];
        if dot_product < 0.0 {
            return Hit {
                distance: -1.0,
                point: vec![0.0, 0.0, 0.0],
                normal: vec![0.0, 0.0, 0.0],
                sphere: sphere.clone(),
            };
        }
        let centre_point = vec![
            self.origin[0] + self.direction[0] * dot_product,
            self.origin[1] + self.direction[1] * dot_product,
            self.origin[2] + self.direction[2] * dot_product,
        ];
        let distance = ((centre_point[0] - sphere.center[0]).powf(2.0) + (centre_point[1] - sphere.center[1]).powf(2.0) + (centre_point[2] - sphere.center[2]).powf(2.0)).sqrt();

        if distance > sphere.radius {
            return Hit {
                distance: -1.0,
                point: vec![0.0, 0.0, 0.0],
                normal: vec![0.0, 0.0, 0.0],
                sphere: sphere.clone(),
            };
        }
        let offset = ((sphere.radius.powf(2.0) - distance.powf(2.0)).abs()).sqrt();
        let point = vec![
            centre_point[0] - self.direction[0] * offset,
            centre_point[1] - self.direction[1] * offset,
            centre_point[2] - self.direction[2] * offset,
        ];
        let normal = vec![
            (point[0] - sphere.center[0]) / sphere.radius,
            (point[1] - sphere.center[1]) / sphere.radius,
            (point[2] - sphere.center[2]) / sphere.radius,
        ];
        return Hit {
            distance: dot_product - offset,
            point: point,
            normal: normal,
            sphere: sphere.clone(),
        };

    }
}



fn generate_scene() -> Vec<Sphere> {
    let colors: Vec<Vec<f64>> = vec![
        vec![1.0, 0.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
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
        });
    }
    spheres.push(Sphere {
        center: vec![0.0, -1008.0, 10.0],
        radius: 1000.0,
        color: vec![0.5, 0.5, 0.0],
        light: 0.0,
    });
    spheres.push(Sphere {
        center: vec![30.0, 0.0, 100.0],
        radius: 50.0,
        color: vec![1.0, 1.0, 1.0],
        light: 5.0,
    }); 
    spheres.push(Sphere {
        center: vec![-30.0, 10.0, -10.0],
        radius: 5.0,
        color: vec![1.0, 1.0, 1.0],
        light: 5.0,
    });
    spheres.push(Sphere {
        center: vec![0.0, -8.0, -10.0],
        radius: 3.5,
        color: vec![1.0, 1.0, 1.0],
        light: 5.0,
    });
    return spheres;
}

fn ray_trace_cube(x_index: u32, y_index: u32, width: u32, height: u32, camera: Camera, spheres: Vec<Sphere>, samples_per_pixel: u32, bounces: u32) -> Cube{
    let mut ray = Ray {
        origin: camera.origin.clone(),
        direction: vec![0.0, 0.0, 0.0],
    };

    let mut pixels: Vec<Vec<u8>> = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let y_index = height * y_index + y;
            let x_index = width * x_index + x;
            let mut total_light = vec![0.0, 0.0, 0.0];
            let mut hit = false;
            for i in 0..samples_per_pixel {
                ray.reset_direction(&camera, x_index, y_index);
                let mut ray_color = vec![1.0, 1.0, 1.0];
                let mut accumulated_light = vec![0.0, 0.0, 0.0];
                
                if i > 0 && hit == false {
                    break;
                }
                for _ in 0..bounces {
                    let mut closest_hit = Hit {
                        distance: -1.0,
                        point: vec![0.0, 0.0, 0.0],
                        normal: vec![0.0, 0.0, 0.0],
                        sphere: spheres[0].clone(),
                    };
                    for sphere in &spheres {
                        let hit = ray.get_collision(&sphere);
                        if hit.distance > 0.00001 && (hit.distance < closest_hit.distance || closest_hit.distance < 0.0) {
                            //let color = (hit.normal[0] * 255.0) as u8;
                            closest_hit = hit;
                        }
                    }
                    if closest_hit.distance < 0.0 {
                        break;
                    }
                    hit = true;
                    ray.direction = get_random_bounce(&closest_hit.normal);
                    ray.origin = closest_hit.point.clone();
                    let light_emitted = vec![
                        closest_hit.sphere.color[0] * closest_hit.sphere.light,
                        closest_hit.sphere.color[1] * closest_hit.sphere.light,
                        closest_hit.sphere.color[2] * closest_hit.sphere.light,
                    ];
                    accumulated_light = vec![
                        accumulated_light[0] + light_emitted[0] * ray_color[0],
                        accumulated_light[1] + light_emitted[1] * ray_color[1],
                        accumulated_light[2] + light_emitted[2] * ray_color[2],
                    ];
                    ray_color = vec![
                        ray_color[0] * closest_hit.sphere.color[0],
                        ray_color[1] * closest_hit.sphere.color[1],
                        ray_color[2] * closest_hit.sphere.color[2],
                    ];
                }
                total_light[0] += accumulated_light[0];
                total_light[1] += accumulated_light[1];
                total_light[2] += accumulated_light[2];
            }
            total_light[0] = total_light[0] * 256.0 / samples_per_pixel as f64;
            total_light[1] = total_light[1] * 256.0 / samples_per_pixel as f64;
            total_light[2] = total_light[2] * 256.0 / samples_per_pixel as f64;
            if total_light == vec![0.0, 0.0, 0.0] && !hit{
                total_light = vec![5.0, 35.0, 84.0];
            }
            let formatted_light = vec![total_light[0] as u8, total_light[1] as u8, total_light[2] as u8];
            pixels.push(formatted_light);
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
    let cube_width = 5;
    let cube_height = 5;
    let x_cubes = width / cube_width;
    let y_cubes = height / cube_height;
    
    let threads = 6;
    let bounces = 10;
    let samples_per_pixel = 2048;
    let mut img = image::Image::blank(width, height);
    let camera = Camera {
        origin: vec![0.0, 0.0, -50.0],
        yaw: 0.0,
        pitch: 0.0,
        fov: 90.0 * std::f64::consts::PI / 180.0,
        width: width,
        height: height,
    };

    

    let (tx, rx) = mpsc::channel();

    for i in 0..threads {
        
        let camera = camera.clone();
        let spheres = spheres.clone();
        let tx = tx.clone();
        
        thread::spawn(move || {
            for j in 0..(x_cubes * y_cubes / threads) {
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
        

        println!("{} {}", cube.x_index, cube.y_index);
        count += 1;
        if count % threads == 0 {
            image::Image::save_to_file(&mut img, "test7.png");
        }
        if count == x_cubes * y_cubes {
            image::Image::save_to_file(&mut img, "test7.png");
            break;
        }
    }
}

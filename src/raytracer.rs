
struct Ray {
    origin: Vec<f64>,
    direction: Vec<f64>,
}
#[derive(Clone)]
struct Sphere {
    center: Vec<f64>,
    radius: f64,
    color: Vec<f64>,
    light: f64,
    smoothness: f64,
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


struct Hit {
    distance: f64,
    point: Vec<f64>,
    normal: Vec<f64>,
    sphere: Sphere,
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

fn get_specular_reflection(normal: &Vec<f64>, direction: &Vec<f64>) -> Vec<f64> {
    let dot_product = normal[0] * direction[0] + normal[1] * direction[1] + normal[2] * direction[2];
    let mut reflection = vec![
        direction[0] - 2.0 * normal[0] * dot_product,
        direction[1] - 2.0 * normal[1] * dot_product,
        direction[2] - 2.0 * normal[2] * dot_product,
    ];
    let size = (reflection[0] * reflection[0] + reflection[1] * reflection[1] + reflection[2] * reflection[2]).sqrt();
    reflection[0] /= size;
    reflection[1] /= size;
    reflection[2] /= size;
    reflection
}

fn interpolate_specular_diffuse(direction1: &Vec<f64>, direction2: &Vec<f64>, smoothness: f64) -> Vec<f64> {
    let mut interpolated = vec![
        direction1[0] * smoothness + direction2[0] * (1.0 - smoothness),
        direction1[1] * smoothness + direction2[1] * (1.0 - smoothness),
        direction1[2] * smoothness + direction2[2] * (1.0 - smoothness),
    ];
    let size = (interpolated[0] * interpolated[0] + interpolated[1] * interpolated[1] + interpolated[2] * interpolated[2]).sqrt();
    interpolated[0] /= size;
    interpolated[1] /= size;
    interpolated[2] /= size;
    interpolated
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
    fn get_collision(&mut self, sphere: &Sphere) -> Hit{
        let delta_position = vec![
            self.origin[0] - sphere.center[0],
            self.origin[1] - sphere.center[1],
            self.origin[2] - sphere.center[2],
        ];
        let a = self.direction[0] * self.direction[0] + self.direction[1] * self.direction[1] + self.direction[2] * self.direction[2];
        let b = 2.0 * (delta_position[0] * self.direction[0] + delta_position[1] * self.direction[1] + delta_position[2] * self.direction[2]);
        let c = delta_position[0] * delta_position[0] + delta_position[1] * delta_position[1] + delta_position[2] * delta_position[2] - sphere.radius * sphere.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Hit {
                distance: -1.0,
                point: vec![0.0, 0.0, 0.0],
                normal: vec![0.0, 0.0, 0.0],
                sphere: sphere.clone(),
            };
        }
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t < 0.00001 {
            return Hit {
                distance: -1.0,
                point: vec![0.0, 0.0, 0.0],
                normal: vec![0.0, 0.0, 0.0],
                sphere: sphere.clone(),
            };
        }
        let point = vec![
            self.origin[0] + self.direction[0] * t,
            self.origin[1] + self.direction[1] * t,
            self.origin[2] + self.direction[2] * t,
        ];
        let normal = vec![
            (point[0] - sphere.center[0]) / sphere.radius,
            (point[1] - sphere.center[1]) / sphere.radius,
            (point[2] - sphere.center[2]) / sphere.radius,
        ];
        Hit {
            distance: t,
            point: point,
            normal: normal,
            sphere: sphere.clone(),
        }
    }
}

fn ray_trace_pixel(x_index: u32, y_index: u32, width: u32, height: u32, camera: Camera, spheres: Vec<Sphere>, samples_per_pixel: u32, bounces: u32) -> Vec<u8> {
    
    let mut total_light = vec![0.0, 0.0, 0.0];
    let mut hit = false;
    let mut ray = Ray {
        origin: vec![0.0, 0.0, 0.0],
        direction: vec![0.0, 0.0, 0.0],
    };
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
                if hit.distance > 0.0 && (hit.distance < closest_hit.distance || closest_hit.distance < 0.0) {
                    //let color = (hit.normal[0] * 255.0) as u8;
                    closest_hit = hit;
                }
            }
            if closest_hit.distance < 0.0 {
                break;
            }
            hit = true;
            let diffuse_bounce = get_random_bounce(&closest_hit.normal);
            let specular_bounce = get_specular_reflection(&closest_hit.normal, &ray.direction);
            let interpolated_bounce = interpolate_specular_diffuse(&specular_bounce, &diffuse_bounce, closest_hit.sphere.smoothness);
            ray.direction = interpolated_bounce;
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
    formatted_light
}

pub fn new_camera(width: u32, height: u32, position: Vec<f64>) -> Camera {
    Camera {
        origin: position,
        yaw: 0.0,
        pitch: 0.0,
        fov: 90.0 * std::f64::consts::PI / 180.0,
        width: width,
        height: height,
    }
}
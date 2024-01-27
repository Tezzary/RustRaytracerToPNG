mod image;


#[derive(Clone)]
struct Sphere {
    center: Vec<f64>,
    radius: f64,
    color: Vec<f64>,
    light: f64,
}

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
        center: vec![30.0, 0.0, 80.0],
        radius: 50.0,
        color: vec![1.0, 1.0, 1.0],
        light: 5.0,
    }); 
    spheres.push(Sphere {
        center: vec![-30.0, 10.0, -10.0],
        radius: 5.0,
        color: vec![1.0, 1.0, 1.0],
        light: 10.0,
    });
    return spheres;
}
fn main() {
    let spheres = generate_scene();
    let width = 200;
    let height = 100;
    let mut img = image::Image::blank(width, height);
    let camera = Camera {
        origin: vec![0.0, 0.0, -50.0],
        yaw: 0.0,
        pitch: 0.0,
        fov: 90.0 * std::f64::consts::PI / 180.0,
        width: width,
        height: height,
    };
    let mut ray = Ray {
        origin: camera.origin.clone(),
        direction: vec![0.0, 0.0, 0.0],
    };
    for y in 0..height {
        for x in 0..width {
            let y_index = height - y - 1;
            ray.reset_direction(&camera, x, y_index);
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
                continue;
            }
            img.write_to_pixel(x, y, [(closest_hit.sphere.color[0] * 256.0) as u8, (closest_hit.sphere.color[1] * 256.0) as u8, (closest_hit.sphere.color[2] * 256.0) as u8, 255]);
        }
    }
    image::Image::save_to_file(&mut img, "test.png");
    
}

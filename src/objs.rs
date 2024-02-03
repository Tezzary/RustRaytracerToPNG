pub struct Sphere {
    center: Vec<f64>,
    radius: f64,
    color: Vec<f64>,
    light: f64,
    smoothness: f64,
}
#[derive(Clone)]
pub struct Camera {
    origin: Vec<f64>,
    yaw: f64,
    pitch: f64,
    fov: f64,
    width: u32,
    height: u32,
}


pub struct Hit {
    distance: f64,
    point: Vec<f64>,
    normal: Vec<f64>,
    sphere: Sphere,
}

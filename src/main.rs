mod raytracer;
mod image;

use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_web::http::header;

use std::thread;
#[get("/generateImage")]
async fn render_image() -> impl Responder{
    let width = 200;
    let height = 100;

    let bounces = 10;
    let samples_per_pixel = 500;

    let threads = 20;

    let spheres = raytracer::generate_scene();
    let camera = raytracer::new_camera(width, height, vec![0.0, 0.0, -50.0]);
    let filename = image::create_unused_filename();
    let thread_filename = filename.clone();
    thread::spawn(move || {
        raytracer::raytrace_image(width, height, &camera, &spheres, samples_per_pixel, bounces, threads, thread_filename);
    });
    HttpResponse::Ok().body(format!("{}.png", filename))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .service(fs::Files::new("/images", "images").show_files_listing())
            .service(render_image)
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

mod raytracer;
mod image;

use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder, Result};
use actix_files::NamedFile;
use std::path::PathBuf;
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/generateImage")]
async fn render_image() -> impl Responder{
    let width = 100;
    let height = 50;

    let bounces = 1;
    let samples_per_pixel = 10;

    let threads = 10;

    let spheres = raytracer::generate_scene();
    let camera = raytracer::new_camera(width, height, vec![0.0, 0.0, -50.0]);

    let filename = raytracer::raytrace_image(width, height, &camera, &spheres, samples_per_pixel, bounces, threads);
    HttpResponse::Ok().body(format!("{}.png", filename))
}


async fn get_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(fs::Files::new("/images", "images").show_files_listing())
            .route("/{filename:.*}", web::get().to(get_file))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

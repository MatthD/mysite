use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;
use actix_files as fs;

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./src/front/public/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(||{
        App::new().service(fs::Files::new("/", "./src/front/public").show_files_listing().index_file("index.html"))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

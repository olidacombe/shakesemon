use actix_web::dev::Server;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

#[derive(Deserialize, Serialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
}

#[get("/pokemon/{name}")]
async fn pokemon(path: web::Path<(String,)>) -> impl Responder {
    let (name,) = path.into_inner();
    web::Json(Pokemon {
        name,
        description: "TODO".to_owned(),
    })
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(pokemon))
        .listen(listener)?
        .run();
    Ok(server)
}

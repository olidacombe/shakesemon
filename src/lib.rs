mod error;
mod pokemon;
mod translation;

use crate::error::Error;
use crate::pokemon::get_pokemon_description_from_name;
pub use crate::pokemon::Pokemon;
use crate::translation::get_shakespearean_translation;

use actix_web::dev::Server;
use actix_web::{get, web, App, HttpServer};
use std::net::TcpListener;

#[get("/pokemon/{name}")]
async fn get_pokemon(path: web::Path<(String,)>) -> Result<web::Json<Pokemon>, Error> {
    let (name,) = path.into_inner();
    let plain_description = get_pokemon_description_from_name(&name)?;
    let description = get_shakespearean_translation(&plain_description).await?;
    Ok(web::Json(Pokemon { name, description }))
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(get_pokemon))
        .listen(listener)?
        .run();
    Ok(server)
}

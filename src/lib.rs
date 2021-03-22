mod error;
#[cfg(test)]
pub mod mocks;
mod pokemon;
mod translation;

use crate::error::Error;
pub use crate::pokemon::Pokemon;
use crate::translation::Translator;

use actix_web::dev::Server;
use actix_web::{get, web, App, HttpServer};
use std::net::TcpListener;

#[get("/pokemon/{name}")]
async fn get_pokemon(path: web::Path<(String,)>) -> Result<web::Json<Pokemon>, Error> {
    let (name,) = path.into_inner();
    let mut pokemon = Pokemon::from_name(&name).await?;
    // warning, testability hack! See tests/integration.rs for motivation
    // TODO properly
    let description = Translator::get()
        .get_shakespearean_translation(&pokemon.description)
        .await?;
    pokemon.set_description(description);
    Ok(web::Json(pokemon))
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let address = listener.local_addr()?;
    let server = HttpServer::new(|| App::new().service(get_pokemon))
        .listen(listener)?
        .run();
    println!("🚀 Shakesemon Listening on {}", address);
    Ok(server)
}

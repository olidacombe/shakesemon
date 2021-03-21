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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pokemon_description_from_name() {
        let test_cases = vec![
            ("wormadam", "When BURMY evolved, its cloak\nbecame a part of this Pokémon’s\nbody. The cloak is never shed."),
        ];

        for (name, description) in test_cases {
            match get_pokemon_description_from_name(name) {
                Ok(fetched_description) => assert_eq!(
                    fetched_description, description,
                    "Expected description `{}` for {}, received `{}`",
                    description, name, fetched_description
                ),
                Err(e) => assert!(false, "Error fetching description for {}: `{}`", name, e),
            }
        }
    }

    #[actix_rt::test]
    async fn test_get_shakespearean_translation() {
        let test_cases = vec![
            ("You gave Mr. Tim a hearty meal, but unfortunately what he ate made him die.", "Thee did giveth mr. Tim a hearty meal,  but unfortunately what he did doth englut did maketh him kicketh the bucket."),
        ];

        for (plain, expected) in test_cases {
            match get_shakespearean_translation(plain).await {
                Ok(received) => assert_eq!(
                    expected, received,
                    "Expected translation `{}` for `{}`, received `{}`",
                    expected, plain, received
                ),
                Err(e) => assert!(false, "Error fetching translation for {}: `{}`", plain, e),
            }
        }
    }
}

use actix_web::dev::Server;
use actix_web::{get, web, App, HttpServer, Responder};
use pokerust::{FromName, PokemonSpecies};
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

async fn get_pokemon_description_from_name(name: &str) -> Result<String, std::io::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!(
            "{}pokemon-species/{}",
            "https://pokeapi.co/api/v2/", name
        ))
        .send()
        .await;
    Ok(format!("description of {}", name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_get_pokemon_description_from_name() {
        let test_cases = vec![
            ("Wormadam", "When the bulb on its back grows large, it appearsto lose the ability to stand on its hind legs."),
        ];

        for (name, description) in test_cases {
            match get_pokemon_description_from_name(name).await {
                Ok(fetched_description) => assert_eq!(
                    fetched_description, description,
                    "Expected description `{}` for {}, received `{}`",
                    description, name, fetched_description
                ),
                Err(e) => assert!(false, "Error fetching description for {}: `{}`", name, e),
            }
        }
    }
}

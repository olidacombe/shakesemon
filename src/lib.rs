use actix_web::dev::Server;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use pokerust::{FlavorText, FromName, PokemonSpecies};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

#[derive(Deserialize, Serialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
struct TranslationContents {
    translated: String,
}

#[derive(Deserialize)]
struct Translation {
    contents: TranslationContents,
}

async fn get_shakespearean_translation(plain: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.funtranslations.com/translate/shakespeare.json")
        .form(&[("text", plain)])
        .send()
        .await?;
    let translation = response.json::<Translation>().await?;
    Ok(translation.contents.translated)
}

#[get("/pokemon/{name}")]
async fn pokemon(path: web::Path<(String,)>) -> Result<web::Json<Pokemon>, Error> {
    let (name,) = path.into_inner();
    let plain_description = get_pokemon_description_from_name(&name)?;
    let description = get_shakespearean_translation(&plain_description).await?;
    Ok(web::Json(Pokemon { name, description }))
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(pokemon))
        .listen(listener)?
        .run();
    Ok(server)
}

fn get_english_description_from_flavor_text_entries(entries: Vec<FlavorText>) -> Option<String> {
    match entries.iter().find(|&entry| entry.language.name == "en") {
        Some(entry) => Some(entry.flavor_text.clone()),
        _ => None,
    }
}

#[derive(Debug)]
enum Error {
    TranslationApi,
    PokemonApi,
    NoPokemonDescription,
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::TranslationApi => HttpResponse::BadGateway().json("Translation API Error."),
            Error::PokemonApi => HttpResponse::BadGateway().json("Pokemon API Error."),
            Error::NoPokemonDescription => {
                HttpResponse::NotFound().json("Pokemon Description Not Found.")
            }
        }
    }
}

// TODO find out what's going on here
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test")
    }
}

impl From<minreq::Error> for Error {
    fn from(_: minreq::Error) -> Error {
        Error::PokemonApi
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Error {
        Error::TranslationApi
    }
}

fn get_pokemon_description_from_name(name: &str) -> Result<String, Error> {
    let species = PokemonSpecies::from_name(name)?;
    match get_english_description_from_flavor_text_entries(species.flavor_text_entries) {
        Some(description) => Ok(description),
        _ => Err(Error::NoPokemonDescription),
    }
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

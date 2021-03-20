use actix_web::dev::Server;
use actix_web::{get, web, App, HttpServer, Responder};
use pokerust::{FlavorText, FromName, PokemonSpecies};
use reqwest;
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

async fn get_shakespearean_translation(plain: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    match client
        .post(&format!(
            "{}",
            "https://api.funtranslations.com/translate/shakespeare.json"
        ))
        .form(&[("text", plain)])
        .send()
        .await
    {
        Ok(response) => match response.json::<Translation>().await {
            Ok(translation) => return Ok(translation.contents.translated),
            _ => Err("Invalid translation received".to_owned()),
        },
        _ => Err(format!(
            "Failed to fetch shakespearean translation for {}",
            plain
        )),
    }
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

fn get_english_description_from_flavor_text_entries(entries: Vec<FlavorText>) -> Option<String> {
    match entries.iter().find(|&entry| entry.language.name == "en") {
        Some(entry) => Some(entry.flavor_text.clone()),
        _ => None,
    }
}

fn get_pokemon_description_from_name(name: &str) -> Result<String, &str> {
    // let species = PokemonSpecies::from_name(name);
    // println!("{:#?}", species);
    if let Ok(species) = PokemonSpecies::from_name(name) {
        if let Some(description) =
            get_english_description_from_flavor_text_entries(species.flavor_text_entries)
        {
            return Ok(description);
        }
    }
    Err("No English text description found")
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

use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
}

impl Pokemon {
    pub async fn from_name(name: &str) -> Result<Self, Error> {
        let name = name.to_lowercase();
        let api = pokeapi::PokeApi::get();
        let description = api.get_pokemon_description_from_name(&name).await?;
        Ok(Self { name, description })
    }

    pub fn set_description(&mut self, description: String) -> &Self {
        self.description = description;
        self
    }
}

mod pokeapi {
    use super::{Deserialize, Error, Serialize};

    #[derive(Deserialize, Serialize)]
    struct Language {
        pub name: String,
    }

    #[derive(Deserialize, Serialize)]
    struct FlavorText {
        pub flavor_text: String,
        pub language: Language,
    }

    #[derive(Serialize, Deserialize)]
    struct Species {
        pub flavor_text_entries: Vec<FlavorText>,
    }

    pub struct PokeApi {
        url: String,
    }

    impl PokeApi {
        pub fn get() -> Self {
            let endpoint = std::env::var("POKEAPI_URI")
                .unwrap_or_else(|_| "https://pokeapi.co/api/v2/".to_owned());
            Self::new(&endpoint)
        }
        pub fn new(url: &str) -> Self {
            Self {
                url: url.to_owned(),
            }
        }
        pub async fn get_pokemon_description_from_name(&self, name: &str) -> Result<String, Error> {
            let client = reqwest::Client::new();
            let name = name.to_lowercase();
            let response = client
                .get(format!("{}pokemon-species/{}", &self.url, &name))
                .send()
                .await?;
            let species = response.json::<Species>().await?;

            match get_english_description_from_flavor_text_entries(species.flavor_text_entries) {
                Some(description) => Ok(description),
                _ => Err(Error::NoPokemonDescription),
            }
        }
    }

    fn get_english_description_from_flavor_text_entries(
        entries: Vec<FlavorText>,
    ) -> Option<String> {
        match entries.iter().find(|&entry| entry.language.name == "en") {
            Some(entry) => Some(entry.flavor_text.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::path_regex;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    pub struct Mocks {
        _server: MockServer,
    }

    impl Mocks {
        pub async fn start() -> Self {
            let server = MockServer::start().await;

            Mock::given(path_regex(r"/pikachu$"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(r#"{"flavor_text_entries":[{"flavor_text":"When several of\nthese POKéMON\ngather, their\nelectricity could\nbuild and cause\nlightning storms.","language":{"name":"en"}}]}"#.as_bytes().to_owned(), "application/json"))
            .mount(&server)
            .await;

            Mock::given(path_regex(r"/invalidpokemonname$"))
                .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
                .mount(&server)
                .await;

            Mock::given(path_regex(r"/nodescription$"))
                .respond_with(ResponseTemplate::new(200).set_body_raw(
                    r#"{"flavor_text_entries":[]}"#.as_bytes().to_owned(),
                    "application/json",
                ))
                .mount(&server)
                .await;

            Mock::given(path_regex(r"/noenglishdescription$"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(r#"{"flavor_text_entries":[{"flavor_text":"When several of\nthese POKéMON\ngather, their\nelectricity could\nbuild and cause\nlightning storms.","language":{"name":"es"}}]}"#.as_bytes().to_owned(), "application/json"))
                .mount(&server)
                .await;

            std::env::set_var("POKEAPI_URI", &server.uri());
            return Self { _server: server };
        }
    }

    #[actix_rt::test]
    async fn test_get_pokemon_description_from_name() {
        let _mocks = Mocks::start().await;
        assert_eq!(
            pokeapi::PokeApi::get()
                .get_pokemon_description_from_name("pikachu")
                .await,
            Ok("When several of\nthese POKéMON\ngather, their\nelectricity could\nbuild and cause\nlightning storms.".to_owned())
        );
    }

    #[actix_rt::test]
    async fn test_get_no_description_error() {
        let _mocks = Mocks::start().await;
        assert_eq!(
            pokeapi::PokeApi::get()
                .get_pokemon_description_from_name("noDescription")
                .await,
            Err(Error::NoPokemonDescription)
        );
        assert_eq!(
            pokeapi::PokeApi::get()
                .get_pokemon_description_from_name("noEnglishDescription")
                .await,
            Err(Error::NoPokemonDescription)
        );
    }

    #[actix_rt::test]
    async fn test_get_pikachu_not_found_error() {
        let _mocks = Mocks::start().await;
        assert_eq!(
            pokeapi::PokeApi::get()
                .get_pokemon_description_from_name("invalidPokemonName")
                .await,
            Err(Error::NoPokemonDescription)
        );
    }
}

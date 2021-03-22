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
    use actix_web::http::StatusCode;

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
                .unwrap_or_else(|_| "https://pokeapi.co/api/v2".to_owned());
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
                .get(format!("{}/pokemon-species/{}", &self.url, &name))
                .send()
                .await?;
            if response.status() == StatusCode::NOT_FOUND {
                return Err(Error::PokemonNotFound);
            }
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
    use mocks::pokeapi::{Mocks, PIKACHU_DESCRIPTION};

    #[actix_rt::test]
    async fn test_get_pokemon_description_from_name() {
        let _mocks = Mocks::start().await;
        assert_eq!(
            pokeapi::PokeApi::get()
                .get_pokemon_description_from_name("pikachu")
                .await,
            Ok(PIKACHU_DESCRIPTION.to_owned())
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
    async fn test_get_pokemon_not_found_error() {
        let _mocks = Mocks::start().await;
        assert_eq!(
            pokeapi::PokeApi::get()
                .get_pokemon_description_from_name("invalidPokemonName")
                .await,
            Err(Error::PokemonNotFound)
        );
    }
}

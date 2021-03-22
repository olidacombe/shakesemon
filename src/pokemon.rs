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
    use super::{Deserialize, Error};

    #[derive(Deserialize)]
    struct Language {
        pub name: String,
    }

    #[derive(Deserialize)]
    struct FlavorText {
        pub flavor_text: String,
        pub language: Language,
    }

    #[derive(Deserialize)]
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

    #[actix_rt::test]
    async fn test_get_pokemon_description_from_name() {
        let test_cases = vec![
            ("wormadam", "When BURMY evolved, its cloak\nbecame a part of this Pokémon’s\nbody. The cloak is never shed."),
            ("WormAdam", "When BURMY evolved, its cloak\nbecame a part of this Pokémon’s\nbody. The cloak is never shed."),
        ];

        for (name, description) in test_cases {
            match pokeapi::PokeApi::get()
                .get_pokemon_description_from_name(name)
                .await
            {
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

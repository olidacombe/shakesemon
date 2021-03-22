use crate::error::Error;
use pokerust::{FlavorText, FromName, PokemonSpecies};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
}

impl Pokemon {
    pub fn from_name(name: &str) -> Result<Self, Error> {
        let name = name.to_lowercase();
        let description = get_pokemon_description_from_name(&name)?;
        Ok(Self { name, description })
    }

    pub fn set_description(&mut self, description: String) -> &Self {
        self.description = description;
        self
    }
}

fn get_english_description_from_flavor_text_entries(entries: Vec<FlavorText>) -> Option<String> {
    match entries.iter().find(|&entry| entry.language.name == "en") {
        Some(entry) => Some(entry.flavor_text.clone()),
        _ => None,
    }
}

fn get_pokemon_description_from_name(name: &str) -> Result<String, Error> {
    let species = PokemonSpecies::from_name(&name.to_lowercase())?;
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
            ("WormAdam", "When BURMY evolved, its cloak\nbecame a part of this Pokémon’s\nbody. The cloak is never shed."),
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
}

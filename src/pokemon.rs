use crate::error::Error;
use pokerust::{FlavorText, FromName, PokemonSpecies};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
}

fn get_english_description_from_flavor_text_entries(entries: Vec<FlavorText>) -> Option<String> {
    match entries.iter().find(|&entry| entry.language.name == "en") {
        Some(entry) => Some(entry.flavor_text.clone()),
        _ => None,
    }
}

pub fn get_pokemon_description_from_name(name: &str) -> Result<String, Error> {
    let species = PokemonSpecies::from_name(name)?;
    match get_english_description_from_flavor_text_entries(species.flavor_text_entries) {
        Some(description) => Ok(description),
        _ => Err(Error::NoPokemonDescription),
    }
}

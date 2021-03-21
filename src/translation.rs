use crate::error::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct TranslationContents {
    translated: String,
}

#[derive(Deserialize)]
struct Translation {
    contents: TranslationContents,
}

pub async fn get_shakespearean_translation(plain: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.funtranslations.com/translate/shakespeare.json")
        .form(&[("text", plain)])
        .send()
        .await?;
    let translation = response.json::<Translation>().await?;
    Ok(translation.contents.translated)
}

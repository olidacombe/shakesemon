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

#[cfg(test)]
mod tests {
    use super::*;

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

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

pub struct Translator {
    url: String,
}

impl Translator {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }

    pub async fn get_shakespearean_translation(&self, plain: &str) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let response = client
            .post(&self.url)
            .form(&[("text", plain)])
            .send()
            .await?;
        let translation = response.json::<Translation>().await?;
        Ok(translation.contents.translated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::body_string_contains;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    struct Mocks {
        server: MockServer,
    }

    impl Mocks {
        pub async fn start() -> Self {
            let server = MockServer::start().await;

            Mock::given(body_string_contains("text=you"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(r#"{"success":{"total":1},"contents":{"translated":"Thee wilt translate me","text":"you will translate me","translation":"shakespeare"}}"#.as_bytes().to_owned(), "application/json"))
            .mount(&server)
            .await;

            Mock::given(body_string_contains("text=this"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(r#"{"error":{"code":429,"message":"Too Many Requests: Rate limit of 5 requests per hour exceeded. Please wait for 46 minutes and 9 seconds."}}"#.as_bytes().to_owned(), "application/json"))
            .mount(&server)
            .await;

            return Self { server };
        }

        pub fn url(&self) -> String {
            self.server.uri()
        }
    }

    #[actix_rt::test]
    async fn test_get_shakespearean_translation() {
        let mocks = Mocks::start().await;
        let translator = Translator::new(&mocks.url());

        assert_eq!(
            translator
                .get_shakespearean_translation("you will translate me")
                .await
                .unwrap(),
            "Thee wilt translate me"
        );
    }
}

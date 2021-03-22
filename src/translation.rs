use crate::error::Error;
use actix_web::http::StatusCode;
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
    // warning, testability hack! See tests/integration.rs for motivation
    // TODO properly
    pub fn get() -> Self {
        let endpoint = std::env::var("SHAKESPEARE_TRANSLATOR_URI").unwrap_or_else(|_| {
            "https://api.funtranslations.com/translate/shakespeare.json".to_owned()
        });
        Self::new(&endpoint)
    }

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
        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            return Err(Error::TranslationApiRateLimit(
                response.text().await.unwrap_or_else(|_| "".to_owned()),
            ));
        }
        let translation = response.json::<Translation>().await?;
        Ok(translation.contents.translated)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use wiremock::matchers::body_string_contains;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    pub struct Mocks {
        _server: MockServer,
    }

    impl Mocks {
        pub async fn start() -> Self {
            let server = MockServer::start().await;

            Mock::given(body_string_contains("text=you"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(r#"{"success":{"total":1},"contents":{"translated":"Thee wilt translate me","text":"you will translate me","translation":"shakespeare"}}"#.as_bytes().to_owned(), "application/json"))
            .mount(&server)
            .await;

            Mock::given(body_string_contains("text=this"))
            .respond_with(ResponseTemplate::new(429).set_body_raw(r#"{"error":{"code":429,"message":"Too Many Requests: Rate limit of 5 requests per hour exceeded. Please wait for 46 minutes and 9 seconds."}}"#.as_bytes().to_owned(), "application/json"))
            .mount(&server)
            .await;

            Mock::given(body_string_contains("text=err"))
                .respond_with(ResponseTemplate::new(400))
                .mount(&server)
                .await;

            Mock::given(body_string_contains("text=When"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(r#"{"success":{"total":1},"contents":{"translated":"At which hour burmy evolved,  its cloak\\nbecame a part of this pokémon’s\\nbody. The cloak is nev'r did shed.","text":"When BURMY evolved, its cloak\\nbecame a part of this Pokémon’s\\nbody. The cloak is never shed.","translation":"shakespeare"}}"#.as_bytes().to_owned(), "application/json"),
                )
                .mount(&server)
                .await;

            std::env::set_var("SHAKESPEARE_TRANSLATOR_URI", &server.uri());
            return Self { _server: server };
        }
    }

    #[actix_rt::test]
    async fn test_get_shakespearean_translation() {
        let _mocks = Mocks::start().await;
        let translator = Translator::get();

        assert_eq!(
            translator
                .get_shakespearean_translation("you will translate me")
                .await
                .unwrap(),
            "Thee wilt translate me"
        );
    }

    #[actix_rt::test]
    async fn test_error_on_rate_limit() {
        let _mocks = Mocks::start().await;
        let translator = Translator::get();

        assert_eq!(
            translator
                .get_shakespearean_translation("this was one request too many")
                .await,
            Err(Error::TranslationApiRateLimit(
                r#"{"error":{"code":429,"message":"Too Many Requests: Rate limit of 5 requests per hour exceeded. Please wait for 46 minutes and 9 seconds."}}"#.to_owned()
            ))
        );
    }

    #[actix_rt::test]
    async fn test_generic_error() {
        let _mocks = Mocks::start().await;
        let translator = Translator::get();

        assert_eq!(
            translator
                .get_shakespearean_translation("err... help?")
                .await,
            Err(Error::TranslationApi)
        );
    }
}

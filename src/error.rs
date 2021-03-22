use actix_web::{http::header::ContentType, http::StatusCode, HttpResponse};
use serde_json::json;

#[derive(Debug, PartialEq)]
pub enum Error {
    TranslationApi,
    TranslationApiRateLimit(String),
    PokemonApi,
    PokemonNotFound,
    NoPokemonDescription,
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::TranslationApi => HttpResponse::BadRequest()
                .json(json!({"error": {"code": 400, "message": "Translation API Error."}})),
            Error::TranslationApiRateLimit(msg) => {
                HttpResponse::build(StatusCode::TOO_MANY_REQUESTS)
                    .append_header(ContentType::json())
                    .body(msg)
            }
            Error::PokemonApi => HttpResponse::BadRequest()
                .json(json!({"error": {"code": 400, "message": "Pokemon API Error."}})),
            Error::PokemonNotFound => HttpResponse::NotFound()
                .json(json!({"error": {"code": 404, "message": "Pokemon Not Found"}})),
            Error::NoPokemonDescription => HttpResponse::NotFound()
                .json(json!({"error": {"code": 404, "message": "Pokemon Description Not Found"}})),
        }
    }
}

// TODO find out what's going on here
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test")
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Error {
        Error::TranslationApi
    }
}

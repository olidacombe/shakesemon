use actix_web::HttpResponse;

#[derive(Debug)]
pub enum Error {
    TranslationApi,
    PokemonApi,
    NoPokemonDescription,
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::TranslationApi => HttpResponse::BadGateway().json("Translation API Error."),
            Error::PokemonApi => HttpResponse::BadGateway().json("Pokemon API Error."),
            Error::NoPokemonDescription => {
                HttpResponse::NotFound().json("Pokemon Description Not Found.")
            }
        }
    }
}

// TODO find out what's going on here
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test")
    }
}

impl From<minreq::Error> for Error {
    fn from(_: minreq::Error) -> Error {
        Error::PokemonApi
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Error {
        Error::TranslationApi
    }
}

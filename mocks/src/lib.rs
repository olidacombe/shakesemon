use serde_json::json;
use wiremock::{Mock, MockServer, ResponseTemplate};

pub mod pokeapi {
    use super::*;
    use wiremock::matchers::path_regex;

    pub static PIKACHU_DESCRIPTION: &str = "When several of\nthese POKéMON\ngather, their\nelectricity could\nbuild and cause\nlightning storms.";

    pub struct Mocks {
        _server: MockServer,
    }

    impl Mocks {
        pub async fn start() -> Self {
            let server = MockServer::start().await;

            Mock::given(path_regex(r"/pikachu$"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "flavor_text_entries": [{
                        "flavor_text": PIKACHU_DESCRIPTION,
                        "language": {
                            "name": "en"
                        }
                    }]
                })))
                .mount(&server)
                .await;

            Mock::given(path_regex(r"/invalidpokemonname$"))
                .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
                .mount(&server)
                .await;

            Mock::given(path_regex(r"/nodescription$"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "flavor_text_entries": [],
                })))
                .mount(&server)
                .await;

            Mock::given(path_regex(r"/noenglishdescription$"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                        "flavor_text_entries": [{
                            "flavor_text": PIKACHU_DESCRIPTION,
                            "language": {
                                "name": "es"
                            }
                        }]
                })))
                .mount(&server)
                .await;

            std::env::set_var("POKEAPI_URI", &server.uri());
            return Self { _server: server };
        }
    }
}

pub mod translation {
    use super::*;
    use wiremock::matchers::body_string_contains;

    pub static PIKACHU_TRANSLATION: &str = "At which hour several of\\nthese pokémon\\ngather,  their\\nelectricity couldst\\nbuild and cause\\nlightning storms.";

    pub struct Mocks {
        _server: MockServer,
    }

    impl Mocks {
        pub async fn start() -> Self {
            let server = MockServer::start().await;

            Mock::given(body_string_contains("text=you"))
            .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({"success":{"total":1},"contents":{"translated":"Thee wilt translate me","text":"you will translate me","translation":"shakespeare"}})))
            .mount(&server)
            .await;

            Mock::given(body_string_contains("text=this"))
            .respond_with(ResponseTemplate::new(429)
            .set_body_json(json!({"error":{"code":429,"message":"Too Many Requests: Rate limit of 5 requests per hour exceeded. Please wait for 46 minutes and 9 seconds."}})))
            .mount(&server)
            .await;

            Mock::given(body_string_contains("text=err"))
                .respond_with(ResponseTemplate::new(400))
                .mount(&server)
                .await;

            Mock::given(body_string_contains("text=When"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_json(json!({"success":{"total":1},"contents":{"translated":"At which hour several of\\nthese pokémon\\ngather,  their\\nelectricity couldst\\nbuild and cause\\nlightning storms.","text":"When BURMY evolved, its cloak\\nbecame a part of this Pokémon’s\\nbody. The cloak is never shed.","translation":"shakespeare"}})))
                .mount(&server)
                .await;

            std::env::set_var("SHAKESPEARE_TRANSLATOR_URI", &server.uri());
            return Self { _server: server };
        }
    }
}

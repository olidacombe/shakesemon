pub static PIKACHU_DESCRIPTION: &str = "When several of\nthese POKéMON\ngather, their\nelectricity could\nbuild and cause\nlightning storms.";

pub mod pokeapi {
    use wiremock::matchers::path_regex;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    pub struct Mocks {
        _server: MockServer,
    }

    impl Mocks {
        pub async fn start() -> Self {
            let server = MockServer::start().await;

            Mock::given(path_regex(r"/pikachu$"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(r#"{"flavor_text_entries":[{"flavor_text":"When several of\nthese POKéMON\ngather, their\nelectricity could\nbuild and cause\nlightning storms.","language":{"name":"en"}}]}"#.as_bytes().to_owned(), "application/json"))
            .mount(&server)
            .await;

            Mock::given(path_regex(r"/invalidpokemonname$"))
                .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
                .mount(&server)
                .await;

            Mock::given(path_regex(r"/nodescription$"))
                .respond_with(ResponseTemplate::new(200).set_body_raw(
                    r#"{"flavor_text_entries":[]}"#.as_bytes().to_owned(),
                    "application/json",
                ))
                .mount(&server)
                .await;

            Mock::given(path_regex(r"/noenglishdescription$"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(r#"{"flavor_text_entries":[{"flavor_text":"When several of\nthese POKéMON\ngather, their\nelectricity could\nbuild and cause\nlightning storms.","language":{"name":"es"}}]}"#.as_bytes().to_owned(), "application/json"))
                .mount(&server)
                .await;

            std::env::set_var("POKEAPI_URI", &server.uri());
            return Self { _server: server };
        }
    }
}

pub mod translation {
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
}

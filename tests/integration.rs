use actix_web::http::StatusCode;
use shakesemon::Pokemon;
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

        Mock::given(body_string_contains("text=Shoots"))
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
async fn success_responses() {
    // Arrange
    let _mocks = Mocks::start().await;

    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        // TODO use these test cases in live api tests
        // ("charizard", "Charizard flies 'round the sky in search of powerful opponents. 't breathes fire of such most wondrous heat yond 't melts aught.  However, 't nev'r turns its fiery breath on any opponent weaker than itself.")
        // ("charizard", "Spits fire yond is hot enow to melt boulders. Known to cause forest fires unintentionally."),
        ("wormadam", "At which hour burmy evolved,  its cloak\\nbecame a part of this pokémon’s\\nbody. The cloak is nev'r did shed."),
    ];

    for (name, description) in test_cases {
        // Act
        let response = client
            // Use the returned application address
            .get(&format!("{}/pokemon/{}", &address, &name))
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert!(response.status().is_success());
        let pokemon = response
            .json::<Pokemon>()
            .await
            .expect("Request returned invalid pokemon data");
        assert_eq!(pokemon.name, name, "Incorrect name serialized for {}", name);
        assert_eq!(
            pokemon.description, description,
            "Incorrect description serialized for {}",
            description
        );
    }
}

#[actix_rt::test]
async fn test_error_on_rate_limit() {
    // Arrange
    let _mocks = Mocks::start().await;

    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        // Use the returned application address
        .get(&format!("{}/pokemon/squirtle", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(
        response.text().await.unwrap(),
        r#"{"error":{"code":429,"message":"Too Many Requests: Rate limit of 5 requests per hour exceeded. Please wait for 46 minutes and 9 seconds."}}"#
    );
}

use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = shakesemon::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

use actix_web::http::StatusCode;
use mocks;
use serde_json::{json, Value};
use shakesemon::Pokemon;

#[actix_rt::test]
async fn success_responses() {
    // Arrange
    let _pokeapi_mocks = mocks::pokeapi::Mocks::start().await;
    let _translation_mocks = mocks::translation::Mocks::start().await;

    let address = spawn_app();
    let client = reqwest::Client::new();

    let name = "pikachu";

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/pokemon/{}", &address, name))
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
    assert_eq!(pokemon.description, mocks::translation::PIKACHU_TRANSLATION);
}

#[actix_rt::test]
async fn test_error_on_rate_limit() {
    // Arrange
    let _pokeapi_mocks = mocks::pokeapi::Mocks::start().await;
    let _translation_mocks = mocks::translation::Mocks::start().await;

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
        response.json::<Value>().await.unwrap(),
        json!({
          "error": {
            "code": 429,
            "message": "Too Many Requests: Rate limit of 5 requests per hour exceeded. Please wait for 46 minutes and 9 seconds."
          }
        })
    );
}

#[actix_rt::test]
async fn test_error_on_no_description() {
    // Arrange
    let _pokeapi_mocks = mocks::pokeapi::Mocks::start().await;
    let _translation_mocks = mocks::translation::Mocks::start().await;

    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec!["nodescription", "noenglishdescription"];

    for name in test_cases {
        let response = client
            // Use the returned application address
            .get(&format!("{}/pokemon/{}", &address, &name))
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(
            response.json::<Value>().await.unwrap(),
            json!({
              "error": {
                "code": 404,
                "message": "Pokemon Description Not Found"
              }
            })
        );
    }
}

#[actix_rt::test]
async fn test_error_on_unknown_pokemon() {
    // Arrange
    let _pokeapi_mocks = mocks::pokeapi::Mocks::start().await;
    let _translation_mocks = mocks::translation::Mocks::start().await;

    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        // Use the returned application address
        .get(&format!("{}/pokemon/invalidpokemonname", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        response.json::<Value>().await.unwrap(),
        json!({
          "error": {
            "code": 404,
            "message": "Pokemon Not Found"
          }
        })
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

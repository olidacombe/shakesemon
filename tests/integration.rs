use serde::Deserialize;

#[derive(Deserialize)]
struct Pokemon {
    name: String,
    description: String,
}

#[actix_rt::test]
async fn pokemon_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("charizard", "Charizard flies 'round the sky in search of powerful opponents. 't breathes fire of such most wondrous heat yond 't melts aught.  However, 't nev'r turns its fiery breath on any opponent weaker than itself.")
    ];

    for (name, description) in test_cases {
        // Act
        let response = client
            // Use the returned application address
            .get(&format!("{}/pokemon/charizard", &address))
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert!(response.status().is_success());
        let pokemon = response
            .json::<Pokemon>()
            .await
            .expect("Request returned invalid pokemon data");
        assert_eq!(pokemon.name, name);
        assert_eq!(pokemon.description, description);
    }
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

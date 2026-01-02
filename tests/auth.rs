use merzah::{models::{auth::RegistrationFormData, user::Identifier}, spawn_app};
mod common;
use common::get_test_db;
use reqwest::Client;

#[tokio::test]
async fn register_server_fn_successfully_register_a_user() {
    let addr = spawn_app();
    let relative_addr = format!("{}/auth/register", addr);

    let client = Client::new();
    let db = get_test_db().await;

    let name = "Armaan Ali".to_string();
    let identifier = Identifier::Mobile("+91 1234567890".to_string());
    let password = "secret".to_string();
    let body = RegistrationFormData::new(name, identifier, password);

    let response = client
        .post(relative_addr)
        .header("Content-type", "application/json")
        // TO-DO: Make this body work, meaning turn it into json
        .body(body)
        .send()
        .await;

    // TO-DO: Write rest of the test
}

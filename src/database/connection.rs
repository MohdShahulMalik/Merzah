use dotenvy::dotenv;
use std::env;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn init_db() -> Surreal<Client> {
    dotenv().ok();

    let db_url = env::var("SURREAL_URL").expect("SURREAL_URL must be set");
    let db_user = env::var("SURREAL_USER").expect("SURREAL_USER must be set");
    let db_pass = env::var("SURREAL_PASS").expect("SURREAL_PASS must be set");
    let db_name = env::var("SURREAL_DB").expect("SURREAL_DB must be set");
    let db_ns = env::var("SURREAL_NS").expect("SURREAL_NS must be set");

    println!("Connecting to: {}", db_url);

    let db = Surreal::new::<Ws>(&db_url)
        .await
        .expect("Failed to connect to database");

    db.signin(Root {
        username: &db_user,
        password: &db_pass,
    })
    .await
    .expect("Failed to sign in to database");

    db.use_ns(&db_ns)
        .use_db(&db_name)
        .await
        .expect("Failed to use namespace or database");
    
    db
}



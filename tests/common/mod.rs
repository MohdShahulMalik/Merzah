use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::surrealdb::SurrealDb;
use tokio::sync::OnceCell;
use uuid::Uuid;

static DB_CONTAINER: OnceCell<testcontainers::ContainerAsync<SurrealDb>> = OnceCell::const_new();

pub async fn get_test_db() -> Surreal<Client> {
    let container = DB_CONTAINER.get_or_init(|| async {
        SurrealDb::default()
            .start()
            .await
            .expect("Failed to start SurrealDB Docker container")
    }).await;

    let port = container.get_host_port_ipv4(8000).await.expect("Failed to get mapped port");
    let url = format!("127.0.0.1:{}", port);

    let db = Surreal::new::<Ws>(&url)
        .await
        .expect("Failed to connect to test database");

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Failed to sign in as root");

    let unique_ns = format!("test_{}", Uuid::new_v4().to_string().replace("-", ""));
    db.use_ns(&unique_ns)
        .use_db("test_db")
        .await
        .expect("Failed to switch to isolated namespace");

    let initial_migration_path = "migrations/definitions/_initial.json";
    let error_msg = &format!("Failed to read migration file: {}. Ensure you run tests from project root.", initial_migration_path);
    let migration_content = tokio::fs::read_to_string(initial_migration_path)
        .await
        .expect(error_msg);

    let migration_json: serde_json::Value = serde_json::from_str(&migration_content)
        .expect("Failed to parse _initial.json");

    if let Some(schemas) = migration_json.get("schemas").and_then(|v| v.as_str()) {
        db.query(schemas)
            .await
            .expect("Failed to execute initial schemas");
    }

    if let Some(events) = migration_json.get("events").and_then(|v| v.as_str()) {
        db.query(events)
            .await
            .expect("Failed to execute initial events");
    }

    db
}

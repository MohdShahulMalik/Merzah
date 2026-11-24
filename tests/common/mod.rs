#[cfg(feature = "ssr")]
use surrealdb::Surreal;
#[cfg(feature = "ssr")]
use surrealdb::engine::remote::ws::{Client, Ws};
#[cfg(feature = "ssr")]
use surrealdb::opt::auth::Root;
#[cfg(feature = "ssr")]
use testcontainers::core::{ContainerPort, WaitFor};
#[cfg(feature = "ssr")]
use testcontainers::runners::AsyncRunner;
#[cfg(feature = "ssr")]
use testcontainers::{ContainerAsync, core::Image};
#[cfg(feature = "ssr")]
use testcontainers_modules::surrealdb::SurrealDb;
#[cfg(feature = "ssr")]
use tokio::sync::OnceCell;
#[cfg(feature = "ssr")]
use uuid::Uuid;

/// The `SurrealDb` module shipped with `testcontainers-modules` still points at
/// the `v2.2` image, while the Rust client here speaks the newer `v3` protocol.
/// This wrapper reuses its env setup but starts a `v3` image instead.
#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
struct SurrealDbV3(SurrealDb);

#[cfg(feature = "ssr")]
impl Image for SurrealDbV3 {
    fn name(&self) -> &str {
        "surrealdb/surrealdb"
    }

    fn tag(&self) -> &str {
        "v3"
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        self.0.ready_conditions()
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        self.0.env_vars()
    }

    fn cmd(&self) -> impl IntoIterator<Item = impl Into<Cow<'_, str>>> {
        self.0.cmd()
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        self.0.expose_ports()
    }
}

#[cfg(feature = "ssr")]
use std::borrow::Cow;

#[cfg(feature = "ssr")]
static DB_CONTAINER: OnceCell<ContainerAsync<SurrealDbV3>> = OnceCell::const_new();

pub async fn get_test_db() -> Surreal<Client> {
    get_test_db_with_ns().await.0
}

/// Returns the database handle together with the isolated namespace, the
/// database name and the mapped container port. Callers that need to reach the
/// same database from another process (for example the `import` binary) need
/// those values to point that process at the very same namespace.
pub async fn get_test_db_with_ns() -> (Surreal<Client>, String, String, u16) {
    let container = DB_CONTAINER
        .get_or_init(|| async {
            SurrealDbV3(SurrealDb::default())
                .start()
                .await
                .expect("Failed to start SurrealDB Docker container")
        })
        .await;

    let port = container
        .get_host_port_ipv4(8000)
        .await
        .expect("Failed to get mapped port");
    let url = format!("127.0.0.1:{}", port);

    let db = Surreal::new::<Ws>(&url)
        .await
        .expect("Failed to connect to test database");

    db.signin(Root {
        username: "root".to_string(),
        password: "root".to_string(),
    })
    .await
    .expect("Failed to sign in as root");

    let unique_ns = format!("test_{}", Uuid::new_v4().to_string().replace("-", ""));
    db.use_ns(&unique_ns)
        .use_db("test_db")
        .await
        .expect("Failed to switch to isolated namespace");

    let initial_migration_path = "migrations/definitions/_initial.json";
    let error_msg = &format!(
        "Failed to read migration file: {}. Ensure you run tests from project root.",
        initial_migration_path
    );
    let migration_content = tokio::fs::read_to_string(initial_migration_path)
        .await
        .expect(error_msg);

    let migration_json: serde_json::Value =
        serde_json::from_str(&migration_content).expect("Failed to parse _initial.json");

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

    // let schema_dir = "schemas";
    // if let Ok(mut dir) = tokio::fs::read_dir(schema_dir).await {
    //     let mut files = Vec::new();
    //     while let Ok(Some(entry)) = dir.next_entry().await {
    //         let path = entry.path();
    //         if path.extension().and_then(|ext| ext.to_str()) == Some("surql") {
    //             files.push(path);
    //         }
    //     }

    //     for path in files {
    //         let content = tokio::fs::read_to_string(&path)
    //             .await
    //             .expect("Failed to read schema file");
    //         if !content.trim().is_empty() {
    //             db.query(content)
    //                 .await
    //                 .expect("Failed to execute schema file");
    //         }
    //     }
    // }

    (db, unique_ns, "test_db".to_string(), port)
}

#[cfg(feature = "ssr")]
use anyhow::Result;
#[cfg(feature = "ssr")]
use surrealdb::{Surreal, engine::remote::ws::Client};

#[cfg(feature = "ssr")]
pub async fn cleanup_expired_sessions_job(db: &Surreal<Client>) -> Result<u64> {
    use crate::auth::session::cleanup_expired_sessions;
    cleanup_expired_sessions(db).await?;
    Ok(0)
}

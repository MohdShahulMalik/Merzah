#[cfg(feature = "ssr")]
use anyhow::Result;
#[cfg(feature = "ssr")]
use surrealdb::{Surreal, engine::remote::ws::Client};

#[cfg(feature = "ssr")]
pub async fn start_scheduler(db: Surreal<Client>) -> Result<()> {
    use tokio_cron_scheduler::{Job, JobScheduler};
    use tracing::{error, info};

    use crate::auth::session::cleanup_expired_sessions;
    use crate::services::recurrence::check_and_rotate_events;

    let scheduler = JobScheduler::new().await?;

    let db_clone = db.clone();
    let job = Job::new_async("0 0 * * * *", move |_uuid, _lock| {
        let db = db_clone.clone();
        Box::pin(async move {
            match check_and_rotate_events(&db).await {
                Ok(rotated_count) => {
                    info!(
                        "Checked and rotated events, {} events rotated",
                        rotated_count
                    );
                }
                Err(e) => {
                    error!("Error rotating events: {:?}", e);
                }
            }
        })
    })?;

    scheduler.add(job).await?;

    let db_sessions = db.clone();
    let session_job = Job::new_async("0 */15 * * * *", move |_uuid, _lock| {
        let db = db_sessions.clone();
        Box::pin(async move {
            if let Err(e) = cleanup_expired_sessions(&db).await {
                error!("Error cleaning expired sessions: {:?}", e);
            } else {
                info!("Cleaned expired sessions");
            }
        })
    })?;

    scheduler.add(session_job).await?;
    scheduler.start().await?;

    Ok(())
}

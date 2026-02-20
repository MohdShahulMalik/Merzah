use anyhow::Result;
use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

use crate::services::recurrence::check_and_rotate_events;

pub async fn start_scheduler(db: Surreal<Client>) -> Result<()> {
    let scheduler = JobScheduler::new().await?;

    let db_clone = db.clone();
    let job = Job::new_async("0 0 * * * *", move |_uuid, _lock| {
        let db = db_clone.clone();
        Box::pin(async move {
            match check_and_rotate_events(&db).await {
                Ok(rotated_count) => {
                    info!("Checked and rotated events, {} events rotated", rotated_count);
                }
                Err(e) => {
                    error!("Error rotating events: {:?}", e);
                }
            }
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    Ok(())
}  

# OneSignal Push Notification Integration Plan

OneSignal is integrated as Merzah's push notification engine, enabling real-time, high-reliability alerts across web (PWA), iOS, and Android platforms. It handles all critical mosque and community communications without requiring a custom notification server, leveraging OneSignal's robust infrastructure for scalability and deliverability.

## Core Purpose in Merzah

Push notifications serve three pillars:

1. **Prayer Timeliness**: Live Iqamah updates from mosque admins (e.g., "Isha Iqamah moved to 8:15 PM").
2. **Community Engagement**: Event reminders and new activity alerts (e.g., "Jumu'ah khutbah on Family Life – RSVP now").
3. **Learning Nudges**: Educational progress prompts (e.g., "Continue your Fiqh course – Lesson 3 ready").

---

## Architecture Overview

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   Frontend      │────▶│  Server Function │────▶│   SurrealDB     │
│ (OneSignal SDK) │     │ (register sub)   │     │ (subscriptions) │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                                │
                                ▼
                        ┌──────────────────┐
                        │ NotificationSvc  │────▶ OneSignal API
                        └──────────────────┘
```

---

## 1. Environment Variables

Add to `.env`:

```
ONESIGNAL_APP_ID=your_app_id
ONESIGNAL_API_KEY=your_rest_api_key
```

---

## 2. Database Schema

Run manually in SurrealDB:

```sql
DEFINE TABLE device_subscriptions SCHEMAFULL;
DEFINE FIELD user ON device_subscriptions TYPE record<users>;
DEFINE FIELD player_id ON device_subscriptions TYPE string;
DEFINE FIELD platform ON device_subscriptions TYPE string;
DEFINE FIELD active ON device_subscriptions TYPE bool DEFAULT true;
DEFINE FIELD created_at ON device_subscriptions TYPE datetime DEFAULT time::now();
DEFINE INDEX player_id_idx ON device_subscriptions COLUMNS player_id UNIQUE;
```

---

## 3. New Files to Create

| File | Purpose |
|------|---------|
| `src/models/notifications.rs` | Data structures |
| `src/services/notifications.rs` | OneSignal API client + helpers |
| `src/server_functions/notifications.rs` | Subscription management endpoints |
| `src/jobs/event_reminders.rs` | Scheduled job for event reminders |

---

## 4. Models (`src/models/notifications.rs`)

```rust
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceSubscription {
    pub id: RecordId,
    pub user: RecordId,
    pub player_id: String,
    pub platform: String,
    pub active: bool,
    pub created_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubscription {
    pub player_id: String,
    pub platform: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct SubscriptionWithUser {
    pub player_id: String,
    pub user: RecordId,
    pub active: bool,
}
```

---

## 5. Notification Service (`src/services/notifications.rs`)

```rust
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use surrealdb::{Surreal, engine::remote::ws::Client as DbClient, RecordId};
use tracing::{error, info};

pub struct OneSignalService {
    app_id: String,
    api_key: String,
    client: Client,
}

impl OneSignalService {
    pub fn from_env() -> Self {
        Self {
            app_id: std::env::var("ONESIGNAL_APP_ID").expect("ONESIGNAL_APP_ID must be set"),
            api_key: std::env::var("ONESIGNAL_API_KEY").expect("ONESIGNAL_API_KEY must be set"),
            client: Client::new(),
        }
    }

    /// Send to specific player IDs
    pub async fn send_to_players(
        &self,
        player_ids: &[String],
        title: &str,
        message: &str,
    ) -> Result<()> {
        if player_ids.is_empty() {
            return Ok(());
        }

        let response = self.client
            .post("https://api.onesignal.com/notifications")
            .header("Authorization", format!("Key {}", self.api_key))
            .json(&json!({
                "app_id": self.app_id,
                "include_player_ids": player_ids,
                "headings": {"en": title},
                "contents": {"en": message}
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            error!("OneSignal error: {:?}", response.text().await?);
        }
        Ok(())
    }

    /// Send to all users who favorited a mosque
    pub async fn notify_mosque_followers(
        &self,
        db: &Surreal<DbClient>,
        mosque_id: &RecordId,
        title: &str,
        message: &str,
    ) -> Result<()> {
        let query = r#"
            SELECT player_id FROM device_subscriptions
            WHERE active = true
            AND user IN (SELECT in FROM favorited WHERE out = $mosque_id)
        "#;
        
        let mut result = db.query(query)
            .bind(("mosque_id", mosque_id.clone()))
            .await?;
        
        let player_ids: Vec<String> = result.take(0)?;
        self.send_to_players(&player_ids, title, message).await
    }
}
```

---

## 6. Server Functions (`src/server_functions/notifications.rs`)

```rust
use leptos::{prelude::ServerFnError, server_fn::codec::Json, *};
use crate::models::notifications::CreateSubscription;
use crate::utils::ssr::{get_authenticated_user, ServerResponse};
use crate::models::api_responses::ApiResponse;

#[server(input = Json, output = Json, prefix = "/notifications", endpoint = "register")]
pub async fn register_subscription(
    subscription: CreateSubscription,
) -> Result<ApiResponse<String>, ServerFnError> {
    let (response_options, db, user) = match get_authenticated_user::<String>().await {
        Ok(ctx) => ctx,
        Err(e) => return Ok(e),
    };
    let responder = ServerResponse::new(response_options);

    // Upsert subscription (update if exists, create if not)
    let query = r#"
        IF $player_id IN (SELECT player_id FROM device_subscriptions WHERE user = $user_id) {
            UPDATE device_subscriptions SET active = true WHERE player_id = $player_id;
        } ELSE {
            CREATE device_subscriptions SET 
                user = $user_id,
                player_id = $player_id,
                platform = $platform,
                active = true;
        }
    "#;

    match db.query(query)
        .bind(("user_id", user.id))
        .bind(("player_id", subscription.player_id))
        .bind(("platform", subscription.platform))
        .await 
    {
        Ok(_) => Ok(responder.ok("Subscription registered".to_string())),
        Err(e) => Ok(responder.internal_server_error(format!("DB error: {}", e))),
    }
}

#[server(input = Json, output = Json, prefix = "/notifications", endpoint = "unsubscribe")]
pub async fn unsubscribe() -> Result<ApiResponse<String>, ServerFnError> {
    let (response_options, db, user) = match get_authenticated_user::<String>().await {
        Ok(ctx) => ctx,
        Err(e) => return Ok(e),
    };
    let responder = ServerResponse::new(response_options);

    let query = "UPDATE device_subscriptions SET active = false WHERE user = $user_id";
    db.query(query).bind(("user_id", user.id)).await?;
    
    Ok(responder.ok("Unsubscribed from notifications".to_string()))
}
```

---

## 7. Scheduled Event Reminders (`src/jobs/event_reminders.rs`)

```rust
use anyhow::Result;
use surrealdb::{engine::remote::ws::Client, Surreal, RecordId};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::error;

use crate::services::notifications::OneSignalService;

pub async fn start_reminder_scheduler(db: Surreal<Client>) -> Result<()> {
    let scheduler = JobScheduler::new().await?;
    let onesignal = OneSignalService::from_env();

    // Run every 15 minutes
    let job = Job::new_async("0 */15 * * * *", move |_uuid, _lock| {
        let db = db.clone();
        let onesignal = onesignal.clone();
        Box::pin(async move {
            if let Err(e) = send_upcoming_event_reminders(&db, &onesignal).await {
                error!("Event reminder job failed: {:?}", e);
            }
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;
    Ok(())
}

async fn send_upcoming_event_reminders(
    db: &Surreal<Client>,
    onesignal: &OneSignalService,
) -> Result<()> {
    // Find events starting in next 15-30 minutes
    let query = r#"
        SELECT 
            id,
            title,
            mosque,
            (SELECT name FROM mosques WHERE id = $parent.mosque)[0].name AS mosque_name
        FROM events
        WHERE date > time::now() 
        AND date < time::now() + 30m
        AND date > time::now() + 15m
    "#;

    let mut result = db.query(query).await?;
    let events: Vec<EventReminder> = result.take(0)?;

    for event in events {
        let title = format!("Upcoming: {}", event.title);
        let message = format!("Starting soon at {}", event.mosque_name.unwrap_or_else(|| "your mosque".into()));
        onesignal.notify_mosque_followers(db, &event.mosque, &title, &message).await?;
    }
    Ok(())
}

struct EventReminder {
    title: String,
    mosque: RecordId,
    mosque_name: Option<String>,
}
```

---

## 8. Integration Hooks

### In `src/server_functions/mosque.rs` - `update_adhan_jamat_times`

Add after successful update:

```rust
let onesignal = OneSignalService::from_env();
onesignal.notify_mosque_followers(&db, &mosque_id, 
    "Prayer Time Update", 
    &format!("Iqamah times have been updated at your favorite mosque")
).await.ok(); // Don't fail the request if notification fails
```

### In `src/server_functions/events.rs` - `add_event`

Add after successful event creation:

```rust
let onesignal = OneSignalService::from_env();
onesignal.notify_mosque_followers(&db, &create_event.mosque,
    "New Event",
    &format!("{} - Check it out!", create_event.title)
).await.ok();
```

---

## 9. Module Registrations

### `src/models/mod.rs`

```rust
pub mod notifications;
```

### `src/services/mod.rs`

```rust
pub mod notifications;
```

### `src/server_functions/mod.rs`

```rust
pub mod notifications;
```

### `src/jobs/mod.rs`

```rust
pub mod event_rotation;
pub mod event_reminders;
```

### `src/main.rs` - Add scheduler

```rust
use merzah::jobs::event_reminders::start_reminder_scheduler;

// In main(), alongside event_rotation scheduler:
let db_for_reminders = db.clone();
tokio::spawn(async move {
    start_reminder_scheduler(db_for_reminders).await
});
```

---

## 10. Execution Order for Single Day

| Time | Task |
|------|------|
| 0:00-0:30 | Add env vars + DB schema (run manually in SurrealDB) |
| 0:30-1:15 | Create models + notification service |
| 1:15-1:45 | Create server functions for subscription |
| 1:45-2:30 | Create event reminder scheduler job |
| 2:30-3:00 | Hook into mosque/events server functions |
| 3:00-3:30 | Module registrations + main.rs integration |
| 3:30-4:00 | Testing with cargo check + fix issues |

---

## 11. Frontend Integration (For Reference)

Add to `main.rs` HTML head:

```html
<script src="https://cdn.onesignal.com/sdks/web/v16/OneSignalSDK.page.js" defer></script>
<script>
  window.OneSignalDeferred = window.OneSignalDeferred || [];
  OneSignalDeferred.push(async function(OneSignal) {
    await OneSignal.init({ appId: "YOUR_APP_ID" });
    
    OneSignal.User.pushSubscription.addEventListener('change', async (subscription) => {
      if (subscription.optedIn) {
        await fetch('/notifications/register', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ 
            player_id: subscription.id, 
            platform: 'web' 
          })
        });
      }
    });
  });
</script>
```

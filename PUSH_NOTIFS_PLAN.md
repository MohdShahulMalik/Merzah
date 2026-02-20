# Phase 5: Push Notifications (Post-Recurrence)

## Overview
Implement cross-platform push notifications using Firebase Cloud Messaging (FCM) v1 API for both Android and iOS devices.

## Prerequisites
- Recurrence system fully implemented and tested
- Firebase project created and configured
- FCM service account credentials
- iOS APNs auth key configured in Firebase (for iOS support)

---

## 1. Architecture

### FCM Integration Flow
```
Mobile App                     Backend                    FCM                    Device
   |                              |                         |                       |
   | 1. Register Device Token -->|                         |                       |
   |                              | 2. Store token          |                       |
   |                              |    (device_tokens)      |                       |
   |<-- 3. Confirm -------------|                         |                       |
   |                              |                         |                       |
   |                              |                         |    [Event Created]    |
   |                              | 4. Query followers      |                       |
   |                              |    & device tokens      |                       |
   |                              | 5. Send FCM request --> |                       |
   |                              |                         | 6. Push notification  |
   |                              |                         | --------------------> |
   |                              |                         |                       |
```

### Notification Types

1. **New Event Notification**
   - Trigger: Event created by mosque admin
   - Recipients: All users who favorited the mosque
   - Content: "New event at [Mosque Name]: [Event Title]"
   - Deep link: Opens event detail page

2. **Event Reminder (1 Hour Before)**
   - Trigger: Scheduled job running every 5 minutes
   - Recipients: Users who RSVP'd to the event
   - Content: "[Event Title] starts in 1 hour at [Mosque Name]"
   - Deep link: Opens event detail page

3. **Event Updated Notification** (Optional)
   - Trigger: Event details changed
   - Recipients: RSVP'd users
   - Content: "[Event Title] has been updated"

---

## 2. Database Schema

### Device Tokens Table

**Create `schemas/device_tokens.surql`:**
```sql
DEFINE TABLE IF NOT EXISTS device_tokens SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS token ON device_tokens TYPE string;
    -- FCM registration token from mobile SDK
DEFINE FIELD IF NOT EXISTS platform ON device_tokens TYPE string;
    -- Values: 'android', 'ios', 'web'
DEFINE FIELD IF NOT EXISTS user ON device_tokens TYPE record<users>;
DEFINE FIELD IF NOT EXISTS created_at ON device_tokens TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON device_tokens TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS is_active ON device_tokens TYPE bool DEFAULT true;
    -- Set to false when token expires/invalid

-- Index for faster lookups
DEFINE INDEX IF NOT EXISTS idx_device_tokens_user ON device_tokens(user);
DEFINE INDEX IF NOT EXISTS idx_device_tokens_token ON device_tokens(token);
```

### Notification Logs (Optional)

**Create `schemas/notifications.surql`:**
```sql
DEFINE TABLE IF NOT EXISTS notifications SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS type ON notifications TYPE string;
    -- 'new_event', 'event_reminder', 'event_updated'
DEFINE FIELD IF NOT EXISTS user ON notifications TYPE record<users>;
DEFINE FIELD IF NOT EXISTS event ON notifications TYPE option<record<events>>;
DEFINE FIELD IF NOT EXISTS title ON notifications TYPE string;
DEFINE FIELD IF NOT EXISTS body ON notifications TYPE string;
DEFINE FIELD IF NOT EXISTS sent_at ON notifications TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS delivered ON notifications TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS fcm_response ON notifications TYPE option<string>;
    -- Store FCM response for debugging
```

---

## 3. Implementation Phases

### Phase 5.1: FCM Infrastructure

**Tasks:**
- [ ] Add FCM dependencies to `Cargo.toml`
  - `jsonwebtoken` (for OAuth2 JWT)
  - `reqwest` with `json` feature (already available)
- [ ] Create `src/services/notifications.rs`
- [ ] Implement FCM authentication (OAuth2 service account)
- [ ] Implement `send_notification()` function
- [ ] Add error handling for invalid/expired tokens

**New Dependencies:**
```toml
[dependencies]
jsonwebtoken = "9"
reqwest = { version = "0.11", features = ["json"] }
```

**Environment Variables:**
```bash
# .env
FCM_PROJECT_ID=your-firebase-project-id
FCM_SERVICE_ACCOUNT_KEY_JSON='{...}'  # JSON string of service account key
# Or
FCM_SERVICE_ACCOUNT_KEY_PATH=/path/to/serviceAccountKey.json
```

**FCM Service Structure:**
```rust
// src/services/notifications.rs

pub struct FcmService {
    project_id: String,
    access_token: String,
    token_expiry: DateTime<Utc>,
}

impl FcmService {
    /// Initialize with service account credentials
    pub async fn new(project_id: String, service_account_key: ServiceAccountKey) -> Result<Self> Error>;
    
    /// Get OAuth2 access token (auto-refresh if expired)
    async fn get_access_token(&mut self) -> Result<String, Error>;
    
    /// Send notification to single device
    pub async fn send_to_device(
        &mut self,
        device_token: &str,
        title: &str,
        body: &str,
        data: Option<HashMap<String, String>>,
    ) -> Result<FcmResponse, Error>;
    
    /// Send to multiple devices (batch)
    pub async fn send_to_devices(
        &mut self,
        device_tokens: &[String],
        title: &str,
        body: &str,
        data: Option<HashMap<String, String>>,
    ) -> Result<Vec<FcmResponse>, Error>;
}

pub struct FcmResponse {
    pub success: bool,
    pub message_id: Option<String>,
    pub error: Option<String>,
}
```

### Phase 5.2: Device Token Management API

**Create `src/server_functions/notifications.rs`:**

**Register Device Token:**
```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterDeviceToken {
    pub token: String,
    pub platform: String, // 'android', 'ios'
}

#[server(RegisterDeviceToken, "/api/notifications/register-device")]
pub async fn register_device_token(
    req: RegisterDeviceToken,
) -> Result<ApiResponse<()>, ServerFnError> {
    // 1. Get current user from session
    // 2. Check if token already exists for this user
    // 3. If exists, update updated_at
    // 4. If not, insert new device token
    // 5. Return success
}
```

**Unregister Device Token:**
```rust
#[server(UnregisterDeviceToken, "/api/notifications/unregister-device")]
pub async fn unregister_device_token(
    token: String,
) -> Result<ApiResponse<()>, ServerFnError> {
    // Soft delete: set is_active = false
}
```

**Token Cleanup (Background Job):**
```rust
// Called periodically to remove inactive tokens older than 90 days
pub async fn cleanup_expired_tokens(db: &Surreal<Client>) -> Result<u64, Error>;
```

### Phase 5.3: Notification Triggers

**A. New Event Notifications**

**Update `add_event` in `src/server_functions/events.rs`:**
```rust
pub async fn add_event(event: CreateEvent) -> Result<ApiResponse<Event>, ServerFnError> {
    // ... existing event creation logic ...
    
    // After successful creation:
    if let Ok(followers) = get_mosque_followers(&event.mosque_id, &db).await {
        let device_tokens = get_user_device_tokens(&followers, &db).await?;
        
        let fcm = FcmService::new(
            std::env::var("FCM_PROJECT_ID").unwrap(),
            load_service_account_key(),
        ).await?;
        
        let title = format!("New event at {}", mosque.name);
        let body = &event.title;
        let data = HashMap::from([
            ("type".to_string(), "new_event".to_string()),
            ("event_id".to_string(), created_event.id.to_string()),
            ("mosque_id".to_string(), event.mosque_id),
        ]);
        
        // Send asynchronously (don't block response)
        tokio::spawn(async move {
            if let Err(e) = fcm.send_to_devices(&device_tokens, &title, body, Some(data)).await {
                tracing::error!("Failed to send notifications: {}", e);
            }
        });
    }
    
    Ok(ApiResponse::success(created_event))
}
```

**B. Event Reminder Notifications**

**Create `src/jobs/event_reminders.rs`:**
```rust
/// Runs every 5 minutes to send 1-hour-before reminders
pub async fn start_reminder_scheduler(db: Surreal<Client>) {
    let scheduler = JobScheduler::new().await.unwrap();
    
    // Every 5 minutes
    scheduler.add(Job::new_async("0 */5 * * * *", move |_, _| {
        let db = db.clone();
        Box::pin(async move {
            if let Err(e) = send_event_reminders(&db).await {
                tracing::error!("Failed to send reminders: {}", e);
            }
        })
    }).unwrap()).await.unwrap();
    
    scheduler.start().await.unwrap();
}

async fn send_event_reminders(db: &Surreal<Client>) -> Result<(), Error> {
    // 1. Find events starting in 55-65 minutes (1 hour window)
    let now = Utc::now();
    let window_start = now + Duration::minutes(55);
    let window_end = now + Duration::minutes(65);
    
    let events = db
        .query("SELECT * FROM events WHERE date > $start AND date < $end")
        .bind(("start", window_start))
        .bind(("end", window_end))
        .await?
        .take::<Vec<Event>>(0)?;
    
    // 2. For each event, find RSVP'd users
    for event in events {
        let attendees = db
            .query("SELECT in FROM attending WHERE out = $event")
            .bind(("event", event.id.clone()))
            .await?
            .take::<Vec<RecordId>>(0)?;
        
        if attendees.is_empty() {
            continue;
        }
        
        // 3. Get device tokens for attendees
        let device_tokens = get_user_device_tokens(&attendees, db).await?;
        
        if device_tokens.is_empty() {
            continue;
        }
        
        // 4. Send notifications
        let mosque = get_mosque_by_id(&event.mosque_id, db).await?;
        let title = format!("{}", event.title);
        let body = format!("Starts in 1 hour at {}", mosque.name);
        
        let mut fcm = FcmService::new(
            std::env::var("FCM_PROJECT_ID").unwrap(),
            load_service_account_key(),
        ).await?;
        
        fcm.send_to_devices(&device_tokens, &title, &body, Some(data)).await?;
        
        // 5. Log notification
        log_notification(&event, &attendees, "event_reminder", db).await?;
    }
    
    Ok(())
}
```

### Phase 5.4: Token Management & Error Handling

**Handle FCM Errors:**
```rust
match fcm_response.error {
    Some(err) if err.contains("registration-token-not-registered") => {
        // Token expired or app uninstalled
        // Mark token as inactive in database
        deactivate_device_token(token, db).await?;
    }
    Some(err) if err.contains("invalid-registration-token") => {
        // Invalid token format
        deactivate_device_token(token, db).await?;
    }
    Some(err) => {
        // Other error, retry later
        tracing::error!("FCM error: {}", err);
    }
    None => {
        // Success
    }
}
```

**Mobile App Integration (Reference):**
```kotlin
// Android (Firebase Cloud Messaging)
FirebaseMessaging.getInstance().token.addOnCompleteListener { task ->
    if (task.isSuccessful) {
        val token = task.result
        // Send to backend via POST /api/notifications/register-device
    }
}

// iOS (Firebase Cloud Messaging + APNs)
// Configure FCM with APNs, get token via Messaging.messaging().token
// Send to backend via POST /api/notifications/register-device
```

---

## 4. Testing Strategy

### Unit Tests
- [ ] FCM OAuth2 token generation
- [ ] FCM request payload construction
- [ ] Device token CRUD operations
- [ ] Error handling for invalid tokens

### Integration Tests
- [ ] Register device token via API
- [ ] Unregister device token
- [ ] Send test notification via FCM
- [ ] Verify notification delivery (manual)

### Manual Testing
- [ ] Test on Android device (FCM)
- [ ] Test on iOS device (FCM + APNs)
- [ ] Test with multiple devices per user
- [ ] Test token refresh scenario
- [ ] Verify deep linking works

---

## 5. Configuration

### Firebase Setup

1. **Create Firebase Project:**
   - Go to https://console.firebase.google.com
   - Create new project or use existing

2. **Add Android App:**
   - Register app with package name
   - Download `google-services.json`
   - Add to mobile app

3. **Add iOS App:**
   - Register app with bundle ID
   - Download `GoogleService-Info.plist`
   - Configure APNs authentication key

4. **Get Service Account Key:**
   - Project Settings > Service Accounts
   - Generate new private key
   - Download JSON file
   - Store in environment variable

5. **Get Project ID:**
   - Found in project settings
   - Store in `FCM_PROJECT_ID`

### Environment Variables

```bash
# Required
FCM_PROJECT_ID=merzah-app-12345
FCM_SERVICE_ACCOUNT_KEY_JSON='{"type":"service_account","project_id":"..."}'

# Optional
NOTIFICATION_BATCH_SIZE=500  # Max devices per batch request
REMINDER_CHECK_INTERVAL_MINUTES=5
```

---

## 6. Success Criteria

- [ ] Device tokens can be registered/unregistered via API
- [ ] Users receive push notifications for new events at favorited mosques
- [ ] Users receive 1-hour reminders for RSVP'd events
- [ ] Notifications work on both Android and iOS
- [ ] Expired/invalid tokens are handled gracefully
- [ ] Batch sending for large audiences
- [ ] Deep links open correct screens in mobile apps

---

## 7. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| FCM rate limits | Medium | Implement batching, queueing, exponential backoff |
| Token expiration | Medium | Handle FCM errors, mark tokens inactive |
| iOS APNs configuration | High | Follow Apple docs carefully, test early |
| Notification fatigue | Low | Allow user preferences, smart grouping |
| Battery drain | Low | Batch notifications, efficient scheduling |

---

## 8. Future Enhancements

### 8.1 User Preferences
Allow users to customize notifications:
- Toggle new event notifications
- Toggle reminder notifications
- Set reminder time (1 hour, 30 min, 15 min)
- Quiet hours (don't send notifications at night)

### 8.2 Rich Notifications
- Images in notifications
- Action buttons ("RSVP", "Dismiss", "View")
- Grouped notifications by mosque

### 8.3 Notification History
- In-app notification center
- Mark as read/unread
- Archive old notifications

### 8.4 Analytics
- Track notification open rates
- A/B test notification copy
- Measure engagement impact

---

## 9. Notes

- FCM v1 API is the modern approach (replaces legacy FCM HTTP API)
- iOS requires APNs setup even when using FCM
- Test on real devices, simulators don't support push notifications
- Consider using Firebase Admin SDK (if available for Rust) instead of raw HTTP
- Keep notification payload under 4KB
- Use data messages for silent notifications (no UI alert)

**Prerequisites Before Starting:**
1. Firebase project created
2. Android and iOS apps registered in Firebase
3. Service account key downloaded
4. iOS APNs auth key uploaded to Firebase

---

**Last Updated:** 2026-02-15
**Status:** Recurrence Phase 1 Ready, Push Notifications Pending Recurrence Completion

# Events Recurrence & Push Notifications Plan

## Overview
Implement event recurrence with in-place rotation (single event record that updates its date when the occurrence ends).

---

## 1. Current State

**Implemented:**
- Favorites system (`favorited` edge: users -> mosques)
- Basic events table (title, description, category, date)
- RSVP system (`attending` edge: users -> events)
- Event categories: Deen, Dunya, Fundraiser

**Status Check:**
- [x] Phase 1: Favorites & Taxonomy
- [x] Phase 2: Basic Event Core (CRUD)
- [x] Phase 3: RSVP System
- [ ] Phase 4: Recurrence (IN PROGRESS)
- [ ] Phase 5: Push Notifications (PENDING)

---

## 2. Recurrence Implementation (In-Place Rotation)

### Core Concept
Instead of deleting expired events and creating new ones, update the existing event's date field to the next occurrence. This preserves:
- Event ID and all relationships
- RSVPs (users remain registered for "next occurrence")
- Event history in a single record

### Database Schema Changes

**Update `schemas/events.surql`:**
```sql
-- Recurrence configuration
DEFINE FIELD IF NOT EXISTS recurrence_pattern ON events TYPE option<string>;
    -- Values: 'daily', 'weekly', 'biweekly', 'monthly', 'quarterly', 'yearly'
    -- No interval field - pattern names are self-descriptive
DEFINE FIELD IF NOT EXISTS recurrence_end_date ON events TYPE option<datetime>;
    -- Calculated from duration dropdown on create (not user-entered)
```

**Duration Dropdown (UI):**
```
Repeats: [Weekly ▼]
Duration: [3 months ▼]
Options:
- 1 month
- 3 months (default)
- 6 months
- 1 year
- Forever
```

**API Calculation (no user input needed):**
```rust
let recurrence_end_date = match duration {
    "1_month" => start_date + Duration::days(30),
    "3_months" => start_date + Duration::days(90),
    "6_months" => start_date + Duration::days(180),
    "1_year" => start_date + Duration::days(365),
    "forever" => None, // NULL = no end date
};
```

### RSVP Behavior for Recurring Events

- **RSVP applies to series:** When a user RSVPs to a recurring event, they are RSVPing to "all future occurrences"
- **On rotation:** RSVPs persist on the same event record
- **Unsubscribe:** User can un-RSVP from the series at any time

### Edit Scopes

When editing a recurring event, user selects scope:

1. **This occurrence only** (not supported - all edits apply to template)
2. **This and future occurrences** (default)
   - Updates the current event record
   - These values become the new "template" for future rotations
3. **All occurrences** (same as #2 with in-place rotation)

### Delete Options

When deleting a recurring event:

1. **Stop recurring** (default)
   - Event becomes a one-time event (current occurrence)
2. **Delete entire series**
   - Actually delete the event record

---

## 3. Implementation Phases

### Phase 1: Database & Models (Priority)

**Tasks:**
- [x] Add recurrence fields to events table schema
- [x] Update `Event` model in `src/models/events.rs`
- [x] Update `CreateEvent` model to support recurrence
- [x] Add `UpdateEvent` scope enum

**Files to Modify:**
- `schemas/events.surql`
- `src/models/events.rs`

### Phase 2: Recurrence Logic Service

**Tasks:**
- [x] Create `src/services/recurrence.rs`
- [x] Implement `calculate_next_date()` function
- [x] Implement `rotate_event()` function (in-place update)
- [x] Add unit tests for date calculations

**Key Functions:**
```rust
use chrono::{DateTime, FixedOffset, Duration, Months};

/// Calculate next occurrence based on pattern (no interval needed)
fn calculate_next_date(
    current: DateTime<FixedOffset>,
    pattern: &str,
) -> DateTime<FixedOffset>;

/// Rotate event to next occurrence (in-place update)
async fn rotate_event(
    event_id: RecordId,
    db: &Surreal<Client>,
) -> Result<Event, Error>;

/// Check if event has passed and needs rotation
pub async fn check_and_rotate_events(
    db: &Surreal<Client>,
) -> Result<Vec<Event>, Error>;
```

### Phase 3: API Updates

**Tasks:**
- [x] Update `add_event` endpoint to support recurrence creation
- [x] Update `update_event` endpoint to handle edit scopes
- [x] Update `delete_event` endpoint to handle delete options
- [x] Update `fetch_users_favorite_mosques_events` to filter properly

**New/Modified Endpoints:**
```rust
// Create recurring event
POST /mosques/events/add-event?mosque_id=...
Body: {
    ...existing fields,
    "recurrence_pattern": "weekly",
    "recurrence_duration": "3_months"  // API calculates end_date internally
}

// Update (all edits apply to current and future)
PATCH /mosques/events/update-event
Body: {
    "event_id": "...",
    "title": "New Title",
}

// Delete with option
DELETE /mosques/events/delete-event
Body: {
    "event_id": "...",
    "delete_option": "stop"  // or "entire_series"
}
```

### Phase 4: Background Rotation Job

**Tasks:**
- [x] Create `src/jobs/event_rotation.rs`
- [x] Set up scheduled job (runs every hour)
- [x] Implement rotation logic with logging
- [ ] Add metrics/observability

**Rotation Flow:**
```rust
use chrono::Utc;

async fn run_rotation(db: &Surreal<Client>) {
    let now = Utc::now().into();
    // 1. Find expired recurring events
    // 2. For each event:
    //    a. Check if recurrence_end_date reached
    //    b. Calculate next date
    //    c. Update event.date to next occurrence
    //    d. Log rotation
    // 3. Return count of rotated events
}
```

### Phase 5: Testing & Validation

**Unit Tests:**
- [ ] Date calculation for daily/weekly/monthly patterns
- [ ] Date calculation with various intervals
- [ ] End date boundary conditions
- [ ] Timezone handling

**Integration Tests:**
- [ ] Create recurring event via API
- [ ] Manual rotation trigger
- [ ] RSVP persistence across rotation
- [ ] Edit with "future" scope
- [ ] Delete with "stop" option
- [ ] Query returns correct events (not rotated yet, not ended)

**Manual Verification:**
- [ ] Create weekly event
- [ ] RSVP to it
- [ ] Trigger rotation (manually or wait)
- [ ] Verify RSVPs persist
- [ ] Verify date updated correctly

---

## 4. Questions to Resolve

### 4.1 Rotation Timing
- **Q:** Should rotation happen immediately when event ends, or on a schedule?
- **Options:**
  - A) Real-time: Hook into event end time
  - B) Scheduled: Hourly cron job
  - C) Lazy: Update when event is fetched after expiration
- **Current Decision:** B) Hourly scheduled job (simpler, batch processing)

### 4.2 Exception Handling
- **Q:** If user edits a single occurrence, should it persist to future rotations?
- **Decision:** No - in-place rotation means all edits apply to the template. No per-instance exceptions supported.

### 4.3 RSVP Clearing
- **Q:** When event rotates, should we clear RSVPs?
- **Options:**
  - A) Keep all RSVPs (users auto-registered for next occurrence)
  - B) Clear all RSVPs (users must re-RSVP each time)
  - C) Track per-occurrence RSVPs (complex, requires history)
- **Current Decision:** A) Keep RSVPs - they apply to series

### 4.4 Notification of Rotation
- **Q:** Should users be notified when an event they RSVP'd to rotates?
- **Options:**
  - A) No notification (silent rotation)
  - B) "Your event has been rescheduled to [new date]"
  - C) Only notify if date changes significantly
- **Current Decision:** A) Silent rotation (simplest, can add later)

---

## 5. Technical Details

### Date Calculation Logic (No Interval Parameter)

```rust
use chrono::{DateTime, FixedOffset, Duration, Months};

match pattern {
    "daily" => current + Duration::days(1),
    "weekly" => current + Duration::weeks(1),
    "biweekly" => current + Duration::weeks(2),
    "monthly" => {
        // Use chrono Months for proper month arithmetic (handles year rollover, Feb 28/29, etc.)
        current.checked_add_months(Months::new(1))
            .unwrap_or(current)
    },
    "quarterly" => current.checked_add_months(Months::new(3))
        .unwrap_or(current),
    "yearly" => current.checked_add_months(Months::new(12))
        .unwrap_or(current),
    _ => current, // Fallback
}
```

**Supported Patterns:**
| Pattern | Interval | Use Case |
|---------|----------|----------|
| `daily` | Every 1 day | Daily prayer times |
| `weekly` | Every 7 days | Weekly halaqah |
| `biweekly` | Every 14 days | Bi-weekly study circle |
| `monthly` | Every 1 month | Monthly community meeting |
| `quarterly` | Every 3 months | Quarterly events |
| `yearly` | Every 12 months | Annual Eid celebration |

### Event Query Logic

When fetching events for a user:
1. Filter out events where `recurrence_end_date < now()`
2. Include both recurring and non-recurring events
3. Sort by `date` ascending
4. Include RSVP status from `attending` edge

**Note:** All dates are stored as `DateTime<FixedOffset>` in the database.

---

## 6. Future Enhancements (Post-MVP)

### 6.1 Exception Handling
Support editing single occurrences without affecting future ones:
- Store "base" event as template
- Create exception records for modified occurrences
- More complex query logic to merge template + exceptions

### 6.2 Recurrence Preview
API endpoint to show next N occurrences without creating them:
```rust
GET /events/{id}/preview?count=5
Response: ["2026-02-20T18:00:00Z", "2026-02-27T18:00:00Z", ...]
```

### 6.3 Advanced Patterns
- Multiple days per week (e.g., "Monday and Wednesday")
- Specific day of month (e.g., "15th of every month")
- Complex patterns (e.g., "First Friday of every month")

### 6.4 Push Notifications (Phase 5)
Return to push notifications after recurrence is stable:
- FCM integration for Android/iOS
- 1-hour-before reminders for RSVP'd users
- New event notifications for mosque followers

---

## 7. Success Criteria

- [ ] Can create recurring events via API
- [ ] Events rotate in-place when they expire
- [ ] RSVPs persist across rotations
- [ ] Edit with "future" scope works correctly
- [ ] Delete with "stop" option works correctly
- [ ] No duplicate events created
- [ ] No data loss during rotation
- [ ] All tests pass
- [ ] Manual testing confirms correct behavior

---

## 8. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Race condition during rotation | Medium | Use SurrealDB transactions or optimistic locking |
| Timezone issues | Medium | Store all dates in UTC, convert for display |
| Endless recurrence (no end date) | Low | Require end_date or max 1 year default |
| Performance with many rotations | Low | Indexed queries, efficient date calculations |
| Data corruption on rotation | High | Backup before rotation, test thoroughly |

---

## 9. Notes

- Keep it simple: in-place rotation is the key insight
- No separate template/instance records
- RSVPs stay on the same event ID
- Daily rotation job is acceptable latency
- Descriptive patterns (daily, weekly, biweekly, monthly) - no mental math
- Duration dropdown (1m, 3m, 6m, 1y, forever) - auto-calculates end date
- All edits apply to "this and future" by default

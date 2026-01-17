# Merzah Events Feature: Strategic Plan

## 1. Understanding the Goal
Transform Merzah into a real-time community hub by centralizing mosque activities.
- **Personalized Feed:** Based on "Favorited" mosques.
- **Event Management:** Admin tools for mosques to post one-time/recurring events.
- **RSVP (Option A):** Simple "Going" toggle for high-speed UX.
- **Real-time Notifications:** FCM integration for Android push alerts.

## 2. Investigation & Analysis
- **Current State:** Auth (`users`), Mosque records (`mosques`), and Admin relations (`handles`) are implemented.
- **Missing components:** No "favorites" system, no event schemas, no notification pipeline, and no category taxonomy.

## 3. Proposed Strategic Approach

### Phase 1: The Foundation (Favorites & Taxonomy)
- **Database:**
    - Define `favorites` relation (`users` -> `mosques`).
    - Define `event_categories` (Prayer, Education, Social, Professional, Fundraiser).
- **Logic:** 
    - Implement "Toggle Favorite" API and frontend button.

### Phase 2: Event Core (CRUD & Recurrence)
- **Database:**
    - Create `events` table with fields for title, description, category, and timing.
    - Implement "Simple Recurrence": Daily, Weekly, Monthly.
- **Admin UI:**
    - Dashboard for mosque admins to manage their event listings.
- **Public UI:**
    - Event listing on mosque detail pages.

### Phase 3: Engagement (RSVP & The Feed)
- **Database:**
    - Create `attendance` relation (`users` -> `events`).
- **RSVP Logic:** 
    - **Simple Toggle:** One-click "Going/Not Going" status (Option A).
- **The "Home" Feed:** 
    - Global feed of events from favorited mosques, sorted by date.

### Phase 4: Push Notifications (FCM Integration)
- **Infrastructure:**
    - Store `device_tokens` for Android users.
    - Set up FCM service account and backend dispatcher.
- **Triggers:**
    - Notify followers on "New Event".
    - Notify RSVP'd users 1 hour before event start.

## 4. Verification Strategy
- **Recurrence Tests:** Verify correct date generation for weekly/monthly events.
- **Feed Integrity:** Ensure users only see events from mosques they follow in their personalized feed.
- **Notification Delivery:** Manual verification on Android hardware.

## 5. Anticipated Challenges
- **FCM Token Management:** Handling expired or refreshed tokens.
- **Timezone Synchronization:** Ensuring events show at the correct local time for users.
- **Feed Performance:** Optimizing the SurrealDB query for users following many mosques.

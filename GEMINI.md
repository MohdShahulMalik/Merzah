# Merzah

Merzah is a full-stack, community‑centric Islamic platform designed to connect Muslims with their local mosques and provide structured access to both religious and worldly knowledge. It is meant to be the digital extension of the masjid: a place to organize prayer, community life, and personal growth in one coherent system.

Below is a detailed, end‑to‑end description of the project: concept, users, features, data model, and technical architecture.

## 1. Core Concept and Goals
Merzah exists to solve three problems at once:

* **Prayer & presence:** Muslims often have outdated or static prayer timings and weak integration with their actual local masjid. Merzah focuses on live, mosque‑provided Iqamah times, not just calculated adhan times.
* **Community life:** Events, classes, and programs are scattered across WhatsApp groups, posters, and announcements after Salah. Merzah centralizes them into a structured, searchable, and subscribable events hub.
* **Holistic growth:** Many apps are either purely “deen” (Quran, hadith) or purely “dunya” (productivity, finance). Merzah intentionally combines both: Islamic learning + worldly skills (study, career, tech, finance, wellbeing) framed within Islamic ethics.

**High‑level vision:**
> A single, trusted digital companion for Muslims to connect with their local mosque, stay informed about community life, and grow in both religious and worldly knowledge.

---

## 2. Primary User Types and Their Journeys

### 2.1. Regular Community Member (User)
**Profile:**
A Muslim who wants to pray in jama’ah, stay informed about mosque activities, and learn in a structured way.

**Key needs:**
* Reliable, live Iqamah times for nearby mosques.
* A simple feed of upcoming events (khutbah topics, classes, youth activities, workshops).
* Access to learning paths that combine religious and worldly knowledge.
* A clean, respectful UI available on web and mobile.

**Journey:**
1.  **Onboarding:** Selects or confirms location; Sees a list/map of nearby mosques; Marks one or more mosques as “Favorite”.
2.  **Daily Use:** Opens dashboard: sees next prayer + countdown; Sees 2–3 upcoming community events; Gets notifications for Iqamah changes or new events.
3.  **Growth:** Browses Education Hub, chooses a track (e.g., “Fiqh of Prayer”); Tracks learning progress and bookmarks resources.

### 2.2. Mosque Administrator
**Profile:**
Imam, committee member, or staff managing a masjid.

**Key needs:**
* A fast, reliable way to update Iqamah times that instantly appears in users’ apps.
* A tool to publish events and announcements without juggling posters, PDFs, and multiple social channels.
* Basic analytics: which events are popular, approximate reach, etc.

**Journey:**
1.  **Onboarding mosque:** Mosque is registered and verified; Admin account linked to mosque profile.
2.  **Day‑to‑day:** Updates prayer schedule (especially Isha/Maghrib); Creates events (Jumu’ah khutbahs, halaqahs, fundraisers); Sends timely announcements (e.g., Janazah prayer).
3.  **Benefits:** Community stays accurately informed; Fewer repetitive questions; Clearer visibility into congregant engagement.

### 2.3. Educator / Content Creator
**Profile:**
Teacher, da’i, subject‑matter expert (religious or worldly, but within Islamic ethics).

**Key needs:**
* Space to publish courses, series, and resources.
* Separation between purely religious content and ethically‑framed worldly skills.
* Ability to build structured learning paths (modules, lessons, progress).

**Journey:**
1.  **Setup:** Verified as an Educator; Gets access to an “Educator Studio”.
2.  **Creating Content:** Defines a course (e.g., “Tajweed for Beginners”); Adds modules, lessons, and attachments.
3.  **Engagement:** Users enroll and track progress; Educator sees completion percentages.

---

## 3. Feature Set in Detail

### 3.1. Dashboard (Home)
**Role:** A single glance view of what matters right now.
**Typical sections (desktop):**
* **Top strip:** User location + quick mosque selector.
* **Primary widget:** Next prayer name + Countdown timer until Iqamah + Mosque name.
* **Today’s Prayer Times Card:** 5 obligatory prayers + Jumu’ah; Iqamah times; Current/upcoming status.
* **Nearby Mosques Preview:** 2–4 cards with distance, upcoming Iqamah, and facilities (women’s area, parking, etc.).
* **Upcoming Events:** List of the next 3–5 events from favorited/nearby mosques.
* **Education Highlight:** Featured or in‑progress courses.

### 3.2. Mosque Module
* **Discovery:** Map + list view of mosques with filters (distance, facilities, language, etc.).
* **Mosque Profile:** Basic info, address, contact, live Iqamah schedule, and specific programs/events.
* **Favorites:** User can follow mosques for personalized feeds and notifications.

### 3.3. Events Module
**Event data includes:** Title, description, date/time, associated mosque, type (youth, lecture, etc.), registration needs, and location.
* **User-side:** Browse by date/mosque/category; Save/RSVP; Get reminders.
* **Admin-side:** Simple form to create/edit; Recurring patterns; Soft deletion for history retention.

### 3.4. Education Hub
**Religious (Deen) Tracks:** Aqeedah, Fiqh, Quran/Tafsir, Seerah, Contemporary topics.
**Worldly (Dunya) Tracks:** Study/Productivity, Tech/Digital skills, Career planning, Halal personal finance, Mental health/Wellbeing.
* **Mechanics:** User enrollment and progress tracking; Modular content (text, video, attachments); Integration with mosque events.

### 3.5. Authentication & Authorization
* **User registration:** Mobile/Email + password (hashed with Argon2/bcrypt).
* **WorkOS:** For using google, meta, x, and other SSO providers.
* **Roles:** `user`, `mosque_admin`, `educator`, `platform_admin`.
* **Permissions:** Role-based access control for editing mosque profiles or creating courses.
* **Audit metadata:** Records created/updated/deleted tracking.

---

## 4. Data & ER Model (Conceptual)
* **Core entities:** User, Mosque, PrayerSchedule, Event, EducationalResource/Course, Lesson, Enrollment/Progress, FavoriteMosque.
* **Relationships:**
    * One User can manage many Mosques.
    * One Mosque has many PrayerSchedules and many Events.
    * One Educator can create many Courses.
    * Many Users can enroll in many Courses (via Enrollment).

---

## 5. Technical Overview (Intended)
* **Backend:** Rust web framework (e.g., Actix‑Web) for REST/RPC, Auth, and serving Leptos functions.
* **Database:** SurrealDB (Flexible schema, graph-like relations, real-time capabilities).
* **Frontend (Web):** Leptos (Shared Rust code, compiled to WebAssembly).
* **Mobile:** React Native app consuming the same backend API.
* **Infra:** Cloud VM/Containers with CI/CD for automatic builds.

---

## 6. Design Language & Branding
* **Name:** Merzah.
* **Colors:**
    * **Light mode:** Pale lavender backgrounds, deep purple text, gold highlights.
    * **Dark mode:** Deep indigo backgrounds, bright lavender text.
* **Tone:** Respectful, calm, dignified, and friendly.

---

## 7. What Makes Merzah Distinct
* **Masjid‑first:** Real-time updates from verified mosque admins.
* **Community‑centric events:** Events are treated as first-class citizens.
* **Holistic knowledge:** Unites Deen and Dunya under one ethical framework.
* **Modern Rust/WASM stack:** High performance, safety, and maintainability.
* **Scalable roles:** Built for growth across users, mosques, and regions.

In short, **Merzah** is conceived as a comprehensive ecosystem: a place where a Muslim can open one app or website and immediately know where and when to engage with their faith and community.

---

## Instructions For Writting Tests
 - For sending json bodies with a http request NEVER use serde_json. Always create structs that will be serialized by serde.
 - For sending requests to the enpoints that are server functions. ALWAYS look closely to their "input" field at the `#[server(input = something)]`. Like PatchJson means it will accept a http Patch Request with a json body. you like to use post for every request.
 - Use multiple test cases wherever necessary with rstest or however it is more appropriate.
 - All tests should be run with --features ssr
 - While working on fixing a particular thing or writing a test then only run those tests that are relevant

 ## Intructions For Checking Code Correctness
  - NEVER use plain ```cargo check``` instead combine it with with features flag like ```cargo check --features ssr```

 ## Intructions For Code Edits (That Apply to Tests Editing and Writting As Well)
  - Don't do unnecessary code edits that are not asked to do. If you think that a particular edit is necessary but is not asked to do then just simple ask me if I want to make you that edit as if you combine the unnecessary edits with the ones that were asked to do then it's not possible for me to reject that edit and then I will end up with some cascaded unnecessary edit.
  - NEVER write builder patterns like this ```let response = client.post(&fetch_url).json(&fetch_params).send().await.expect("Failed to fetch");``` instead write them like this ```let response = client.post(&fetch_url)
        .json(&fetch_params)
        .send()
        .await
        .expect("failed to fetch");```
 - NEVER use types like this `crate::models::user::User`, always use the type after importing it at the top.

## Intructions When Asked A Question
 - DON'T go and just start changing or writting code in the codebase, use every other tool that the write tool and just answer the question properly!

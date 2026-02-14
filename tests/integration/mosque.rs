use crate::common::get_test_db;
use merzah::{
    models::{
        api_responses::{ApiResponse, MosqueResponse}, auth::{Platform, RegistrationFormData}, mosque::{PrayerTimes, PrayerTimesUpdate, MosqueRecord, MosqueSearchResult}, user::{Identifier, User}
    },
    spawn_app,
};
use merzah::auth::session::create_session;
use reqwest::Client;
use rstest::rstest;
use serde::Serialize;
use chrono::NaiveTime;
use surrealdb::{Datetime, RecordId, sql::Geometry};

#[derive(Serialize)]
struct AddMosqueParams {
    south: f64,
    west: f64,
    north: f64,
    east: f64,
}

#[derive(Serialize)]
struct FetchMosqueParams {
    lat: f64,
    lon: f64,
}

#[derive(Serialize)]
struct AddAdminParam {
    mosque_supervisor: String,
    requested_user: String,
    mosque_id: String,
}

#[derive(Serialize)]
struct FavoriteParams {
    user_id: String,
    mosque_id: String,
}

#[derive(Serialize)]
struct UpdatePersonnelParams {
    person_type: String,
    person_id: String,
    mosque_id: String,
}

#[derive(Serialize)]
struct CreateMosque {
    pub location: Geometry,
    pub name: String,
}

#[derive(Serialize)]
struct AddFavoriteParams {
    pub mosque_id: String,
}

#[derive(serde::Deserialize)]
struct Favorited {
    #[allow(dead_code)]
    id: RecordId,
    #[serde(rename = "in")]
    #[allow(dead_code)]
    user: RecordId,
    #[serde(rename = "out")]
    #[allow(dead_code)]
    mosque: RecordId,
}

#[rstest]
#[case::app_admin("app_admin", false, 200)]
#[case::mosque_admin("regular", true, 200)]
#[case::unauthorized_user("regular", false, 401)]
#[tokio::test]
async fn test_update_mosque_personnel(
    #[case] role: &str,
    #[case] is_admin_of_mosque: bool,
    #[case] expected_status: u16,
) {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    // 1. Create a mosque
    let mosque: MosqueRecord = db.create("mosques")
        .content(CreateMosque {
            location: Geometry::Point((0.0, 0.0).into()),
            name: "Test Mosque".to_string(),
        })
        .await
        .expect("Failed to create mosque")
        .expect("Not returned");

    // 2. Create the acting user
    let user_id = RecordId::from(("users", format!("user_{}", uuid::Uuid::new_v4())));
    let user: User = db.create(user_id.clone())
        .content(User {
            id: user_id.clone(),
            created_at: Datetime::default(),
            display_name: "Acting User".to_string(),
            password_hash: "hash".to_string(),
            role: role.to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create user")
        .expect("Not returned");

    // 3. If mosque admin, relate user to mosque
    if is_admin_of_mosque {
        db.query("RELATE $user -> handles -> $mosque SET granted_by = $user")
            .bind(("user", user.id.clone()))
            .bind(("mosque", mosque.id.clone()))
            .await
            .expect("Failed to relate");
    }
/*
Running tests/integration.rs (target/debug/deps/integration-d7d297805f91e71a)
running 3 tests
test mosque::test_update_mosque_personnel::case_3_unauthorized_user ... ok
test mosque::test_update_mosque_personnel::case_1_app_admin ... FAILED
test mosque::test_update_mosque_personnel::case_2_mosque_admin ... FAILED

failures:

---- mosque::test_update_mosque_personnel::case_1_app_admin stdout ----

thread 'mosque::test_update_mosque_personnel::case_1_app_admin' panicked at tests/integration/mosque.rs:156:77:
Failed to select: Db(Serialization("failed to deserialize; expected an object-like struct named $surrealdb::private::sql::Thing, found Id::String(\"imam_user\")"))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- mosque::test_update_mosque_personnel::case_2_mosque_admin stdout ----

thread 'mosque::test_update_mosque_personnel::case_2_mosque_admin' panicked at tests/integration/mosque.rs:156:77:
Failed to select: Db(Serialization("failed to deserialize; expected an object-like struct named $surrealdb::private::sql::Thing, found Id::String(\"imam_user\")"))

failures:
    mosque::test_update_mosque_personnel::case_1_app_admin
    mosque::test_update_mosque_personnel::case_2_mosque_admin

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 21 filtered out; finished in 1.82s */

    let session = create_session(user.id.clone(), &db).await.expect("Failed to create session");

    // 4. Create a personnel user to assign
    let imam_id = RecordId::from(("users", "imam_user"));
    let _: User = db.create(imam_id.clone())
        .content(User {
            id: imam_id.clone(),
            created_at: Datetime::default(),
            display_name: "Imam User".to_string(),
            password_hash: "hash".to_string(),
            role: "regular".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create imam")
        .expect("Not returned");

    // 5. Attempt update
    let update_url = format!("{}/mosques/update-personnel", addr);
    let params = UpdatePersonnelParams {
        person_type: "imam".to_string(),
        person_id: imam_id.to_string(),
        mosque_id: mosque.id.to_string(),
    };

    let response = client.patch(&update_url)
        .json(&params)
        .header("Authorization", format!("Bearer {}", session))
        .send()
        .await
        .expect("Failed to send update");

    assert_eq!(response.status().as_u16(), expected_status);

    // 6. If success, verify in DB
    if expected_status == 200 {
        let updated_mosque: Option<MosqueSearchResult> = db.query("SELECT * FROM mosques WHERE id = $mosque_id LIMIT 1 FETCH imam, muazzin")
            .bind(("mosque_id", mosque.id))
            .await
            .expect("Failed to select")
            .take(0)
            .expect("Take failed");

        let updated_mosque = updated_mosque.expect("Mosque not found");

        assert_eq!(updated_mosque.imam.map(|u| u.id), Some(imam_id));
    }
}

#[tokio::test]
async fn update_mosque_personnel_invalid_type() {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    // 1. Create app admin
    let app_admin: User = db.create("users")
        .content(User {
            id: RecordId::from(("users", "app_admin")),
            created_at: Datetime::default(),
            display_name: "App Admin".to_string(),
            password_hash: "hash".to_string(),
            role: "app_admin".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create app admin")
        .expect("Not returned");

    let admin_session = create_session(app_admin.id.clone(), &db).await.expect("Failed to create session");

    // 2. Attempt update with invalid type
    let update_url = format!("{}/mosques/update-personnel", addr);
    let params = UpdatePersonnelParams {
        person_type: "invalid_type".to_string(),
        person_id: "users:any".to_string(),
        mosque_id: "mosques:any".to_string(),
    };

    let response = client.patch(&update_url)
        .json(&params)
        .header("Authorization", format!("Bearer {}", admin_session))
        .send()
        .await
        .expect("Failed to send update");

    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn add_and_fetch_mosques() {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    // 1. Create an app_admin user directly in DB
    let app_admin: User = db.create("users")
        .content(User {
            id: RecordId::from(("users", "test_admin")),
            created_at: Datetime::default(),
            display_name: "Test Admin".to_string(),
            password_hash: "somehash".to_string(),
            role: "app_admin".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create app admin")
        .expect("User not returned");

    // 2. Create a session for the app admin
    use merzah::auth::session::create_session;
    let session_token = create_session(app_admin.id.clone(), &db).await.expect("Failed to create session");

    // 1. Add Mosques (Dearborn, MI area - small box containing Islamic Center of America)
    // Coords approx: 42.337, -83.223
    let add_url = format!("{}/mosques/add-mosque-of-region", addr);
    let add_params = AddMosqueParams {
        south: 42.32,
        west: -83.24,
        north: 42.35,
        east: -83.20,
    };

    let response = client.post(&add_url)
        .json(&add_params)
        .header("Authorization", format!("Bearer {}", session_token))
        .send()
        .await
        .expect("Failed to execute add_mosques_of_region");
    
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        panic!("Add mosques failed. Status: {}, Body: {}", status, text);
    }

    // 2. Fetch Mosques
    // Center point roughly in the middle
    let fetch_url = format!("{}/mosques/fetch-mosques-for-location", addr);
    let fetch_params = FetchMosqueParams {
        lat: 42.335,
        lon: -83.22,
    };
    
    // Trying form urlencoded first as it is the default for server functions without input=Json
    let response = client.post(&fetch_url)
        .json(&fetch_params)
        .send()
        .await
        .expect("Failed to execute fetch_mosques_for_location");

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        panic!("Fetch mosques failed. Status: {}, Body: {}", status, text);
    }

    let api_response = response.json::<ApiResponse<Vec<MosqueResponse>>>().await.expect("Failed to deserialize");
    let mosques = api_response.data.expect("No data returned");
    
    assert!(!mosques.is_empty(), "Should have found mosques in Dearborn");
    
    // Debug print found mosques
    for mosque in &mosques {
        println!("Found mosque: {:?}", mosque);
    }
}

#[derive(Serialize)]
struct ElevateSupervisorParams {
    app_admin_id: String,
    user_id: String,
}

#[derive(Serialize)]
struct UpdatePrayerTimesParams {
    mosque_admin: String,
    mosque_id: String,
    prayer_times: PrayerTimesUpdate,
}

#[tokio::test]
async fn update_mosque_prayer_times() {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    // 1. Create an app_admin user and session
    let app_admin: User = db.create("users")
        .content(User {
            id: RecordId::from(("users", "admin")),
            created_at: Datetime::default(),
            display_name: "Admin".to_string(),
            password_hash: "somehash".to_string(),
            role: "app_admin".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create an app admin") 
        .expect("The user doesn't exists");

    let admin_session = create_session(app_admin.id.clone(), &db).await.expect("Failed to create admin session");

    // 2. Add Mosques (Dearborn area again)
    let add_url = format!("{}/mosques/add-mosque-of-region", addr);
    let add_params = AddMosqueParams {
        south: 42.32,
        west: -83.24,
        north: 42.35,
        east: -83.20,
    };

    let response = client.post(&add_url)
        .json(&add_params)
        .header("Authorization", format!("Bearer {}", admin_session))
        .send()
        .await
        .expect("Failed to execute add_mosques_of_region");
    
    assert!(response.status().is_success(), "Failed to add mosques: {:?}", response.text().await);

    // 2. Fetch Mosques to get an ID
    let fetch_url = format!("{}/mosques/fetch-mosques-for-location", addr);
    let fetch_params = FetchMosqueParams {
        lat: 42.335,
        lon: -83.22,
    };
    
    let response = client.post(&fetch_url)
        .json(&fetch_params)
        .send()
        .await
        .expect("Failed to execute fetch_mosques_for_location");

    assert!(response.status().is_success(), "Failed to fetch mosques");

    let api_response = response.json::<ApiResponse<Vec<MosqueResponse>>>().await.expect("Failed to deserialize");
    let mosques = api_response.data.expect("No data returned");
    let mosque_id = mosques.first().expect("No mosques found").id.clone();

    // 3. Create supervisor user
    let supervisor_user: User = db.create("users")
        .content(User {
            id: RecordId::from(("users", format!("supervisor_{}", uuid::Uuid::new_v4()))),
            created_at: Datetime::default(),
            display_name: "Supervisor".to_string(),
            password_hash: "somehash".to_string(),
            role: "regular".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create supervisor user")
        .expect("The user doesn't exists");

    // 4. Create mosque admin user
    let mosque_admin_user: User = db.create("users")
        .content(User {
            id: RecordId::from(("users", format!("mosque_admin_{}", uuid::Uuid::new_v4()))),
            created_at: Datetime::default(),
            display_name: "Mosque Admin".to_string(),
            password_hash: "somehash".to_string(),
            role: "regular".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create mosque admin user")
        .expect("The user doesn't exists");

    // 5. Elevate supervisor
    let elevate_supervisor_url = format!("{}/mosques/elevate-user-to-mosque-supervisor", addr);
    let elevate_params = ElevateSupervisorParams {
        app_admin_id: app_admin.id.to_string(),
        user_id: supervisor_user.id.to_string(),
    };

    let response = client.post(&elevate_supervisor_url)
        .json(&elevate_params)
        .header("Authorization", format!("Bearer {}", admin_session))
        .send()
        .await
        .expect("Failed to execute elevate-user-to-mosque-supervisor");

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        panic!("Elevate supervisor failed. Status: {}, Body: {}", status, text);
    }
    let elevate_response = response.json::<ApiResponse<String>>().await.expect("Failed to deserialize elevate response");
    assert_eq!(elevate_response.data, Some("Elevated the user to mosque_supervisor".to_string()));

    // 4. Assign mosque admin
    let add_admin_url = format!("{}/mosques/add-admin", addr);
    let add_admin_params = AddAdminParam {
        mosque_supervisor: supervisor_user.id.to_string(),
        requested_user: mosque_admin_user.id.to_string(),
        mosque_id: mosque_id.to_string(),
    };

    // Create session for supervisor
    let supervisor_session = create_session(supervisor_user.id.clone(), &db).await.expect("Failed to create supervisor session");

    let response = client.post(&add_admin_url)
        .json(&add_admin_params)
        .header("Authorization", format!("Bearer {}", supervisor_session))
        .send()
        .await
        .expect("Failed to execute add-admin");

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        panic!("Add admin failed. Status: {}, Body: {}", status, text);
    }
    let add_admin_response = response.json::<ApiResponse<String>>().await.expect("Failed to deserialize add admin response");
    assert_eq!(add_admin_response.data, Some("Elevated the user to a requested_user".to_string()));

    // 5. Update Prayer Times
    let update_url = format!("{}/mosques/update-adhan-jamat-times", addr);

    let fajr = NaiveTime::from_hms_opt(5, 30, 0).unwrap();
    let dhuhr = NaiveTime::from_hms_opt(13, 30, 0).unwrap();
    let asr = NaiveTime::from_hms_opt(17, 0, 0).unwrap();
    let maghrib = NaiveTime::from_hms_opt(20, 15, 0).unwrap();
    let isha = NaiveTime::from_hms_opt(21, 45, 0).unwrap();
    let jummah = NaiveTime::from_hms_opt(13, 15, 0).unwrap();

    let new_times = PrayerTimes {
        fajr, dhuhr, asr, maghrib, isha, jummah
    };

    let update_params = UpdatePrayerTimesParams {
        mosque_admin: mosque_admin_user.id.to_string(),
        mosque_id: mosque_id.to_string(),
        prayer_times: PrayerTimesUpdate {
            adhan_times: Some(new_times.clone()),
            jamat_times: Some(new_times),
        },
    };

    // Create session for mosque admin
    let mosque_admin_session = create_session(mosque_admin_user.id.clone(), &db).await.expect("Failed to create mosque admin session");

    let response = client.patch(&update_url)
        .json(&update_params)
        .header("Authorization", format!("Bearer {}", mosque_admin_session))
        .send()
        .await
        .expect("Failed to execute update_adhan_jamat_times");

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        panic!("Update prayer times failed. Status: {}, Body: {}", status, text);
    }
    
    let update_response = response.json::<ApiResponse<String>>().await.expect("Failed to deserialize update response");
    assert_eq!(update_response.data, Some("Successfully updated jamat and adhan times".to_string()));
}

#[tokio::test]
async fn favorite_and_unfavorite_mosques() {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    // 1. Create an app_admin user and session for adding mosques
    let app_admin: User = db.create("users")
        .content(User {
            id: RecordId::from(("users", "test_admin")),
            created_at: Datetime::default(),
            display_name: "Test Admin".to_string(),
            password_hash: "somehash".to_string(),
            role: "app_admin".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create app admin")
        .expect("User not returned");

    let admin_session = create_session(app_admin.id.clone(), &db).await.expect("Failed to create admin session");

    // 1. Add Mosques (Mandawali, Delhi area - high density)
    let add_url = format!("{}/mosques/add-mosque-of-region", addr);
    let add_params = AddMosqueParams {
        south: 28.61,
        west: 77.28,
        north: 28.64,
        east: 77.31,
    };
    client.post(&add_url)
        .json(&add_params)
        .header("Authorization", format!("Bearer {}", admin_session))
        .send()
        .await
        .expect("Failed to add mosques");

    // 2. Setup User
    let user: User = db.create("users")
        .content(User {
            id: RecordId::from(("users", "fan_user")),
            created_at: Datetime::default(),
            display_name: "Fan User".to_string(),
            password_hash: "hash".to_string(),
            role: "regular".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create user")
        .expect("User not returned");

    // Create session for the regular user
    let user_session = create_session(user.id.clone(), &db).await.expect("Failed to create user session");

    // 3. Fetch Mosques
    let fetch_url = format!("{}/mosques/fetch-mosques-for-location", addr);
    let fetch_params = FetchMosqueParams {
        lat: 28.625,
        lon: 77.295,
    };
    let response = client.post(&fetch_url)
        .json(&fetch_params)
        .send()
        .await
        .expect("Failed to fetch");

    let api_response = response.json::<ApiResponse<Vec<MosqueResponse>>>().await.expect("Failed to deserialize");
    let mosques = api_response.data.expect("No mosques data");
    
    assert!(mosques.len() >= 3, "Need at least 3 mosques for this test");

    // 4. Favorite first 3 mosques
    let add_fav_url = format!("{}/mosques/add-favorite", addr);
    let mosques_to_fav = &mosques[0..3];
    
    for mosque in mosques_to_fav {
        let params = FavoriteParams {
            user_id: user.id.to_string(),
            mosque_id: mosque.id.to_string(),
        };
        let res = client.post(&add_fav_url)
            .json(&params)
            .header("Authorization", format!("Bearer {}", user_session))
            .send()
            .await
            .expect("Failed to send fav");

        if !res.status().is_success() {
             let text = res.text().await.unwrap_or_default();
             panic!("Failed to favorite mosque {}: {}", mosque.id, text);
        }
    }

    // Verify favorites exist in DB
    // Querying the 'favorited' relation table
    let relations: Vec<Favorited> = db.query("SELECT * FROM favorited WHERE in = $user")
         .bind(("user", user.id.clone()))
         .await
         .expect("Query failed")
         .take(0)
         .expect("Take failed");
    assert_eq!(relations.len(), 3, "Should have 3 favorites");

    // 5. Remove 2 favorites
    // Note: The server function is defined with endpoint="/remove-favorite"
    // Leptos/Actix usually normalize this to /mosque/remove-favorite
    let remove_fav_base_url = format!("{}/mosques/remove-favorite", addr); 
    
    let mosques_to_remove = &mosques[0..2];
    for mosque in mosques_to_remove {
        // DeleteUrl expects params in query string
        let params = [
            ("user_id", user.id.to_string()),
            ("mosque_id", mosque.id.to_string()),
        ];
        
        let res = client.delete(&remove_fav_base_url)
            .query(&params)
            .header("Authorization", format!("Bearer {}", user_session))
            .send()
            .await
            .expect("Failed to send unfav");

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            panic!("Remove favorite failed. Status: {}, Body: {}", status, text);
        }

        assert!(res.status().is_success(), "Failed to remove favorite for mosque {}", mosque.id);
    }

    // 6. Verify removals
    let relations_after: Vec<Favorited> = db.query("SELECT * FROM favorited WHERE in = $user")
         .bind(("user", user.id.clone()))
         .await
         .expect("Query failed")
         .take(0)
         .expect("Take failed");
    assert_eq!(relations_after.len(), 1, "Should have 1 favorite left");
}

#[derive(Debug, Clone, Copy)]
enum AuthMethod {
    Web,
    Mobile,
}

fn build_auth_headers(client: Client, session: &str, auth_method: AuthMethod, url: &str) -> reqwest::RequestBuilder {
    match auth_method {
        AuthMethod::Web => client.post(url).header("Cookie", format!("__Host-session={}", session)),
        AuthMethod::Mobile => client.post(url).header("Authorization", format!("Bearer {}", session)),
    }
}

fn build_auth_delete(client: Client, session: &str, auth_method: AuthMethod, url: &str) -> reqwest::RequestBuilder {
    match auth_method {
        AuthMethod::Web => client.delete(url).header("Cookie", format!("__Host-session={}", session)),
        AuthMethod::Mobile => client.delete(url).header("Authorization", format!("Bearer {}", session)),
    }
}

#[rstest]
#[case::web(AuthMethod::Web, "web_client")]
#[case::mobile(AuthMethod::Mobile, "mobile_client")]
#[tokio::test]
async fn test_favorite_mosque_with_both_auth_methods(
    #[case] auth_method: AuthMethod,
    #[case] _description: &str,
) {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    // 1. Create admin and add mosques
    let app_admin: User = db.create("users")
        .content(User {
            id: RecordId::from(("users", format!("admin_{}", uuid::Uuid::new_v4()))),
            created_at: Datetime::default(),
            display_name: "Test Admin".to_string(),
            password_hash: "hash".to_string(),
            role: "app_admin".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create admin")
        .expect("Not returned");

    let admin_session = create_session(app_admin.id.clone(), &db).await.expect("Failed to create session");

    let add_url = format!("{}/mosques/add-mosque-of-region", addr);
    let add_params = AddMosqueParams {
        south: 28.61,
        west: 77.28,
        north: 28.64,
        east: 77.31,
    };

    let add_req = build_auth_headers(client.clone(), &admin_session, auth_method, &add_url);
    let add_response = add_req.json(&add_params).send().await.expect("Failed to add mosques");
    
    if !add_response.status().is_success() {
        let text = add_response.text().await.unwrap_or_default();
        println!("Overpass API might be rate limited or unavailable. Response: {}. Skipping test.", text);
        return;
    }

    // 2. Create regular user
    let user: User = db.create("users")
        .content(User {
            id: RecordId::from(("users", format!("user_{}", uuid::Uuid::new_v4()))),
            created_at: Datetime::default(),
            display_name: "Test User".to_string(),
            password_hash: "hash".to_string(),
            role: "regular".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create user")
        .expect("Not returned");

    let user_session = create_session(user.id.clone(), &db).await.expect("Failed to create user session");

    // 3. Fetch mosques
    let fetch_url = format!("{}/mosques/fetch-mosques-for-location", addr);
    let fetch_params = FetchMosqueParams {
        lat: 28.625,
        lon: 77.295,
    };

    let fetch_response = client.post(&fetch_url)
        .json(&fetch_params)
        .send()
        .await
        .expect("Failed to fetch");

    let api_response = fetch_response.json::<ApiResponse<Vec<MosqueResponse>>>()
        .await
        .expect("Failed to deserialize");
    let mosques = api_response.data.expect("No mosques");

    assert_eq!(mosques.len(), 3, "Should have exactly 3 mosques for this test");
    
    // 4. Add favorite using the specified auth method
    let add_fav_url = format!("{}/mosques/add-favorite", addr);
    let favorite_params = FavoriteParams {
        user_id: user.id.to_string(),
        mosque_id: mosques[0].id.to_string(),
    };

    let fav_req = build_auth_headers(client.clone(), &user_session, auth_method, &add_fav_url);
    let fav_response = fav_req.json(&favorite_params).send().await.expect("Failed to send fav");

    assert!(fav_response.status().is_success(),
        "Favorite should succeed with {:?}. Status: {:?}",
        auth_method, fav_response.status());

    let fav_api_response: ApiResponse<String> = fav_response
        .json()
        .await
        .expect("Failed to deserialize");
    assert!(fav_api_response.error.is_none(), 
        "Favorite should not have error: {:?}", fav_api_response.error);
}

#[rstest]
#[case::web(AuthMethod::Web)]
#[case::mobile(AuthMethod::Mobile)]
#[tokio::test]
async fn test_unauthenticated_access_to_protected_mosque_endpoints(
    #[case] auth_method: AuthMethod,
) {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    let add_fav_url = format!("{}/mosques/add-favorite", addr);
    let favorite_params = AddFavoriteParams {
        mosque_id: "mosques:test".to_string(),
    };

    let mut req = client.post(&add_fav_url).json(&favorite_params);

    match auth_method {
        AuthMethod::Web => {
            req = req.header("Cookie", "__Host-session=invalid_session");
        }
        AuthMethod::Mobile => {
            req = req.header("Authorization", "Bearer invalid_token");
        }
    }

    let response = req.send().await.expect("Failed to send request");

    assert_eq!(response.status(), 401,
        "Unauthenticated {:?} should return 401, got: {}",
        auth_method, response.status());
}


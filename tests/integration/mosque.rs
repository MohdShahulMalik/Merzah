use crate::{auth::RegisterationFormWrapper, common::get_test_db};
use merzah::{
    models::{
        api_responses::{ApiResponse, MosqueResponse}, auth::{Platform, RegistrationFormData}, mosque::{PrayerTimes, PrayerTimesUpdate}, user::{Identifier, User}
    },
    spawn_app,
};
use reqwest::Client;
use serde::Serialize;
use chrono::NaiveTime;
use surrealdb::{Datetime, RecordId};

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

#[tokio::test]
async fn add_and_fetch_mosques() {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    let register_url = format!("{}/auth/register", addr);

    let form = RegistrationFormData::new(
        "Logout User".to_string(),
        Identifier::Email("logout@example.com".to_string()),
        "password123".to_string(),
        Platform::Web,
    );
    let body = RegisterationFormWrapper { form };

    // 1. Register
    let response = client
        .post(&register_url)
        .json(&body)
        .send()
        .await
        .expect("Failed to register");

    assert!(response.status().is_success());

    // 2. Extract Cookie
    let cookie_header = response
        .headers()
        .get("set-cookie")
        .expect("Missing Set-Cookie header in registration response");
    
    let cookie_str = cookie_header.to_str().expect("Failed to convert cookie to string");
    // Extract name=value part (strip attributes like Path, HttpOnly)
    let session_cookie = cookie_str.split(';').next().expect("Failed to parse cookie");

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
        .header("Cookie", session_cookie)
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

    // 1. Add Mosques (Dearborn area again)
    let add_url = format!("{}/mosques/add-mosque-of-region", addr);
    let add_params = AddMosqueParams {
        south: 42.32,
        west: -83.24,
        north: 42.35,
        east: -83.20,
    };

    let response = client.post(&add_url)
        .json(&add_params)
        .send()
        .await
        .expect("Failed to execute add_mosques_of_region");
    
    assert!(response.status().is_success(), "Failed to add mosques");

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

    let supervisor_user: User = db.create("users")
        .content(User {
            id: RecordId::from(("users", "supervisor")),
            created_at: Datetime::default(),
            display_name: "Supervisor".to_string(),
            password_hash: "somehash".to_string(),
            role: "regular".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create supervisor user")
        .expect("The user doesn't exists");

    let mosque_admin_user: User = db.create("users")
        .content(User {
            id: RecordId::from(("users", "mosque_admin")),
            created_at: Datetime::default(),
            display_name: "Mosque Admin".to_string(),
            password_hash: "somehash".to_string(),
            role: "regular".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create mosque admin user")
        .expect("The user doesn't exists");

    // 3. Elevate supervisor
    let elevate_supervisor_url = format!("{}/mosques/elevate-user-to-mosque-supervisor", addr);
    let elevate_params = ElevateSupervisorParams {
        app_admin_id: app_admin.id.to_string(),
        user_id: supervisor_user.id.to_string(),
    };

    let response = client.post(&elevate_supervisor_url)
        .json(&elevate_params)
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

    let response = client.post(&add_admin_url)
        .json(&add_admin_params)
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

    let response = client.patch(&update_url)
        .json(&update_params)
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

    // 1. Add Mosques (Mandawali, Delhi area - high density)
    let add_url = format!("{}/mosques/add-mosque-of-region", addr);
    let add_params = AddMosqueParams {
        south: 28.61,
        west: 77.28,
        north: 28.64,
        east: 77.31,
    };
    client.post(&add_url).json(&add_params).send().await.expect("Failed to add mosques");

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

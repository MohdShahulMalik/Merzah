use crate::common::get_test_db;
use merzah::{
    models::{
        api_responses::{ApiResponse, MosqueApiResponse},
        mosque::{PrayerTimes, PrayerTimesUpdate}, user::User,
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

#[tokio::test]
async fn add_and_fetch_mosques() {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

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
    let fetch_url = format!("{}/mosque/fetch-mosques-for-location", addr);
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

    let api_response = response.json::<ApiResponse<Vec<MosqueApiResponse>>>().await.expect("Failed to deserialize");
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
    let fetch_url = format!("{}/mosque/fetch-mosques-for-location", addr);
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

    let api_response = response.json::<ApiResponse<Vec<MosqueApiResponse>>>().await.expect("Failed to deserialize");
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
    let elevate_supervisor_url = format!("{}/mosque/elevate-user-to-mosque-supervisor", addr);
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
    let add_admin_url = format!("{}/mosque/add-admin", addr);
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
    let update_url = format!("{}/mosque/update-adhan-jamat-times", addr);

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

use crate::common::get_test_db;
use merzah::{
    models::{
        api_responses::{ApiResponse, MosqueApiResponse},
        mosque::{PrayerTimes, PrayerTimesUpdate},
    },
    spawn_app,
};
use reqwest::Client;
use serde::Serialize;
use chrono::NaiveTime;

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

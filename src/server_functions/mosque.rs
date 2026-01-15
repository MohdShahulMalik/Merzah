use leptos::{prelude::ServerFnError, server_fn::codec::Json, *};
use crate::models::api_responses::{ApiResponse, MosqueApiResponse};

#[cfg(feature = "ssr")]
use tracing::error;
#[cfg(feature = "ssr")]
use surrealdb::{RecordId, Surreal, engine::remote::ws::Client, sql::Geometry};
#[cfg(feature = "ssr")]
use actix_web::web;
#[cfg(feature = "ssr")]
use crate::models::mosque::{MosqueRecord, MosquesResponse, MosqueDbRes};

#[server(input=Json, output=Json, prefix = "/mosques", endpoint = "add-mosque-of-region")]
pub async fn add_mosques_of_region(
    south: f64,
    west: f64,
    north: f64,
    east: f64,
) -> Result<ApiResponse<String>, ServerFnError> {
    let db = leptos_actix::extract::<web::Data<Surreal<Client>>>().await?;

    let query = format!(
        r#"[out:json][timeout:30];
        (
            node["amenity"="place_of_worship"]["religion"="muslim"]({},{},{},{});
            way["amenity"="place_of_worship"]["religion"="muslim"]({},{},{},{});
        );
        out center;"#,
    south, west, north, east,
    south, west, north, east
    );

    let endpoints = [
        "https://overpass-api.de/api/interpreter",
        "https://overpass.kumi.systems/api/interpreter",
        "https://overpass.osm.ch/api/interpreter",
    ];

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(45))
        .build()?;

    let mut response = None;
    let mut last_error = None;

    for endpoint in endpoints {
        let mut attempts = 0;
        let max_attempts = 2;
        
        while attempts < max_attempts {
            attempts += 1;
            match client
                .post(endpoint)
                .body(query.clone())
                .send()
                .await {
                    Ok(res) => {
                        if res.status().is_success() {
                            response = Some(res);
                            break;
                        } else {
                            let status = res.status();
                            let body = res.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
                            let err_msg = format!("Endpoint {} returned {}, body: {}", endpoint, status, body);
                            error!("{}", err_msg);
                            last_error = Some(err_msg);
                            if status.is_server_error() && attempts < max_attempts {
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                continue;
                            }
                            break; // Try next endpoint
                        }
                    },
                    Err(e) => {
                        let err_msg = format!("Endpoint {} failed: {}", endpoint, e);
                        error!("{}", err_msg);
                        last_error = Some(err_msg);
                        if attempts < max_attempts {
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            continue;
                        }
                        break; // Try next endpoint
                    }
                }
        }
        if response.is_some() {
            break;
        }
    }

    let response = match response {
        Some(res) => res,
        None => return Err(ServerFnError::ServerError(format!("All Overpass API endpoints failed. Last error: {:?}", last_error))),
    };
    let data: MosquesResponse = response.json().await?;

    let mosques: Vec<MosqueRecord> = data.elements
        .into_iter()
        .filter_map(|elem| {
            let (lat, lon) = match elem.element_type.as_str() {
                "node" => (elem.lat?, elem.lon?),
                "way" => {
                    let center = elem.center?;
                    (center.lat, center.lon)
                },
                _ => return None,
            };
            let location = Geometry::Point((lon, lat).into());
            let (name, city, street) = elem.tags
                .map(|tags| (
                    tags.name,
                    tags.street,
                    tags.city,
                ))
                .unwrap_or((None, None, None));

            Some(MosqueRecord {
                id: RecordId::from(("mosques", elem.id)),
                name,
                location,
                street,
                city,
            })
        }).collect();
    let insert_query = "INSERT INTO mosques $mosques";

    //NOTE: I previously used .create() here and that's is wrong as it just expect us to push a single record to the db while .insert() handles pushing bulk record
    let num_mosques = mosques.len();

    db.query(insert_query)
        .bind(("mosques", mosques))
        .await?;

    Ok(ApiResponse {
        data: Some(format!("Added {} mosques for the region {} {} {} {} successfully", num_mosques, south, west, north, east)),
        error: None,
    })
}

#[server(prefix = "/mosque", endpoint = "fetch-mosques-for-location")]
pub async fn fetch_mosques_for_location(lat: f64, lon: f64) -> Result<ApiResponse<Vec<MosqueApiResponse>>, ServerFnError> {
    let db = leptos_actix::extract::<web::Data<Surreal<Client>>>().await?;
    let point = Geometry::Point((lon, lat).into());
    
    let radius_in_meters = 5000;
    let query = r#"
        SELECT *, geo::distance(location, $point) AS distance FROM mosques
        WHERE geo::distance(location, $point) < $radius
        ORDER BY distance ASC
    "#;
    let mut result = db.query(query)
        .bind(("point", point))
        .bind(("radius", radius_in_meters))
        .await?;

    let mosques: Vec<MosqueDbRes> = result.take(0)?;

    Ok(ApiResponse {
        data: Some(mosques.into_iter().map(|m| m.from()).collect()),
        error: None,
    })
}

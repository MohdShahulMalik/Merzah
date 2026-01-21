use actix_web::http::StatusCode;
use leptos::{prelude::{expect_context, ServerFnError}, server_fn::codec::{Json, PatchJson}, *};
use leptos_actix::ResponseOptions;
use crate::{models::{api_responses::{ApiResponse, MosqueApiResponse}, mosque::PrayerTimesUpdate, user::User}, utils::user_elevation::elevate_user, errors::user_elevation::UserElevationError};

#[cfg(feature = "ssr")]
use tracing::error;
#[cfg(feature = "ssr")]
use surrealdb::{RecordId, Surreal, engine::remote::ws::Client, sql::Geometry};
#[cfg(feature = "ssr")]
use actix_web::web;
#[cfg(feature = "ssr")]
use crate::models::mosque::{MosqueFromOverpass, OverpassResponse, MosqueSearchResult, MosqueRecord};

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
        None => return Err(ServerFnError::ServerError(format!("All Overpass API endpoints failed. Last error: {}", last_error.unwrap()))),
    };
    let data: OverpassResponse = response.json().await?;

    let mosques: Vec<MosqueFromOverpass> = data.elements
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

            Some(MosqueFromOverpass {
                id: RecordId::from(("mosques", elem.id)),
                name,
                location,
                street,
                city,
            })
        }).collect();

    let num_mosques = mosques.len();

    let insert_query = "INSERT INTO mosques $mosques";

    db.query(insert_query)
        .bind(("mosques", mosques))
        .await?;

    Ok(ApiResponse {
        data: Some(format!("Added {} mosques for the region {} {} {} {} successfully", num_mosques, south, west, north, east)),
        error: None,
    })
}

#[server(input = Json, output = Json, prefix = "/mosque", endpoint = "fetch-mosques-for-location")]
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

    let mosques: Vec<MosqueSearchResult> = result.take(0)?;

    Ok(ApiResponse {
        data: Some(mosques.into_iter().map(|m| m.from()).collect()),
        error: None,
    })
}

#[server(input = PatchJson, output = Json, prefix = "/mosque", endpoint = "update-adhan-jamat-times")]
pub async fn update_adhan_jamat_times(mosque_admin: String, mosque_id: String, prayer_times: PrayerTimesUpdate) ->
    Result<ApiResponse<String>, ServerFnError> {
    let db = leptos_actix::extract::<web::Data<Surreal<Client>>>().await?;
    let response_options = expect_context::<ResponseOptions>();

    let mosque_admin: RecordId = match mosque_admin.parse() {
        Ok(id) => id,
        Err(e) => {
            error!(?e, "Failed to parse mosque_admin");
            return Err(ServerFnError::ServerError("Failed to parse mosque_admin".to_string()));
        }
    };

    let check_user_id_response_result = db.query("SELECT id FROM $mosque_admin WHERE ->handles->mosques CONTAINS $mosque_id")
        .bind(("mosque_admin", mosque_admin))
        .bind(("mosque_id", mosque_id.clone()))
        .await;

    if let Err(error) = check_user_id_response_result {
        error!(?error, "Failed to fetch the data from db to check mosque_admin");
        return Err(ServerFnError::ServerError("Failed to fetch the data from db to check the mosque_admin".to_string()));
    }else {
        let mut check_user_id_response = check_user_id_response_result?;
        let check_user_id: Option<User> = check_user_id_response.take(0)?;
        match check_user_id {
            Some(_) => (),
            None => {
                error!("The user trying to update mosque info is not an admin of that mosque");
                response_options.set_status(StatusCode::UNAUTHORIZED);
                return Ok(ApiResponse::error("The user trying to update mosque info is not an admin of that mosque".to_string()));
            }
        }
        
    }

    let mosque_id: RecordId = match mosque_id.parse() {
        Ok(id) => id,
        Err(e) => return Err(ServerFnError::ServerError(format!("Invalid mosque ID: {}", e))),
    };

    db.update::<Option<MosqueRecord>>(mosque_id)
        .merge(prayer_times)
        .await?;
    
    Ok(ApiResponse::data("Successfully updated jamat and adhan times".to_string()))
}

#[server(input = Json, output = Json, prefix = "/mosque", endpoint = "add-admin")]
pub async fn add_admin(
    mosque_supervisor: String,
    requested_user: String,
    mosque_id: String
) -> Result<ApiResponse<String>, ServerFnError> {
    let db = leptos_actix::extract::<web::Data<Surreal<Client>>>().await?;
    let response_options = expect_context::<ResponseOptions>();

    let mosque_supervisor: RecordId = match mosque_supervisor.parse() {
        Ok(id) => id,
        Err(e) => {
            error!(?e, "Failed to parse mosque_supervisor");
            return Err(ServerFnError::ServerError("Failed to parse mosque_supervisor".to_string()));
        }
    };

    let requested_user: RecordId = match requested_user.parse() {
        Ok(id) => id,
        Err(e) => {
            error!(?e, "Failed to parse requested_user");
            return Err(ServerFnError::ServerError("Failed to parse requested_user".to_string()));
        }
    };

    let mosque_id: RecordId = match mosque_id.parse() {
        Ok(id) => id,
        Err(e) => {
            error!(?e, "Failed to parse mosque_id");
            return Err(ServerFnError::ServerError("Failed to parse mosque_id".to_string()));
        }
    };

    let check_mosque_supervisor_id_response_result = db.select(mosque_supervisor.clone())
        .await;

    if let Err(error) = check_mosque_supervisor_id_response_result {
        error!(?error, "Failed to fetch the data from db to check mosque_supervisor");
        return Err(ServerFnError::ServerError("Failed to fetch the data from db to check the mosque_supervisor".to_string()));
    } else {
        let check_mosque_supervisor_id: Option<User> = check_mosque_supervisor_id_response_result?;
        match check_mosque_supervisor_id {
            Some(user) => {
                if !user.is_mosque_supervisor() {
                    error!("The user trying to elevate other user's permission to mosque_admin is not a mosque_supervisor");
                    response_options.set_status(StatusCode::UNAUTHORIZED);
                    return Ok(ApiResponse::error("The user trying to elevate other user's permission to mosque_admin is not a mosque_supervisor".to_string()));
                }
            },
            None => {
                error!("The mosque supervisor trying to elevate permission doesn't exists");
                response_options.set_status(StatusCode::NOT_FOUND);
                return Ok(ApiResponse::error("The user trying to elevate other user's permission to mosque_admin is not a mosque_supervisor".to_string()));
            }
        }
    }

    let relation_query = r#"
        RELATE $requested_user -> handles -> $mosque
            SET granted_by = $mosque_supervisor 
    "#;
    let elevation_result = db.query(relation_query)
        .bind(("requested_user", requested_user))
        .bind(("mosque", mosque_id))
        .bind(("mosque_supervisor", mosque_supervisor))
        .await;

    match elevation_result {
        Ok(_) => (),
        Err(error) => {
            error!(?error, "Failed to elevate the user to a mosque admin due to db error");
            return Err(ServerFnError::ServerError("Failed to elevate the user to a mosque admin due to db error".to_string()))
        }
    }

    Ok(ApiResponse::data("Elevated the user to a requested_user".to_string()))
}

#[server(input = Json, output = Json, prefix = "/mosque", endpoint = "elevate-user-to-mosque-supervisor")]
pub async fn elevate_user_to_mosque_supervisor(app_admin_id: String, user_id: String) -> Result<ApiResponse<String>, ServerFnError> {
    let response_option = expect_context::<ResponseOptions>();

    let app_admin_id: RecordId = match app_admin_id.parse() {
        Ok(id) => id,
        Err(error) => {
            error!(?error, "Failed to parse app_admin_id");
            return Err(ServerFnError::ServerError("Failed to parse the app admin's id".to_string()))
        }
    };

    let user_id: RecordId = match user_id.parse() {
        Ok(id) => id,
        Err(error) => {
            error!(?error, "Failed to parse user_id");
            response_option.set_status(StatusCode::NOT_FOUND);
            return Err(ServerFnError::ServerError("Failed to parse the requested users's id".to_string()))
        }
    };

    let db = leptos_actix::extract::<web::Data<Surreal<Client>>>().await?;

    let result = elevate_user(app_admin_id, user_id, "mosque_supervisor".to_string(), &db).await;

    let elevation_error = match result {
        Ok(success_msg) => return Ok(ApiResponse::data(success_msg)),
        Err(e) => e,
    };

    let (status, msg) = match elevation_error {
        UserElevationError::Unauthorized => (
            StatusCode::UNAUTHORIZED,
            "You are not authorized to perform this action".to_string(),
        ),
        UserElevationError::AdminNotFound => (
            StatusCode::UNAUTHORIZED,
            "Admin user not found".to_string(),
        ),
        UserElevationError::TargetUserNotFound => (
            StatusCode::NOT_FOUND,
            "User to elevate not found".to_string(),
        ),
        UserElevationError::AlreadyElevated(role) => (
            StatusCode::CONFLICT,
            format!("User is already a {}", role),
        ),
        UserElevationError::SelfElevationNotAllowed => (
            StatusCode::BAD_REQUEST,
            "You cannot elevate yourself".to_string(),
        ),
        UserElevationError::DatabaseError(db_err) => {
            error!(?db_err, "Database error during user elevation");
            return Err(ServerFnError::ServerError("Internal server error during elevation".to_string()));
        }
    };

    error!("User elevation failed: {}", msg);
    response_option.set_status(status);
    Ok(ApiResponse::error(msg))
}

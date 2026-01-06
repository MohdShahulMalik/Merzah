use leptos::{prelude::ServerFnError, server_fn::codec::Json, *};
use crate::models::{api_responses::ApiResponse, mosque::{Mosque, MosquesResponse}};

#[server(input=Json, output=Json, prefix = "/mosques", endpoint = "add-mosque-of-region")]
pub async fn add_mosques_of_region(
    south: f64,
    west: f64,
    north: f64,
    east: f64,
) -> Result<ApiResponse<String>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use surrealdb::{RecordId, Surreal};
        use surrealdb::engine::remote::ws::Client;
        use surrealdb::sql::Geometry;
        use actix_web::web;

        let db = leptos_actix::extract::<web::Data<Surreal<Client>>>().await?;

        let query = format!(
            r#"[out:json];
            (
                node["amenity"="place_of_worship"]["religion"="muslim"]["building"="mosque"]({},{},{},{});
                way["amenity"="place_of_worship"]["religion"="muslim"]["building"="mosque"]({},{},{},{});
            )
            out center;"#,
        south, west, north, east,
        south, west, north, east
        );

        let client = reqwest::Client::new();
        let response = client
            .post("https://overpass-api.de/api/interpreter")
            .body(query)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
            return Err(ServerFnError::ServerError(format!("Fetching data returned non 200 status, status: {}, response: {}", status, body)));
        }

        let data: MosquesResponse = response.json().await?;

        let mosques: Vec<Mosque> = data.elements
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

                Some(Mosque {
                    id: RecordId::from(("mosques", elem.id)),
                    name,
                    location,
                    street,
                    city,
                })
            }).collect();

        db.create::<Option<Mosque>>("mosques")
            .content(mosques)
            .await?;

        Ok(ApiResponse {
            data: Some(format!("Added mosques for the region {} {} {} {} successfully", south, west, north, east)),
            error: None,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (south, west, north, east);
        unreachable!()
    }
}

#[server(prefix = "/mosque", endpoint = "fetch-mosques-for-location")]
pub async fn fetch_mosques_for_location(lat: f64, lon: f64) -> Result<ApiResponse<Vec<Mosque>>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use surrealdb::Surreal;
        use surrealdb::engine::remote::ws::Client;
        use surrealdb::sql::Geometry;
        use actix_web::web;

        let db = leptos_actix::extract::<web::Data<Surreal<Client>>>().await?;
        let point = Geometry::Point((lon, lat).into());
        
        let radius_in_meters = 5000;
        let query = r#"
            SELECT * FROM mosques
            WHERE geo::distance(location, $point) < $radius
            ORDER BY geo::distance(location, $point) ASC
        "#;
        let mut result = db.query(query)
            .bind(("point", point))
            .bind(("radius", radius_in_meters))
            .await?;

        let mosques: Vec<Mosque> = result.take(0)?;

        Ok(ApiResponse {
            data: Some(mosques),
            error: None,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (lat, lon);
        unreachable!()
    }
}

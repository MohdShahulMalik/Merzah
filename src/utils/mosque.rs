use std::collections::HashMap;
use std::collections::HashSet;

use surrealdb::RecordId;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Geometry;

use crate::models::api_responses::MosqueResponse;
use crate::models::mosque::MosqueFromOverpass;
use crate::models::mosque::MosqueSearchResult;
use crate::models::mosque::OverpassResponse;
use crate::models::user::UserIdentifier;
use crate::models::user::UserIdentifierOnClient;

/// Turns the answer of the Overpass API into mosque records that can be
/// written to the database.
pub fn parse_overpass_mosques(data: OverpassResponse) -> Vec<MosqueFromOverpass> {
    data.elements
        .into_iter()
        .filter_map(|elem| {
            let (lat, lon) = match elem.element_type.as_str() {
                "node" => (elem.lat?, elem.lon?),
                "way" => {
                    let center = elem.center?;
                    (center.lat, center.lon)
                }
                _ => return None,
            };
            let location = Geometry::Point((lon, lat).into());
            let (name, street, city, wikimedia_commons) = elem
                .tags
                .map(|tags| (tags.name, tags.street, tags.city, tags.wikimedia_commons))
                .unwrap_or((None, None, None, None));

            let cover_img = match wikimedia_commons {
                Some(ref filename) if !filename.is_empty() => Some(format!(
                    "https://commons.wikimedia.org/wiki/Special:FilePath/{}",
                    filename
                )),
                _ => None,
            };

            Some(MosqueFromOverpass {
                id: RecordId::from(("mosques", elem.id)),
                name,
                location,
                street,
                city,
                cover_img,
            })
        })
        .collect()
}

pub async fn enrich_mosques_with_contacts(
    mosques: Vec<MosqueSearchResult>,
    db: &Surreal<Client>,
) -> Result<Vec<MosqueResponse>, surrealdb::Error> {
    let mut user_ids = HashSet::new();
    for mosque in &mosques {
        if let Some(ref imam) = mosque.imam {
            user_ids.insert(imam.id.to_string());
        }
        if let Some(ref muazzin) = mosque.muazzin {
            user_ids.insert(muazzin.id.to_string());
        }
    }

    let user_ids_vec: Vec<String> = user_ids.into_iter().collect();
    let mut id_to_contacts: HashMap<RecordId, Vec<UserIdentifierOnClient>> = HashMap::new();

    if !user_ids_vec.is_empty() {
        let mut ident_res = db
            .query("SELECT * FROM user_identifier WHERE user IN $user_ids")
            .bind(("user_ids", user_ids_vec))
            .await?;
        let identifiers: Vec<UserIdentifier> = ident_res.take(0)?;

        for ident in identifiers {
            id_to_contacts
                .entry(ident.user)
                .or_default()
                .push(UserIdentifierOnClient::new(
                    ident.identifier_type,
                    ident.identifier_value,
                ));
        }
    }

    let mosque_responses = mosques
        .into_iter()
        .map(|m| {
            let imam_id = m.imam.as_ref().map(|u| u.id.clone());
            let muazzin_id = m.muazzin.as_ref().map(|u| u.id.clone());
            let mut res = m.from();

            if let Some(id) = imam_id {
                if let Some(contacts) = id_to_contacts.get(&id) {
                    res.imam_contact = contacts.clone();
                }
            }

            if let Some(id) = muazzin_id {
                if let Some(contacts) = id_to_contacts.get(&id) {
                    res.muazzin_contact = contacts.clone();
                }
            }

            res
        })
        .collect();

    Ok(mosque_responses)
}

use axum::{
    extract::{State,Path},
    Json,
    http::StatusCode
};
use std::sync::Arc;

use crate::{
    models::contact::{self, Contact, NewContact, PhoneSearchResult, UpdateContact},
    repository::ContactRepository,
};

pub type Repo = Arc<dyn ContactRepository>;

pub async fn list_contacts(State(repo): State<Repo>) -> Json<Vec<Contact>> {
    let contacts = repo.list_contact().await;
    Json(contacts)
}

pub async fn new_contact(
    State(repo):State<Repo>,
    Json(payload):Json<NewContact>
)->Result<Json<Contact>,StatusCode> {
    if payload.name.len()<=2||payload.phone.len()!=10{
        return Err(StatusCode::BAD_REQUEST);
    }
    let contact=repo.create_contact(payload).await;
    Ok(Json((contact)))
}

pub async fn edit_contact(
    State(repo): State<Repo>,
    Path(name): Path<String>,
    Json(payload): Json<UpdateContact>,
) -> Result<Json<Contact>, StatusCode> {
    if let Some(ref phone) = payload.phone {
        if phone.len() != 10 {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    match repo.update_contact_by_name(&name, payload).await {
        Some(contact) => Ok(Json(contact)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn search_contact_by_phone(
    State(repo): State<Repo>,
    Path(phone): Path<String>,
) -> Json<Vec<PhoneSearchResult>> {
    println!("Phone: {} ",phone);
    let results = repo.search_by_phone(&phone).await;
    let mapped: Vec<PhoneSearchResult> = results
        .into_iter()
        .map(|c| PhoneSearchResult {
            name: c.name,
            matched: phone.clone(),
        })
        .collect();

    Json(mapped)
}

pub async fn delete_contact_by_name(
    Path(name): Path<String>,
    State(repo): State<Repo>,
) -> Json<serde_json::Value> {
    let deleted = repo.delete_by_name(name).await;
    if deleted {
        Json(serde_json::json!({ "status": "deleted" }))
    } else {
        Json(serde_json::json!({ "status": "not_found" }))
    }
}

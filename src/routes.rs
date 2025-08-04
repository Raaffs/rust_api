use axum::{Router, routing::{get, post, put,delete}};
use crate::handlers::contact_handler::{
    list_contacts, new_contact, edit_contact, search_contact_by_phone, delete_contact_by_name, Repo,
};

pub fn create_routes(repo: Repo) -> Router {
    Router::new()
        .route("/contacts", get(list_contacts).post(new_contact))
        .route("/contacts/:name", put(edit_contact))
        .route("/contacts/search/:phone", get(search_contact_by_phone))
        .route("/delete/:name", delete(delete_contact_by_name)) 
        .with_state(repo)
}

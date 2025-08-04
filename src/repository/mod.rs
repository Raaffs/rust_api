pub mod mock_repository;
pub mod pg_repository;
use async_trait::async_trait;
use crate::models::contact::{Contact,NewContact,UpdateContact};

#[async_trait]
pub trait ContactRepository: Send + Sync + 'static{
    async fn list_contact(&self)->Vec<Contact>;
    async fn create_contact(&self, new_contact:NewContact)->Contact;
    async fn update_contact_by_name(&self, name:&str, update:UpdateContact)->Option<Contact>;
    async fn search_by_phone(&self, phone: &str) -> Vec<Contact>;
    async fn delete_by_name(&self, name: String) -> bool; 
}
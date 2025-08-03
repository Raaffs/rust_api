use std::{fs::OpenOptions, sync::{Arc, Mutex}};
use uuid::Uuid;
use async_trait::async_trait;

use crate::{models::contact::{Contact,NewContact,UpdateContact}, repository::ContactRepository};

pub struct MockContactRepository{
    data: Arc<Mutex<Vec<Contact>>>
}

impl MockContactRepository{
    pub fn new()->Self{
        return Self {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl ContactRepository for MockContactRepository{
    async fn list_contact(&self)->Vec<Contact>{
        let db=self.data.lock().unwrap();
        return db.clone()
    }
    async fn create_contact(&self, new_contact: NewContact)->Contact{
        let mut db=self.data.lock().unwrap();
        let contact=Contact{
            id: Uuid::new_v4().to_string(),
            name: new_contact.name,
            phone: new_contact.phone,
        };
        db.push(contact.clone());
        return contact
    }
    async fn update_contact_by_name(&self, name: &str, update: UpdateContact)->Option<Contact>{
        let mut db = self.data.lock().unwrap();
        if let Some(c)=db.iter_mut().find(|c| c.name==name){
            if let Some(phone)=update.phone{
                c.phone=phone
            }
            return Some(c.clone())
        }
        return  None;
    }
    async fn search_by_phone(&self, phone: &str) -> Vec<Contact> {
        let db = self.data.lock().unwrap();
        println!("Searching for phone: {}", phone);
        println!("DB has {} contacts", db.len());

        for c in db.iter() {
            println!("DB Contact => {:?}", c);
        }

        let matched: Vec<Contact> = db
            .iter()
            .filter(|c| c.phone.contains(phone))
            .cloned()
            .collect();

        println!("Found {} matches", matched.len());
        for m in &matched {
            println!("Matched => {:?}", m);
        }
        matched
    }
    async fn delete_by_name(&self, name: String) -> bool {
        let mut db = self.data.lock().unwrap();
        let len_before = db.len();
        db.retain(|c| c.name != name);
        len_before != db.len() // true if something was deleted
    }

}
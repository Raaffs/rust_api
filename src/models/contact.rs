use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Contact{
    pub id: String,
    pub name: String,
    pub phone: String,
}

#[derive(Deserialize,Clone)]
pub struct NewContact{
    pub name:String,
    pub phone:String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateContact {
    pub phone: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct PhoneSearchResult {
    pub name: String,
    pub matched: String,
}


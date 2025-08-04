    use serde::{Serialize, Deserialize};
    use sqlx::FromRow;
    #[derive(Serialize, Deserialize, Clone,Debug, FromRow)]
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


use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use crate::models::contact::{Contact, NewContact, UpdateContact};
use super::ContactRepository;

#[derive(Clone)]
pub struct PostgresContactRepository {
    pub pool: PgPool,
}

impl PostgresContactRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ContactRepository for PostgresContactRepository {
    async fn list_contact(&self) -> Vec<Contact> {
        sqlx::query_as::<_, Contact>("SELECT * FROM contacts")
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default()
    }

    async fn create_contact(&self, new_contact: NewContact) -> Contact {
        sqlx::query_as::<_, Contact>(
            "INSERT INTO contacts (name, phone) VALUES ($1, $2) RETURNING *",
        )
        .bind(new_contact.name)
        .bind(new_contact.phone)
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    async fn update_contact_by_name(&self, name: &str, update: UpdateContact) -> Option<Contact> {
        if let Some(phone) = update.phone {
            sqlx::query_as::<_, Contact>(
                "UPDATE contacts SET phone = $1 WHERE name = $2 RETURNING *",
            )
            .bind(phone)
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()
        } else {
            None
        }
    }

    async fn search_by_phone(&self, phone: &str) -> Vec<Contact> {
        let pattern = format!("%{}%", phone);
        sqlx::query_as::<_, Contact>(
            "SELECT * FROM contacts WHERE phone LIKE $1",
        )
        .bind(pattern)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default()
    }

    async fn delete_by_name(&self, name: String) -> bool {
        let result = sqlx::query("DELETE FROM contacts WHERE name = $1")
            .bind(name)
            .execute(&self.pool)
            .await;

        result.map(|r| r.rows_affected() > 0).unwrap_or(false)
    }
}

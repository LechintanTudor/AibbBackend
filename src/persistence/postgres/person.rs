use crate::domain::Person;
use crate::persistence::error::RepositoryError;
use crate::persistence::interface::PersonRepository;
use async_trait::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;

/// (name) -> (id, name)
const SQL_PERSON_INSERT: &str = include_str!("sql/person/insert.sql");
/// (id, name) -> u64
const SQL_PERSON_UPDATE: &str = include_str!("sql/person/update.sql");
/// (id) -> (id, name)
const SQL_PERSON_REMOVE: &str = include_str!("sql/person/remove.sql");
/// (id) -> (id, name)
const SQL_PERSON_GET: &str = include_str!("sql/person/get.sql");
/// () -> [(id, name)]
const SQL_PERSON_GET_ALL: &str = include_str!("sql/person/get_all.sql");
/// () -> ()
const SQL_PERSON_CLEAR: &str = include_str!("sql/person/clear.sql");

#[derive(Clone)]
pub struct PgPersonRepository {
    pool: PgPool,
}

impl PgPersonRepository {
    pub fn new(pool: PgPool) -> Arc<dyn PersonRepository> {
        Arc::new(Self { pool })
    }
}

#[async_trait]
impl PersonRepository for PgPersonRepository {
    async fn create(&self, person: &Person) -> Result<Person, RepositoryError> {
        sqlx::query_as::<_, Person>(SQL_PERSON_INSERT)
            .bind(&person.name)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, person: &Person) -> Result<bool, RepositoryError> {
        sqlx::query(SQL_PERSON_UPDATE)
            .bind(person.id)
            .bind(&person.name)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected() == 1)
    }

    async fn remove(&self, id: i32) -> Result<Option<Person>, RepositoryError> {
        sqlx::query_as::<_, Person>(SQL_PERSON_REMOVE).bind(id).fetch_optional(&self.pool).await
    }

    async fn get(&self, id: i32) -> Result<Option<Person>, RepositoryError> {
        sqlx::query_as::<_, Person>(SQL_PERSON_GET).bind(id).fetch_optional(&self.pool).await
    }

    async fn get_all(&self) -> Result<Vec<Person>, RepositoryError> {
        sqlx::query_as::<_, Person>(SQL_PERSON_GET_ALL).fetch_all(&self.pool).await
    }

    async fn clear(&self) -> Result<u64, RepositoryError> {
        sqlx::query(SQL_PERSON_CLEAR).execute(&self.pool).await.map(|r| r.rows_affected())
    }
}

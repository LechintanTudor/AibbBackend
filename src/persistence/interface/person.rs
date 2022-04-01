use crate::domain::Person;
use crate::persistence::error::RepositoryError;
use async_trait::async_trait;

#[async_trait]
pub trait PersonRepository: Send + Sync + 'static {
    async fn create(&self, person: &Person) -> Result<Person, RepositoryError>;

    async fn update(&self, person: &Person) -> Result<bool, RepositoryError>;

    async fn remove(&self, id: i32) -> Result<Option<Person>, RepositoryError>;

    async fn get(&self, id: i32) -> Result<Option<Person>, RepositoryError>;

    async fn get_all(&self) -> Result<Vec<Person>, RepositoryError>;

    async fn clear(&self) -> Result<u64, RepositoryError>;
}

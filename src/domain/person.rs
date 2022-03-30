use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Person {
    pub id: i32,
    pub name: String,
}

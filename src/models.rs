use diesel::Insertable;
use diesel::Queryable;
use serde::Deserialize;
use serde::Serialize;
use crate::schema::rustaceans;

#[derive(Deserialize, Serialize, Queryable)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

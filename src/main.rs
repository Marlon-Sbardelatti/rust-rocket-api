#[macro_use]
extern crate rocket;
mod auth;
mod models;
mod schema;
mod repositories;
use auth::BasicAuth;
// use diesel::query_dsl::methods::{LimitDsl, OrderDsl};
// use diesel::ExpressionMethods;
use models::NewRustacean;
use models::Rustacean;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use rocket_sync_db_pools::database;
use repositories::RustaceanRepository;

#[database("sqlite")]

struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let rustaceans = RustaceanRepository::find_multiple(c, 1000)
            .expect("Error getting all rustaceans");
        json!(rustaceans)
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let rustacean = RustaceanRepository::find(c, id)
            .expect("Error finding rustacean");
        json!(rustacean)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = RustaceanRepository::create(c, new_rustacean.into_inner())
            .expect("Error inserting rustacean");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    auth: BasicAuth,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Value {
    db.run(move |c| {
        let result = RustaceanRepository::save(c, id, rustacean.into_inner())
            .expect("Error updating rustacean");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .expect("Error deleting rustacean");
        status::NoContent
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}
#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized")
}
#[catch(422)]
fn unprocessable_entity() -> Value {
    json!("Unprocessable Entity")
}
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean,
            ],
        )
        .register(
            "/",
            catchers![not_found, unauthorized, unprocessable_entity],
        )
        .attach(DbConn::fairing())
        .launch()
        .await;
}

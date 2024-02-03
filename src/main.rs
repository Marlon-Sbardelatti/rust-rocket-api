#[macro_use]
extern crate rocket;
mod auth;
mod models;
mod repositories;
mod schema;
use auth::BasicAuth;
use diesel::result::Error::NotFound;
// use diesel::query_dsl::methods::{LimitDsl, OrderDsl};
// use diesel::ExpressionMethods;
use models::NewRustacean;
use models::Rustacean;
use repositories::RustaceanRepository;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_sync_db_pools::database;

#[database("sqlite")]

struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        match RustaceanRepository::find_multiple(c, 1000) {
            Ok(rustaceans) => Ok(json!(rustaceans)),
            Err(e) => Err(Custom(Status::InternalServerError, json!(e.to_string()))),
        }
        // RustaceanRepository::find_multiple(c, 1000)
        //     .map(|rustaceans| json!(rustaceans))
        //     .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        // let rustacean = RustaceanRepository::find(c, id)
        //     .expect("Error finding rustacean");
        // json!(rustacean)
        match RustaceanRepository::find(c, id) {
            Ok(rustacean) => Ok(json!(rustacean)),
            Err(e) => match e {
                NotFound => Err(Custom(Status::NotFound, json!(e.to_string()))),
                _ => Err(Custom(Status::InternalServerError, json!(e.to_string()))),
            },
        }
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        // let result = RustaceanRepository::create(c, new_rustacean.into_inner())
        //     .expect("Error inserting rustacean");
        // json!(result)
        match RustaceanRepository::create(c, new_rustacean.into_inner()) {
            Ok(created_rustacean) => Ok(json!(created_rustacean)),
            Err(e) => Err(Custom(Status::InternalServerError, json!(e.to_string()))),
        }
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    auth: BasicAuth,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        // let result = RustaceanRepository::save(c, id, rustacean.into_inner())
        //     .expect("Error updating rustacean");
        // json!(result)
        match RustaceanRepository::save(c, id, rustacean.into_inner()) {
            Ok(rustacean) => Ok(json!(rustacean)),
            Err(e) => Err(Custom(Status::InternalServerError, json!(e.to_string()))),
        }
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(
    id: i32,
    auth: BasicAuth,
    db: DbConn,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        if let Err(e) = RustaceanRepository::find(c, id) {
            return Err(Custom(Status::NotFound, json!(e.to_string())));
        } else {
            match RustaceanRepository::delete(c, id) {
                Ok(_) => Ok(status::NoContent),
                Err(e) => Err(Custom(Status::InternalServerError, json!(e.to_string()))),
            }
        }
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

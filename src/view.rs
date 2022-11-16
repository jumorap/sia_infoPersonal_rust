// insert into VIVIENDA(tipo, direccion, departamento, codigo_postal, telefono, estrato, nombre_usuario) values("", ...);
// insert into RESPONSABLE(nombre, tipo_doc, numero_doc, telefono, nombre_usuario) values("", ...);

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rusqlite::Connection;
use std::{string::String, vec};

mod model;
mod controller;


#[get("/user/<user>")]
fn fetch_users(user: String) -> Result<Json<model::DataList>, String> {
    controller::fetch_users(user)
}

#[put("/update", format = "json", data = "<user_data>")]
fn update_item(user_data: Json<model::UserItemUpdate>) -> Result<Json<model::StatusMessage>, String> {
    controller::update_item(user_data)
}

#[post("/new", format = "json", data = "<user_data>")]
fn insert(user_data: Json<model::UserItem>) -> Result<Json<model::StatusMessage>, String> {
    controller::insert(user_data)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![fetch_users, update_item, insert])
}

// insert into VIVIENDA(tipo, direccion, departamento, codigo_postal, telefono, estrato, nombre_usuario) values("", ...);
// insert into RESPONSABLE(nombre, tipo_doc, numero_doc, telefono, nombre_usuario) values("", ...);

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::Request;
use rusqlite::Connection;
use serde::{Serialize, Deserialize};
use std::{string::String, vec};


#[derive(Serialize, Deserialize, Clone)]
struct StatusMessage {
    message: String,
}

#[derive(Serialize)]
pub struct DataList {
    items: Vec<UserItem>,
}

#[derive(Serialize)]
pub struct DataResponsableList {
    items: Vec<ResponsablesItem>,
}

#[derive(Serialize)]
pub struct DataViviendaList {
    items: Vec<ViviendaItem>,
}

#[derive(Serialize, Deserialize, Clone)]
struct UserItem {
    nombre_usuario: String,
    nombre_completo: String,
    documento_identidad: String,
    lugar_expedicion: String,
    sexo: String,
    etnia: String,
    email_personal: String,
    email_institucional: String,
    telefono_movil: String,
    fecha_nacimiento: String,
    lugar_nacimiento: String,
    nacionalidad: String,
    tipo_sangre: String,
    eps: String,
    situacion_militar: String,
    responsables: Vec<ResponsablesItem>,
    vivienda: Vec<ViviendaItem>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ResponsablesItem {
    responsable_nombre: String,
    responsable_tipo_doc: String,
    responsable_numero_doc: String,
    responsable_telefono: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct ViviendaItem {
    vivienda_tipo: String,
    vivienda_direccion: String,
    vivienda_departamento: String,
    vivienda_codigo_postal: String,
    vivienda_telefono: String,
    vivienda_estrato: String,
}

fn fetch_responsables(user: String) -> Result<Json<DataResponsableList>, String> {
    let db_connection = match Connection::open("./SIA_INFO_PERSONAL_DB.db") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database"))
    };

    let mut statement = match db_connection.prepare(&*format!("select * from RESPONSABLE where nombre_usuario = '{}';", user)) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(ResponsablesItem {
            responsable_nombre: row.get(0)?,
            responsable_tipo_doc: row.get(1)?,
            responsable_numero_doc: row.get(2)?,
            responsable_telefono: row.get(3)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(DataResponsableList { items })),
                Err(_) => Err("Could not collect items".into()),
            }
        }
        Err(_) => Err("Failed to fetch items".into()),
    }
}

fn fetch_vivienda(user: String) -> Result<Json<DataViviendaList>, String> {
    let db_connection = match Connection::open("./SIA_INFO_PERSONAL_DB.db") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database"))
    };

    let mut statement = match db_connection.prepare(&*format!("select * from VIVIENDA where nombre_usuario = '{}';", user)) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(ViviendaItem {
            vivienda_tipo: row.get(0)?,
            vivienda_direccion: row.get(1)?,
            vivienda_departamento: row.get(2)?,
            vivienda_codigo_postal: row.get(3)?,
            vivienda_telefono: row.get(4)?,
            vivienda_estrato: row.get(5)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(DataViviendaList { items })),
                Err(_) => Err("Could not collect items".into()),
            }
        }
        Err(_) => Err("Failed to fetch items".into()),
    }
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[get("/user/<user>")]
fn fetch_users(user: String) -> Result<Json<DataList>, String> {
    let db_connection = match Connection::open("./SIA_INFO_PERSONAL_DB.db") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database"))
    };

    let mut statement = match db_connection.prepare(&*format!("select * from USUARIO where nombre_usuario = '{}';", user)) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(UserItem {
            nombre_usuario: row.get(0)?,
            nombre_completo: row.get(1)?,
            documento_identidad: row.get(2)?,
            lugar_expedicion: row.get(3)?,
            sexo: row.get(4)?,
            etnia: row.get(5)?,
            email_personal: row.get(6)?,
            email_institucional: row.get(7)?,
            telefono_movil: row.get(8)?,
            fecha_nacimiento: row.get(9)?,
            lugar_nacimiento: row.get(10)?,
            nacionalidad: row.get(11)?,
            tipo_sangre: row.get(12)?,
            eps: row.get(13)?,
            situacion_militar: row.get(14)?,
            responsables: vec![fetch_responsables(user.clone()).unwrap().into_inner().items[0].clone(), fetch_responsables(user.clone()).unwrap().into_inner().items[1].clone()],
            vivienda: vec![fetch_vivienda(user.clone()).unwrap().into_inner().items[0].clone(), fetch_vivienda(user.clone()).unwrap().into_inner().items[1].clone()],
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(DataList { items })),
                Err(_) => Err("Could not collect items".into()),
            }
        }
        Err(_) => Err("Failed to fetch items".into()),
    }
}


#[put("/update", format = "json", data = "<user_data>")]
fn update_item(user_data: Json<UserItem>) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("./SIA_INFO_PERSONAL_DB.db") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database"))
    };

    // Afect the USUARIOS table
    let param = format!("UPDATE USUARIO SET lugar_expedicion = '{}', email_personal = '{}', telefono_movil = '{}', eps = '{}', situacion_militar = '{}' WHERE nombre_usuario = '{}';",
                        user_data.lugar_expedicion, user_data.email_personal, user_data.telefono_movil, user_data.eps, user_data.situacion_militar, user_data.nombre_usuario);
    let mut statement = match db_connection.prepare(&*param) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.execute(rusqlite::NO_PARAMS);

    // Afect the VIVIENDA table
    // Use a for cycle to move through the 2 items
    for i in 0..2 {
        if user_data.vivienda[i].vivienda_tipo == "a" {
            let param = format!("UPDATE VIVIENDA SET direccion = '{}', departamento = '{}', codigo_postal = '{}', telefono = '{}', estrato = '{}' WHERE nombre_usuario = '{}' and tipo = 'a';",
                                user_data.vivienda[i].vivienda_direccion, user_data.vivienda[i].vivienda_departamento, user_data.vivienda[i].vivienda_codigo_postal, user_data.vivienda[i].vivienda_telefono, user_data.vivienda[i].vivienda_estrato, user_data.nombre_usuario);
            let mut statement = match db_connection.prepare(&*param) {
                Ok(statement) => statement,
                Err(_) => return Err("Failed to prepare query".into()),
            };

            let results = statement.execute(rusqlite::NO_PARAMS);
        }
    }

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows afected!", rows_affected),
        })),
        Err(_) => Err("Failed to insert item".into()),
    }
}


#[post("/new", format = "json", data = "<user_data>")]
fn insert(user_data: Json<UserItem>) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("./SIA_INFO_PERSONAL_DB.db") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database"))
    };

    // Afect the RESPONSABLES table
    // Use a for cycle to move through the 2 items
    for i in 0..2 {
        let param = format!("INSERT INTO RESPONSABLES (nombre, tipo_doc, numero_doc, telefono, nombre_usuario) VALUES ('{}', '{}', '{}', '{}', '{}');",
                            user_data.responsables[i].responsable_nombre, user_data.responsables[i].responsable_tipo_doc, user_data.responsables[i].responsable_numero_doc, user_data.responsables[i].responsable_telefono, user_data.nombre_usuario);
        let mut statement = match db_connection.prepare(&*param) {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        };

        let results = statement.execute(rusqlite::NO_PARAMS);
    }

    // Afect the VIVIENDA table
    // Use a for cycle to move through the 2 items
    for i in 0..2 {
        let param = format!("INSERT INTO VIVIENDA (tipo, direccion, departamento, codigo_postal, telefono, estrato, nombre_usuario) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}');",
                            user_data.vivienda[i].vivienda_tipo, user_data.vivienda[i].vivienda_direccion, user_data.vivienda[i].vivienda_departamento, user_data.vivienda[i].vivienda_codigo_postal, user_data.vivienda[i].vivienda_telefono, user_data.vivienda[i].vivienda_estrato, user_data.nombre_usuario);
        let mut statement = match db_connection.prepare(&*param) {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        };

        let results = statement.execute(rusqlite::NO_PARAMS);
    }

    // Afect the USUARIOS table
    let param = format!("INSERT INTO USUARIO (nombre_usuario, nombre_completo, documento_identidad, lugar_expedicion, sexo, etnia, email_personal, email_institucional, telefono_movil, fecha_nacimiento, lugar_nacimiento, nacionalidad, tipo_sangre, eps, situacion_militar) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');",
                        user_data.nombre_usuario, user_data.nombre_completo, user_data.documento_identidad, user_data.lugar_expedicion, user_data.sexo, user_data.etnia, user_data.email_personal, user_data.email_institucional, user_data.telefono_movil, user_data.fecha_nacimiento, user_data.lugar_nacimiento, user_data.nacionalidad, user_data.tipo_sangre, user_data.eps, user_data.situacion_militar);
    let mut statement = match db_connection.prepare(&*param) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.execute(rusqlite::NO_PARAMS);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows afected!", rows_affected),
        })),
        Err(_) => Err("Failed to insert item".into()),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, fetch_users, update_item, insert])
}

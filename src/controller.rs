use rocket::serde::json::Json;
use rusqlite::Connection;
use std::{string::String};

use crate::model;


pub fn fetch_responsables(user: String) -> Result<Json<model::DataResponsableList>, String> {
    let db_connection = match Connection::open("./SIA_INFO_PERSONAL_DB.db") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database"))
    };

    let mut statement = match db_connection.prepare(&*format!("select * from RESPONSABLE where nombre_usuario = '{}';", user)) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.query_map([], |row| {
        Ok(model::ResponsablesItem {
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
                Ok(items) => Ok(Json(model::DataResponsableList { items })),
                Err(_) => Err("Could not collect items".into()),
            }
        }
        Err(_) => Err("Failed to fetch items".into()),
    }
}

pub fn fetch_vivienda(user: String) -> Result<Json<model::DataViviendaList>, String> {
    let db_connection = match Connection::open("./SIA_INFO_PERSONAL_DB.db") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database"))
    };

    let mut statement = match db_connection.prepare(&*format!("select * from VIVIENDA where nombre_usuario = '{}';", user)) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.query_map([], |row| {
        Ok(model::ViviendaItem {
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
                Ok(items) => Ok(Json(model::DataViviendaList { items })),
                Err(_) => Err("Could not collect items".into()),
            }
        }
        Err(_) => Err("Failed to fetch items".into()),
    }
}

pub fn fetch_users(user: String) -> Result<Json<model::DataList>, String> {
    let db_connection = match Connection::open("./SIA_INFO_PERSONAL_DB.db") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database"))
    };

    let mut statement = match db_connection.prepare(&*format!("select * from USUARIO where nombre_usuario = '{}';", user)) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.query_map([], |row| {
        Ok(model::UserItem {
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
                Ok(items) => Ok(Json(model::DataList { items })),
                Err(_) => Err("Could not collect items".into()),
            }
        }
        Err(_) => Err("Failed to fetch items".into()),
    }
}

pub fn update_item(user_data: Json<model::UserItemUpdate>) -> Result<Json<model::StatusMessage>, String> {
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

    let results = statement.execute([]);

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

            let _results = statement.execute([]);
        }
    }

    match results {
        Ok(rows_affected) => Ok(Json(model::StatusMessage {
            message: format!("{} rows afected!", rows_affected),
        })),
        Err(_) => Err("Failed to insert item".into()),
    }
}

pub fn insert(user_data: Json<model::UserItem>) -> Result<Json<model::StatusMessage>, String> {
    let db_connection = match Connection::open("./SIA_INFO_PERSONAL_DB.db") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database"))
    };

    for i in 0..2 {
        // Afect the RESPONSABLES table
        // Use a for cycle to move through the 2 items
        let param = format!("INSERT INTO RESPONSABLE (nombre, tipo_doc, numero_doc, telefono, nombre_usuario) VALUES ('{}', '{}', '{}', '{}', '{}');",
                            user_data.responsables[i].responsable_nombre, user_data.responsables[i].responsable_tipo_doc, user_data.responsables[i].responsable_numero_doc, user_data.responsables[i].responsable_telefono, user_data.nombre_usuario);
        let mut statement = match db_connection.prepare(&*param) {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        };

        let _results = statement.execute([]);

        // Afect the VIVIENDA table
        // Use a for cycle to move through the 2 items
        let param = format!("INSERT INTO VIVIENDA (tipo, direccion, departamento, codigo_postal, telefono, estrato, nombre_usuario) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}');",
                            user_data.vivienda[i].vivienda_tipo, user_data.vivienda[i].vivienda_direccion, user_data.vivienda[i].vivienda_departamento, user_data.vivienda[i].vivienda_codigo_postal, user_data.vivienda[i].vivienda_telefono, user_data.vivienda[i].vivienda_estrato, user_data.nombre_usuario);
        let mut statement = match db_connection.prepare(&*param) {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        };

        let _results = statement.execute([]);
    }

    // Afect the USUARIOS table
    let param = format!("INSERT INTO USUARIO (nombre_usuario, nombre_completo, documento_identidad, lugar_expedicion, sexo, etnia, email_personal, email_institucional, telefono_movil, fecha_nacimiento, lugar_nacimiento, nacionalidad, tipo_sangre, eps, situacion_militar) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');",
                        user_data.nombre_usuario, user_data.nombre_completo, user_data.documento_identidad, user_data.lugar_expedicion, user_data.sexo, user_data.etnia, user_data.email_personal, user_data.email_institucional, user_data.telefono_movil, user_data.fecha_nacimiento, user_data.lugar_nacimiento, user_data.nacionalidad, user_data.tipo_sangre, user_data.eps, user_data.situacion_militar);
    let mut statement = match db_connection.prepare(&*param) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.execute([]);

    match results {
        Ok(rows_affected) => Ok(Json(model::StatusMessage {
            message: format!("{} rows afected!", rows_affected),
        })),
        Err(_) => Err("Failed to insert item".into()),
    }
}

// use rocket::Request;
use serde::{Serialize, Deserialize};
use std::{string::String};


#[derive(Serialize, Deserialize, Clone)]
pub struct StatusMessage {
    pub message: String,
}

#[derive(Serialize)]
pub struct DataList {
    pub items: Vec<UserItem>,
}

#[derive(Serialize)]
pub struct DataResponsableList {
    pub items: Vec<ResponsablesItem>,
}

#[derive(Serialize)]
pub struct DataViviendaList {
    pub items: Vec<ViviendaItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserItem {
    pub nombre_usuario: String,
    pub nombre_completo: String,
    pub documento_identidad: String,
    pub lugar_expedicion: String,
    pub sexo: String,
    pub etnia: String,
    pub email_personal: String,
    pub email_institucional: String,
    pub telefono_movil: String,
    pub fecha_nacimiento: String,
    pub lugar_nacimiento: String,
    pub nacionalidad: String,
    pub tipo_sangre: String,
    pub eps: String,
    pub situacion_militar: String,
    pub responsables: Vec<ResponsablesItem>,
    pub vivienda: Vec<ViviendaItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserItemUpdate {
    pub nombre_usuario: String,
    pub lugar_expedicion: String,
    pub email_personal: String,
    pub telefono_movil: String,
    pub eps: String,
    pub situacion_militar: String,
    pub vivienda: Vec<ViviendaItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponsablesItem {
    pub responsable_nombre: String,
    pub responsable_tipo_doc: String,
    pub responsable_numero_doc: String,
    pub responsable_telefono: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ViviendaItem {
    pub vivienda_tipo: String,
    pub vivienda_direccion: String,
    pub vivienda_departamento: String,
    pub vivienda_codigo_postal: String,
    pub vivienda_telefono: String,
    pub vivienda_estrato: String,
}

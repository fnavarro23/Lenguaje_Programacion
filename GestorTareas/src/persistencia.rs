use crate::tarea::Tarea;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;
use std::fmt;

#[derive(Debug)]
pub enum ErrorPersistencia {
    Io(std::io::Error),
    Json(serde_json::Error),
}

#[derive(Serialize, Deserialize)]
struct IdData {
    next_id: u32,
}

impl fmt::Display for ErrorPersistencia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorPersistencia::Io(err) => write!(f, "Error de E/S: {}", err),
            ErrorPersistencia::Json(err) => write!(f, "Error de formato JSON: {}", err),
        }
    }
}

impl From<std::io::Error> for ErrorPersistencia {
    fn from(err: std::io::Error) -> ErrorPersistencia {
        ErrorPersistencia::Io(err)
    }
}

impl From<serde_json::Error> for ErrorPersistencia {
    fn from(err: serde_json::Error) -> ErrorPersistencia {
        ErrorPersistencia::Json(err)
    }
}

pub fn cargar_id(nombre_archivo: &str) -> Result<u32, ErrorPersistencia> {
    match fs::read_to_string(nombre_archivo) {
        Ok(contenido) => {
            let data: IdData = serde_json::from_str(&contenido)?;
            Ok(data.next_id)
        }
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(1),
        Err(e) => Err(e.into()),
    }
}

pub fn guardar_id(id: u32, nombre_archivo: &str) -> Result<(), ErrorPersistencia> {
    let data = IdData { next_id: id };
    let json = serde_json::to_string_pretty(&data)?;
    fs::write(nombre_archivo, json)?;
    Ok(())
}

pub fn cargar(nombre_archivo: &str) -> Result<Vec<Tarea>, ErrorPersistencia> {
    match fs::read_to_string(nombre_archivo) {
        Ok(contenido) => {
            if contenido.trim().is_empty() {
                return Ok(vec![]);
            }
            let tareas = serde_json::from_str(&contenido)?;
            Ok(tareas)
        }
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(vec![]),
        Err(e) => Err(e.into()),
    }
}

pub fn guardar(tareas: &[Tarea], nombre_archivo: &str) -> Result<(), ErrorPersistencia> {
    let json = serde_json::to_string_pretty(tareas)?;
    fs::write(nombre_archivo, json)?;
    Ok(())
}
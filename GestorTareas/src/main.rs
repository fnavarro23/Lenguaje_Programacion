mod tarea;
mod gestor;
mod persistencia;
mod comandos;

use clap::{CommandFactory, Parser, Subcommand};
use gestor::Gestor;

const NOMBRE_ARCHIVO: &str = "tareas.json";
const ID_ARCHIVO: &str = "id.json";

#[derive(Parser, Debug)]
#[command(name = "gestor-tareas", version = "1.0", author = "Tu Nombre", about = "gestor")]
struct Cli {
    #[command(subcommand)]
    comando: Comandos,
}

#[derive(Subcommand, Debug)]
enum Comandos {
    Agregar {
        descripcion: String,
    },
    Lista,
    Actualizar {
        id: u32,
        estado: String,
    },
    Borrar {
        id: u32,
    },
    Ayuda,
}

fn main() {
    let cli = Cli::parse();

    let tareas_cargadas = match persistencia::cargar(NOMBRE_ARCHIVO) {
        Ok(tareas) => tareas,
        Err(e) => {
            eprintln!("Error al cargar las tareas desde '{}': {}", NOMBRE_ARCHIVO, e);
            std::process::exit(1);
        }
    };

    let siguiente_id = match persistencia::cargar_id(ID_ARCHIVO) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error al cargar el ID desde '{}': {}", ID_ARCHIVO, e);
            std::process::exit(1);
        }
    };

    let mut gestor = Gestor::new(tareas_cargadas, siguiente_id);

    let mut necesita_guardar = false;

    match cli.comando {
        Comandos::Agregar { descripcion } => {
            comandos::agregar::ejecutar(&mut gestor, descripcion);
            necesita_guardar = true;
        }
        Comandos::Lista => {
            comandos::listar::ejecutar(&gestor);
        }
        Comandos::Actualizar { id, estado } => {
            comandos::actualizar::ejecutar(&mut gestor, id, estado);
            necesita_guardar = true;
        }
        Comandos::Borrar { id } => {
            comandos::borrar::ejecutar(&mut gestor, id);
            necesita_guardar = true;
        }
        Comandos::Ayuda => {
            Cli::command().print_help().unwrap();
        }
    }

    if necesita_guardar {
        if let Err(e) = persistencia::guardar(&gestor.listar(), NOMBRE_ARCHIVO) {
            eprintln!("Error al guardar las tareas en '{}': {}", NOMBRE_ARCHIVO, e);
        }
        if let Err(e) = persistencia::guardar_id(gestor.siguiente_id(), ID_ARCHIVO) {
            eprintln!("Error al guardar el ID en '{}': {}", ID_ARCHIVO, e);
        }
    }
}
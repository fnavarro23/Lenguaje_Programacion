
use crate::gestor::Gestor;

pub fn ejecutar(gestor: &mut Gestor, id: u32) {
    match gestor.borrar(id) {
        Ok(tarea_borrada) => {
            println!("Tarea borrada:");
            println!("   ID: {}", tarea_borrada.id);
            println!("   Descripción: '{}'", tarea_borrada.descripcion);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
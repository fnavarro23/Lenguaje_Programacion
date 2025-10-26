
use crate::gestor::Gestor;

pub fn ejecutar(gestor: &mut Gestor, id: u32, estado_str: String) {
    match estado_str.parse() {
        Ok(nuevo_estado) => {
            match gestor.actualizar(id, nuevo_estado) {
                Ok(tarea_actualizada) => {
                    println!("Tarea actualizada:");
                    println!("   ID: {}", tarea_actualizada.id);
                    println!("   Nuevo Estado: {}", tarea_actualizada.estado);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("   Los estados válidos son pendiente, en proceso, finalizado, cancelado.");
        }
    }
}
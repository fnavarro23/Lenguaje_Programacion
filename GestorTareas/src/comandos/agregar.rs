use crate::gestor::Gestor;

pub fn ejecutar(gestor: &mut Gestor, descripcion: String) {
    match gestor.agregar(descripcion) {
        Ok(tarea_agregada) => {
            println!("Tarea agregada:");
            println!("   ID: {}", tarea_agregada.id);
            println!("   Descripción: {}", tarea_agregada.descripcion);
        }
        Err(e) => eprintln!("{}", e),
    }
}
use crate::gestor::Gestor;

pub fn ejecutar(gestor: &Gestor) {
    let tareas = gestor.listar();
    if tareas.is_empty() {
        println!("No hay tareas en la lista.");
    } else {
        println!("Lista de Tareas:");
        println!("{:-<62}", "");
        println!("{:>3} | {:<25} | {:<12} | {:<19}", "ID", "Descripción", "Estado", "Fecha Creación");
        println!("{:<62}", "");
        for tarea in tareas {
            println!(
                "{:>3} | {:<25} | {:<12} | {}",
                tarea.id,
                tarea.descripcion,
                tarea.estado.to_string(), 
                tarea.fecha_creacion
            );
        }
        println!("{:<62}", "");
    }
}
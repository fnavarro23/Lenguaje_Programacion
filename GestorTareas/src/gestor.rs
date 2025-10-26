use crate::persistencia::{ErrorPersistencia};
use crate::tarea::{Tarea, EstadoTarea};
use chrono::Local;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ErrorGestor {
    TareaNoEncontrada(u32),
    DescripcionVacia,
    ErrorPersistencia(String),
}

impl fmt::Display for ErrorGestor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorGestor::TareaNoEncontrada(id) => write!(f, "No se encontró ninguna tarea con el ID {}.", id),
            ErrorGestor::DescripcionVacia => write!(f, "La descripción de la tarea no puede estar vacía."),
            ErrorGestor::ErrorPersistencia(e) => write!(f, "Error de persistencia: {}", e),
        }
    }
}

impl From<ErrorPersistencia> for ErrorGestor {
    fn from(e: ErrorPersistencia) -> Self {
        ErrorGestor::ErrorPersistencia(e.to_string())
    }
}

pub struct Gestor {
    pub tareas: Vec<Tarea>,
    siguiente_id: u32,
}

impl Gestor {
    pub fn new(tareas: Vec<Tarea>, siguiente_id: u32) -> Self {
        Gestor {
            tareas,
            siguiente_id,
        }
    }

    pub fn agregar(&mut self, descripcion: String) -> Result<&Tarea, ErrorGestor> {
        if descripcion.trim().is_empty() {
            return Err(ErrorGestor::DescripcionVacia);
        }

        let nueva_tarea = Tarea {
            id: self.siguiente_id,
            descripcion,
            estado: EstadoTarea::Pendiente,
            fecha_creacion: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        self.tareas.push(nueva_tarea);
        self.siguiente_id += 1;
        Ok(self.tareas.last().unwrap())
    }

    pub fn siguiente_id(&self) -> u32 {
        self.siguiente_id
    }

    pub fn listar(&self) -> &Vec<Tarea> {
        &self.tareas
    }

    pub fn actualizar(&mut self, id_tarea: u32, nuevo_estado: EstadoTarea) -> Result<&Tarea, ErrorGestor> {
        if let Some(tarea) = self.tareas.iter_mut().find(|t| t.id == id_tarea) {
            tarea.estado = nuevo_estado;
            Ok(tarea)
        } else {
            Err(ErrorGestor::TareaNoEncontrada(id_tarea))
        }
    }

    pub fn borrar(&mut self, id_tarea: u32) -> Result<Tarea, ErrorGestor> {
        if let Some(indice) = self.tareas.iter().position(|t| t.id == id_tarea) {
            Ok(self.tareas.remove(indice))
        } else {
            Err(ErrorGestor::TareaNoEncontrada(id_tarea))
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn test_new_con_tareas_existentes() {
        let tareas_iniciales = vec![
            Tarea { id: 1, descripcion: "Tarea 1".to_string(), estado: EstadoTarea::Pendiente, fecha_creacion: "".to_string() },
            Tarea { id: 3, descripcion: "Tarea 3".to_string(), estado: EstadoTarea::Pendiente, fecha_creacion: "".to_string() },
        ];
        let mut gestor = Gestor::new(tareas_iniciales, 4);
        assert_eq!(gestor.siguiente_id, 4);

        gestor.agregar("Nueva tarea".to_string()).unwrap();
        assert_eq!(gestor.tareas.len(), 3);
        assert_eq!(gestor.tareas.last().unwrap().id, 4);
        assert_eq!(gestor.siguiente_id, 5);
    }

    #[test]
    fn test_agregar_y_proximo_id() {
        let mut gestor = Gestor::new(vec![], 1);
        
        gestor.agregar("Tarea de prueba 1".to_string()).unwrap();
        assert_eq!(gestor.tareas.len(), 1);
        assert_eq!(gestor.tareas[0].id, 1);
        assert_eq!(gestor.siguiente_id, 2);

        gestor.agregar("Tarea de prueba 2".to_string()).unwrap();
        assert_eq!(gestor.tareas.len(), 2);
        assert_eq!(gestor.tareas[1].id, 2);
        assert_eq!(gestor.siguiente_id, 3);
    }

    #[test]
    fn test_actualizar_estado_existente() {
        let mut gestor = Gestor::new(vec![], 1);
        gestor.agregar("Tarea para actualizar".to_string()).unwrap();

        let resultado = gestor.actualizar(1, EstadoTarea::Finalizado);
        assert!(resultado.is_ok());
        let tarea_actualizada = resultado.unwrap();
        assert_eq!(tarea_actualizada.id, 1);
        assert_eq!(tarea_actualizada.estado, EstadoTarea::Finalizado);

        assert_eq!(gestor.tareas[0].estado, EstadoTarea::Finalizado);
    }

    #[test]
    fn test_actualizar_estado_no_existente() {
        let mut gestor = Gestor::new(vec![], 1);
        gestor.agregar("Una tarea".to_string()).unwrap();

        let resultado = gestor.actualizar(99, EstadoTarea::Finalizado);
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), ErrorGestor::TareaNoEncontrada(99));
    }

    #[test]
    fn test_borrar_tarea_existente() {
        let mut gestor = Gestor::new(vec![], 1);
        gestor.agregar("Tarea para borrar".to_string()).unwrap();
        assert_eq!(gestor.tareas.len(), 1);

        let resultado = gestor.borrar(1);
        assert!(resultado.is_ok()); 
        assert_eq!(resultado.unwrap().descripcion, "Tarea para borrar");
        assert_eq!(gestor.tareas.len(), 0);
    }
    
    #[test]
    fn test_borrar_tarea_no_existente() {
        let mut gestor = Gestor::new(vec![], 1);
        gestor.agregar("Tarea única".to_string()).unwrap();
        
        let resultado = gestor.borrar(99); 
        assert!(resultado.is_err()); 
        assert_eq!(resultado.unwrap_err(), ErrorGestor::TareaNoEncontrada(99));
        assert_eq!(gestor.tareas.len(), 1); 
    }

    #[test]
    fn test_agregar_tarea_descripcion_vacia() {
        let mut gestor = Gestor::new(vec![], 1);
        let resultado = gestor.agregar("".to_string());
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), ErrorGestor::DescripcionVacia);
        assert_eq!(gestor.tareas.len(), 0);
    }

    #[test]
    fn test_agregar_tarea_descripcion_con_espacios() {
        let mut gestor = Gestor::new(vec![], 1);
        let resultado = gestor.agregar("   ".to_string());
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), ErrorGestor::DescripcionVacia);
        assert_eq!(gestor.tareas.len(), 0);
    }

    #[test]
    fn test_ids_no_se_reutilizan_al_borrar() {
        let mut gestor = Gestor::new(vec![], 1);
        
        gestor.agregar("Tarea 1".to_string()).unwrap(); 
        gestor.agregar("Tarea 2".to_string()).unwrap(); 
        
        assert_eq!(gestor.siguiente_id, 3);
        
        gestor.borrar(1).unwrap();
        assert_eq!(gestor.siguiente_id, 3);

        let nueva_tarea = gestor.agregar("Tarea 3".to_string()).unwrap(); 
        assert_eq!(nueva_tarea.id, 3, "El ID de la nueva tarea debería ser 3, no uno reutilizado.");

        assert_eq!(gestor.siguiente_id, 4);
    }
}
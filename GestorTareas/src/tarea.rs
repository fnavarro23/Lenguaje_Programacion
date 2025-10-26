use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EstadoTarea {
    Pendiente,
    EnProceso,
    Finalizado,
    Cancelado,
}

impl FromStr for EstadoTarea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pendiente" => Ok(EstadoTarea::Pendiente),
            "enproceso" => Ok(EstadoTarea::EnProceso),
            "finalizado" => Ok(EstadoTarea::Finalizado),
            "cancelado" => Ok(EstadoTarea::Cancelado),
            _ => Err(format!("'{}' no es un estado válido.", s)),
        }
    }
}

impl Display for EstadoTarea {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            EstadoTarea::Pendiente => write!(f, "Pendiente"),
            EstadoTarea::EnProceso => write!(f, "En Proceso"),
            EstadoTarea::Finalizado => write!(f, "Finalizado"),
            EstadoTarea::Cancelado => write!(f, "Cancelado"),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tarea {
    pub id: u32,
    pub descripcion: String,
    pub estado: EstadoTarea,
    pub fecha_creacion: String,
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn test_estado_from_str_ok() {
        assert_eq!("pendiente".parse::<EstadoTarea>().unwrap(), EstadoTarea::Pendiente);
        assert_eq!("enproceso".parse::<EstadoTarea>().unwrap(), EstadoTarea::EnProceso);
        assert_eq!("finalizado".parse::<EstadoTarea>().unwrap(), EstadoTarea::Finalizado);
        assert_eq!("cancelado".parse::<EstadoTarea>().unwrap(), EstadoTarea::Cancelado);
        assert_eq!("Pendiente".parse::<EstadoTarea>().unwrap(), EstadoTarea::Pendiente);
    }

    #[test]
    fn test_estado_from_str_err() {
        let resultado = "invalido".parse::<EstadoTarea>();
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), "'invalido' no es un estado válido.");
    }

    #[test]
    fn test_estado_display() {
        assert_eq!(format!("{}", EstadoTarea::Pendiente), "Pendiente");
        assert_eq!(format!("{}", EstadoTarea::EnProceso), "En Proceso");
        assert_eq!(format!("{}", EstadoTarea::Finalizado), "Finalizado");
        assert_eq!(format!("{}", EstadoTarea::Cancelado), "Cancelado");
    }

    #[test]
    fn test_tarea_clone_y_partial_eq_estado() {
        let estado1 = EstadoTarea::EnProceso;
        let estado2 = estado1.clone();
        assert_eq!(estado1, estado2);
        assert_ne!(estado1, EstadoTarea::Pendiente);
    }
}
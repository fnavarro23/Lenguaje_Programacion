package tarea

import "time"

type Tarea struct {
	ID            int       `json:"id"`
	Descripcion   string    `json:"descripcion"`
	Estado        string    `json:"estado"`
	FechaCreacion time.Time `json:"fecha_creacion"`
}

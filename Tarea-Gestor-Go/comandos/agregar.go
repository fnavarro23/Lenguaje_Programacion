package comandos

import (
	"fmt"
	"gestor_tareas/manejador"
)

func AgregarTarea(man *manejador.ManejadorTareas, descripcion string) {
	if descripcion == "" {
		fmt.Println("Error: La descripción de la tarea no puede estar vacía.")
		return
	}
	man.Agregar(descripcion)
	if err := man.Guardar(); err != nil {
		fmt.Printf("Error al guardar la tarea: %v\n", err)
		return
	}
	fmt.Printf(" Tarea agregada: \"%s\"\n", descripcion)
}

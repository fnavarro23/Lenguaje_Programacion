package comandos

import (
	"fmt"
	"gestor_tareas/manejador"
	"strconv"
)

func BorrarTarea(man *manejador.ManejadorTareas, idStr string) {
	if idStr == "" {
		fmt.Println("Error: Debes proporcionar el ID de la tarea a borrar.")
		return
	}
	id, err := strconv.Atoi(idStr)
	if err != nil {
		fmt.Println("Error: El ID proporcionado no es un número válido.")
		return
	}

	if err := man.Borrar(id); err != nil {
		fmt.Println(err) 
		return
	}

	if err := man.Guardar(); err != nil {
		fmt.Printf("Error al guardar los cambios: %v\n", err)
		return
	}

	fmt.Printf(" Tarea con ID %d borrada.\n", id)
}

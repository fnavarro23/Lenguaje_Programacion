package comandos

import (
	"fmt"
	"gestor_tareas/manejador"
	"strconv"
)

func CambiarEstadoTarea(man *manejador.ManejadorTareas, idStr string, nuevoEstado string) {
	if idStr == "" || nuevoEstado == "" {
		fmt.Println("Error: Debes proporcionar el ID y el nuevo estado.")
		fmt.Println("Uso: go run main.go estado <ID> <pendiente|en proceso|finalizada|cancelada>")
		return
	}

	id, err := strconv.Atoi(idStr)
	if err != nil {
		fmt.Println("Error: El ID proporcionado no es un número válido.")
		return
	}

	if err := man.CambiarEstado(id, nuevoEstado); err != nil {
		fmt.Println(err)
		return
	}

	if err := man.Guardar(); err != nil {
		fmt.Printf("Error al guardar los cambios: %v\n", err)
		return
	}

	fmt.Printf("Estado de la tarea con ID %d cambiado a '%s'.\n", id, nuevoEstado)
}

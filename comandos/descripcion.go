package comandos

import (
	"fmt"
	"gestor_tareas/manejador"
	"strconv"
	"strings"
)

func ActualizarDescripcionTarea(man *manejador.ManejadorTareas, idStr string, args []string) {
	if idStr == "" || len(args) == 0 {
		fmt.Println("Error: Debes proporcionar el ID y la nueva descripción.")
		fmt.Println("Uso: go run main.go descripcion <ID> <nueva descripción de la tarea>")
		return
	}

	id, err := strconv.Atoi(idStr)
	if err != nil {
		fmt.Println("Error: El ID proporcionado no es un número válido.")
		return
	}

	nuevaDescripcion := strings.Join(args, " ")

	if err := man.ActualizarDescripcion(id, nuevaDescripcion); err != nil {
		fmt.Println(err)
		return
	}

	if err := man.Guardar(); err != nil {
		fmt.Printf("Error al guardar los cambios: %v\n", err)
		return
	}

	fmt.Printf("Descripción de la tarea con ID %d actualizada.\n", id)
}

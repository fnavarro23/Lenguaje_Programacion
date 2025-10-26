package main

import (
	"fmt"
	"os"
	"strings"

	"gestor_tareas/comandos"
	"gestor_tareas/manejador"
)

const nombreArchivo = "tareas.json"

func mostrarAyuda() {
	fmt.Println("Gestor de Tareas")
	fmt.Println("go run main.go [comando] [argumentos]")
	fmt.Println("\nComandos:")
	fmt.Println("  ayuda                        - Muestra este menú de ayuda.")
	fmt.Println("  agregar (descripcion)        - Agrega una nueva tarea.")
	fmt.Println("  lista                        - Muestra todas las tareas.")
	fmt.Println("  descripcion (ID)  (texto)    - Actualiza la descripción de una tarea.")
	fmt.Println("  estado (ID)  (nuevo_estado)  - Cambia el estado de una tarea.")
	fmt.Println("                                (Estados: pendiente, en proceso, finalizada, cancelada)")
	fmt.Println("  borrar (ID)                  - Elimina una tarea por su ID.")
}

func main() {
	man, err := manejador.NuevoManejador(nombreArchivo)
	if err != nil {
		fmt.Printf("Error fatal al inicializar: %v\n", err)
		os.Exit(1)
	}

	if len(os.Args) < 2 {
		mostrarAyuda()
		os.Exit(0)
	}

	comando := os.Args[1]
	argumentos := os.Args[2:]

	switch comando {
	case "ayuda":
		mostrarAyuda()

	case "agregar":
		if len(argumentos) == 0 {
			fmt.Println("Error: Se requiere la descripción de la tarea.")
			mostrarAyuda()
			os.Exit(1)
		}
		descripcion := strings.Join(argumentos, " ")
		comandos.AgregarTarea(man, descripcion)

	case "lista":
		comandos.ListarTareas(man)

	case "descripcion":
		if len(argumentos) < 2 {
			fmt.Println("Error: Se requiere el ID de la tarea y la nueva descripción.")
			mostrarAyuda()
			os.Exit(1)
		}
		idStr := argumentos[0]
		nuevaDescripcionArgs := argumentos[1:]
		comandos.ActualizarDescripcionTarea(man, idStr, nuevaDescripcionArgs)

	case "borrar":
		if len(argumentos) != 1 {
			fmt.Println("Error: Se requiere el ID de la tarea a borrar.")
			mostrarAyuda()
			os.Exit(1)
		}
		comandos.BorrarTarea(man, argumentos[0])

	case "estado":
		if len(argumentos) < 2 {
			fmt.Println("Error: Se requiere el ID y el nuevo estado.")
			mostrarAyuda()
			os.Exit(1)
		}
		idStr := argumentos[0]
		nuevoEstado := strings.Join(argumentos[1:], " ")
		comandos.CambiarEstadoTarea(man, idStr, nuevoEstado)

	default:
		fmt.Printf("Error: Comando '%s' no reconocido.\n", comando)
		mostrarAyuda()
	}
}

package comandos

import (
	"fmt"
	"gestor_tareas/manejador"
	"os"
	"text/tabwriter"
)

func ListarTareas(man *manejador.ManejadorTareas) {
	if len(man.Tareas) == 0 {
		fmt.Println("No hay tareas para mostrar")
		return
	}

	fmt.Println("----- Tareas -----")
	w := tabwriter.NewWriter(os.Stdout, 0, 0, 3, ' ', tabwriter.TabIndent)
	fmt.Fprintln(w, "ID\tDescripcion\tEstado\tFecha de Creacion")
	fmt.Fprintln(w, "--\t-----------\t------\t-----------------")

	for _, t := range man.Tareas {
		fmt.Fprintf(w, "%d\t%s\t%s\t%s\n", t.ID, t.Descripcion, t.Estado, t.FechaCreacion.Format("2006-01-02 15:04"))
	}

	w.Flush()
	fmt.Println("-----------------------")
}

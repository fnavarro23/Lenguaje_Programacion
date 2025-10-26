package manejador

import (
	"os"
	"testing"
)

func setup(t *testing.T) (man *ManejadorTareas, cleanup func()) {
	archivoPrueba := "tareas_test.json"
	man, err := NuevoManejador(archivoPrueba)
	if err != nil {
		t.Fatalf("No se pudo crear el manejador de prueba: %v", err)
	}

	cleanup = func() {
		os.Remove(archivoPrueba)
	}

	return man, cleanup
}

func TestAgregarTarea(t *testing.T) {
	man, cleanup := setup(t)
	defer cleanup()

	descripcion := "Hacer la compra"
	man.Agregar(descripcion)

	if len(man.Tareas) != 1 {
		t.Errorf("Se esperaba 1 tarea, pero se encontraron %d", len(man.Tareas))
	}
	if man.Tareas[0].Descripcion != descripcion {
		t.Errorf("Descripción incorrecta. Se esperaba '%s', se obtuvo '%s'", descripcion, man.Tareas[0].Descripcion)
	}
	if man.Tareas[0].Estado != "Pendiente" {
		t.Errorf("Estado incorrecto. Se esperaba 'Pendiente', se obtuvo '%s'", man.Tareas[0].Estado)
	}
}

func TestBorrarTarea(t *testing.T) {
	man, cleanup := setup(t)
	defer cleanup()

	man.Agregar("Tarea para borrar")
	id := man.Tareas[0].ID

	if err := man.Borrar(id); err != nil {
		t.Fatalf("Error al borrar la tarea: %v", err)
	}
	if len(man.Tareas) != 0 {
		t.Errorf("La tarea no se borró. Quedan %d tareas", len(man.Tareas))
	}

	if err := man.Borrar(999); err == nil {
		t.Error("Se esperaba un error")
	}
}

func TestCambiarEstado(t *testing.T) {
	man, cleanup := setup(t)
	defer cleanup()

	man.Agregar("Cambiar mi estado")
	id := man.Tareas[0].ID

	if err := man.CambiarEstado(id, "finalizada"); err != nil {
		t.Fatalf("Error al cambiar el estado de la tarea: %v", err)
	}
	if man.Tareas[0].Estado != "Finalizada" {
		t.Errorf("Estado no se cambió. Se esperaba 'Finalizada', se obtuvo '%s'", man.Tareas[0].Estado)
	}

	if err := man.CambiarEstado(id, "perdida"); err == nil {
		t.Errorf("Se esperaba un error")
	}
}

func TestGuardarYCargar(t *testing.T) {
	man, cleanup := setup(t)
	defer cleanup()

	man.Agregar("Tarea 1")
	man.Agregar("Tarea 2")

	if err := man.Guardar(); err != nil {
		t.Fatalf("Error al guardar las tareas: %v", err)
	}

	man2, err := NuevoManejador(man.Archivo)
	if err != nil {
		t.Fatalf("Error al crear el segundo manejador para cargar: %v", err)
	}

	if len(man2.Tareas) != 2 {
		t.Fatalf("No se cargaron las tareas correctamente. Se esperaban 2, se obtuvieron %d", len(man2.Tareas))
	}
	if man2.Tareas[1].Descripcion != "Tarea 2" {
		t.Errorf("La descripción de la tarea cargada no coincide")
	}
}
func TestActualizarDescripcion(t *testing.T) {
	man, cleanup := setup(t)
	defer cleanup()

	man.Agregar("Descripción original")
	id := man.Tareas[0].ID
	nuevaDesc := "Esta es la descripción actualizada"

	if err := man.ActualizarDescripcion(id, nuevaDesc); err != nil {
		t.Fatalf("Error al actualizar la descripción: %v", err)
	}

	if man.Tareas[0].Descripcion != nuevaDesc {
		t.Errorf("La descripción no se actualizó. Se esperaba '%s', se obtuvo '%s'", nuevaDesc, man.Tareas[0].Descripcion)
	}

	if err := man.ActualizarDescripcion(999, "test"); err == nil {
		t.Error("Se esperaba un error al actualizar una tarea inexistente, pero no se obtuvo ninguno")
	}
}

package manejador

import (
	"encoding/json"
	"errors"
	"fmt"
	"gestor_tareas/tarea"
	"os"
	"strings"
	"time"
)

type ManejadorTareas struct {
	Tareas  []tarea.Tarea
	Archivo string
}

func NuevoManejador(archivo string) (*ManejadorTareas, error) {
	manejador := &ManejadorTareas{
		Archivo: archivo,
	}

	err := manejador.Cargar()
	if os.IsNotExist(err) {
		return manejador, nil
	}
	return manejador, err
}

func (m *ManejadorTareas) Cargar() error {
	datos, err := os.ReadFile(m.Archivo)
	if err != nil {
		return err
	}
	if len(datos) == 0 {
		return nil
	}
	return json.Unmarshal(datos, &m.Tareas)
}

func (m *ManejadorTareas) Guardar() error {
	datos, err := json.MarshalIndent(m.Tareas, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(m.Archivo, datos, 0644)
}

func (m *ManejadorTareas) encontrarSiguienteID() int {
	if len(m.Tareas) == 0 {
		return 1
	}
	maxID := 0
	for _, t := range m.Tareas {
		if t.ID > maxID {
			maxID = t.ID
		}
	}
	return maxID + 1
}

func (m *ManejadorTareas) Agregar(descripcion string) {
	nuevaTarea := tarea.Tarea{
		ID:            m.encontrarSiguienteID(),
		Descripcion:   descripcion,
		Estado:        "Pendiente",
		FechaCreacion: time.Now(),
	}
	m.Tareas = append(m.Tareas, nuevaTarea)
}

func (m *ManejadorTareas) Borrar(id int) error {
	indiceAEliminar := -1
	for i, t := range m.Tareas {
		if t.ID == id {
			indiceAEliminar = i
			break
		}
	}

	if indiceAEliminar == -1 {
		return errors.New("error: Tarea con el ID especificado no encontrada")
	}

	m.Tareas = append(m.Tareas[:indiceAEliminar], m.Tareas[indiceAEliminar+1:]...)
	return nil
}

func (m *ManejadorTareas) CambiarEstado(id int, nuevoEstado string) error {
	estadoNormalizado := strings.ToLower(nuevoEstado)
	estadosValidos := map[string]string{
		"pendiente":  "Pendiente",
		"en proceso": "En Proceso",
		"finalizada": "Finalizada",
		"cancelada":  "Cancelada",
	}

	estadoCapitalizado, ok := estadosValidos[estadoNormalizado]
	if !ok {
		return fmt.Errorf("error: estado '%s' no es válido. Usa: pendiente, en proceso, finalizada, cancelada", nuevoEstado)
	}

	for i := range m.Tareas {
		if m.Tareas[i].ID == id {
			m.Tareas[i].Estado = estadoCapitalizado
			return nil
		}
	}

	return errors.New("error: Tarea con el ID especificado no encontrada")

}

func (m *ManejadorTareas) ActualizarDescripcion(id int, nuevaDescripcion string) error {
	if strings.TrimSpace(nuevaDescripcion) == "" {
		return errors.New("error: la descripción no puede estar vacía")
	}

	for i := range m.Tareas {
		if m.Tareas[i].ID == id {
			m.Tareas[i].Descripcion = nuevaDescripcion
			return nil
		}
	}

	return errors.New("error: Tarea con el ID especificado no encontrada")
}

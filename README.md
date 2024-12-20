# To-Do List en Rust

Este proyecto crea una aplicación de consola básica para gestionar una lista de tareas (To-Do List). Los usuarios pueden agregar, eliminar y ver tareas a través de un menú interactivo.

## Índice
1. [Descripción General](#descripcion-general)
2. [Dependencias](#dependencias)
3. [Estructura del Código](#estructura-del-codigo)
4. [Explicación Detallada del Código](#explicacion-detallada-del-codigo)
    - [Función main](#funcion-main)
    - [Función add_task](#funcion-add_task)
    - [Función erase_task](#funcion-erase_task)

## 1. Descripción General <a name="descripcion-general"></a>
Este es un proyecto simple de Rust que utiliza vectores (`Vec<String>`) para almacenar una lista de tareas. Los usuarios pueden:
- Agregar tareas.
- Eliminar tareas.
- Ver las tareas actuales.

La interacción con el usuario se realiza mediante la entrada de texto en la consola, y el flujo se repite hasta que el usuario decide salir de la aplicación.

## 2. Dependencias <a name="dependencias"></a>
Este proyecto no tiene dependencias externas. Utiliza la biblioteca estándar de Rust (`std::io`) para manejar la entrada y salida de texto.

## 3. Estructura del Código <a name="estructura-del-codigo"></a>
### Funciones Principales <a name="funciones"></a>
- `main`: Controla el flujo principal de la aplicación.
- `add_task`: Permite agregar una nueva tarea a la lista.
- `erase_task`: Permite eliminar una tarea seleccionada por el usuario.

## 4. Explicación Detallada del Código <a name="explicacion-detallada-del-codigo"></a>

### Función main <a name="funcion-main"></a>
La función `main` es el punto de entrada de la aplicación. Controla el ciclo principal del programa, donde el usuario puede seleccionar una acción del menú (agregar, eliminar o salir).

```rust
use std::io;

fn main() {
    let mut todo_list: Vec<String> = Vec::new();  

    loop {
        
        println!("\nTo-Do List:");
        println!("1. Add a task");
        println!("2. Remove a task");
        println!("3. CLOSE");

        
        for (i, task) in todo_list.iter().enumerate() {
            println!("{}. {}", i + 1, task);
        }

       
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        
        match choice.trim() {
            "1" => add_task(&mut todo_list),
            "2" => erase_task(&mut todo_list),
            "3" => {
                println!("Closing the To-Do List.");
                break;
            },
            _ => println!("Invalid option, please try again."),
        }
    }
}
```
## Explicación:

- **todo_list**: Creamos un vector vacío para almacenar nuestras tareas. Cada tarea es una cadena de texto (`String`).
- **Menú Interactivo**: En cada iteración del ciclo `loop`, se muestran tres opciones para el usuario: agregar una tarea, eliminar una tarea, o cerrar la aplicación.
- **Ciclo infinito**: El ciclo `loop` se ejecuta hasta que el usuario elija la opción "3" para cerrar la aplicación.
- **Entrada del Usuario**: Usamos `io::stdin().read_line()` para leer lo que el usuario ingresa. El resultado se procesa con `match` para determinar qué acción tomar.

### Función add_task <a name="funcion-add_task"></a>
La función `add_task` permite al usuario agregar una nueva tarea a la lista.

```rust
fn add_task(todo_list: &mut Vec<String>) {
    println!("Enter the name of the task:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read line");

    todo_list.push(task.trim().to_string());  
    println!("Task added!");
}
```
## Explicación:

- **Entrada de tarea**: Pedimos al usuario que ingrese el nombre de una nueva tarea.
- **Agregar tarea a la lista**: La tarea se agrega al vector `todo_list` utilizando el método `push()`.
- **Confirmación**: Se imprime un mensaje de confirmación.

### Función erase_task <a name="funcion-erase_task"></a>
La función `erase_task` permite al usuario eliminar una tarea especificando su número en la lista.

```rust
fn erase_task(todo_list: &mut Vec<String>) {
    if todo_list.is_empty() {
        println!("No tasks to erase.");
        return;
    }

    println!("Enter the number of the task to erase:");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    match index.trim().parse::<usize>().ok().filter(|&i| i > 0 && i <= todo_list.len()) {
        Some(i) => {
            todo_list.remove(i - 1);  
            println!("Task erased!");
        }
        None => println!("Invalid task number."),
    }
}
```
## Explicación:

- **Verificación de tareas**: Antes de pedir el número de tarea, verificamos si la lista está vacía.
- **Entrada del número de tarea**: Solicitamos al usuario que ingrese el número de la tarea a eliminar.
- **Conversión a número (parse)**: Convertimos el input del usuario a un número (`usize`) utilizando `parse::<usize>()`.
- **Filtro de índices válidos (filter)**: Verificamos si el número ingresado es válido.
- **match para validación**:
  - Si el número es válido (`Some(i)`), eliminamos la tarea correspondiente del vector.
  - Si el número no es válido (`None`), mostramos un mensaje de error.

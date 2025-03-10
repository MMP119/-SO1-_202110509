# [SO1]_202110509
 
# Manual de Uso e Instalación

Este documento describe de forma breve el modo de uso, instalación y ejemplos básicos, compuesto por tres módulos principales:

- **Módulo Kernel (C):** Se encarga de recolectar métricas del sistema (uso de CPU, memoria, I/O y disco) y gestionar la información de contenedores.  
  Ver: [`Proyecto1/kernel/sysinfo.c`](Proyecto1/kernel/sysinfo.c)

- **Servicio de Gestión de Contenedores (Rust):** Proporciona funciones para la creación y eliminación de contenedores, la gestión de logs y el envío de métricas a un API.  
  Ver: [`Proyecto1/rust/src/main.rs`](Proyecto1/rust/src/main.rs)

- **Scripts Bash para Creación de Contenedores de Estrés:** Script encargados de crear los contenedores que realizan una prueba de estrés.
  Ver: [`Proyecto1/contenedores/create.sh`](Proyecto1/contenedores/create.sh)

---

## 1. Instalación

### Requisitos

- **Kernel Module (C):**  
  - Entorno de compilación para C (gcc, make).
  - Permisos de administrador (para compilar e insertar módulos en el kernel).

- **Proyecto Rust:**  
  - [Rust](https://www.rust-lang.org/tools/install) (cargo y rustc instalados).

### Pasos de Instalación

#### Módulo Kernel (C)

1. Navega al directorio del kernel:
    ```sh
    cd Proyecto1/kernel
    ```
2. Compila el módulo usando `make`:
    ```sh
    make
    ```
   Esto generará el módulo correspondiente que debe cargarse en el kernel.

#### Proyecto Rust

1. Navega al directorio del proyecto Rust:
    ```sh
    cd Proyecto1/rust
    ```
2. Instala las dependencias y compila el proyecto:
    ```sh
    cargo build --release
    ```
3. (Opcional) Ejecuta el servicio:
    ```sh
    cargo run --release
    ```

### Encender entorno grafana: 
1. Ingresa en la terminal:
```bash
sudo /bin/systemctl start grafana-server
```
---

## 2. Iniciar el Proyecto

Para iniciar el proyecto de forma rápida, se proporciona el script `start_project.sh`. Este script se encarga de configurar, compilar y ejecutar los diferentes módulos del proyecto. Sigue los siguientes pasos:

1. Asegúrate de que el archivo `start_project.sh` tenga permisos de ejecución. Si no los tiene, otórgale permisos:
    ```bash
    chmod +x start_project.sh
    ```

2. Ejecuta el script desde la raíz del proyecto:
    ```bash
    ./start_project.sh
    ```

El script realizará las siguientes acciones:
- Ejecutará el script para crear contendores, creacá 10 contenedores aleatorios cada 10 segundos durante un minuto.
- Compilará y ejecutará el Servicio de Gestión de Contenedores (Rust).


Con esto, el entorno se inicializará y podrás comenzar a interactuar con el proyecto. Si encuentras algún inconveniente, revisa los logs generados en la terminal para una mayor resolución de problemas.

---

## 3. Uso y Ejemplos

### Uso del Módulo Kernel

El módulo de kernel se encarga de leer información de cgroups para determinar el uso de CPU, memoria, I/O y disco de los contenedores. Por ejemplo:

- **Cálculo del Uso de CPU:**  
  Se lee dos veces el valor `cpustat` para calcular la diferencia y obtener el porcentaje de uso.  
  Ver el cálculo en: sysinfo.c: líneas 50-88

- **Gestión de Contenedores:**  
  Se recorre la lista de procesos para identificar contenedores y se obtienen métricas como memoria y CPU.  
  Revisar la función en: sysinfo.c: líneas 324-369

### Uso del Servicio Rust

El servicio Rust ofrece funcionalidades para la gestión de contenedores y logs. Algunas funciones destacadas son:

- **Creación del Contenedor de Logs:**  
  La función [docker::crear_contenedor_logs](http://_vscodecontentref_/0) es llamada al inicio para generar el contenedor encargado de los logs.

- **Manejo de Contenedores:**  
  Durante la ejecución se evalúa el estado de los contenedores y, en función de las métricas recolectadas, se eliminan aquellos marcados.  
  Ver ejemplo en: main.rs: línea 95

- **Envío de Logs a la API:**  
  Se muestra un ejemplo en la función [enviar_logs_api](http://_vscodecontentref_/1) para enviar los logs finales al servicio, con la siguiente llamada:
  ```rust
  if let Err(e) = enviar_logs_api(&registro_final) {
      eprintln!("Error al enviar logs finales: {}", e);
  }
  ```
  (Más detalles en main.rs: línea 70)


## 4. Ejemplos Prácticos

### Ejemplo 1: Compilar y Ejecutar el Servicio Rust

1. Abre una terminal en el directorio rust y compila el proyecto:
```bash
cargo build
```

2. Ejecuta el binario generado:
```bash
cargo run
```

3. Se iniciará el servicio, mostrando mensajes como:
    - "🚀 Iniciando servicio de gestión de contenedores..."
    - "✅ Uso de CPU: <porcentaje>%"
    - "✅ Memoria Total: <valor>"


### Ejemplo 2: Compilar el Módulo Kernel

1. Abre una terminal en el directorio kernel y compila
```bash
make
```

2. Inserta el módulo en el kernel (ejemplo, usando insmod con permisos de administrador):
```bash
sudo insmod sysinfo.ko
```

3. Consulta la salida del módulo:
```bash
cat /proc/sysinfo_202110509
```

Verás algo como:
![SalidaModulo](/imgs/SalidaModulo.png)


### Ejemplo 3: Imágenes en Grafana

1. Dirígete a Granafa ingresando la siguiente url en tu navegador:
```bash
http://http://localhost:3000
```

2. Ve a la pestaña de dashboard y podrás visualiazar el llamado "todos", donde podrás visualizar todas las gráficas correspondientes a los resultados obtenidos.
![grafana](/imgs/grafana.png)


## 5. Logs_data

Esta sección se enfoca en la recolección y gestión de los datos de logs generados por el servicio de gestión de contenedores.

### ¿Qué es logs_data?

- **logs_data** es una estructura que almacena la información de eventos y errores durante la ejecución del servicio.
- Contiene detalles relevantes como identificadores de contenedor, marcas de tiempo, tipo de evento, métricas (uso de CPU, memoria, etc.) y posibles mensajes de error.

### Estructura de logs_data

Un ejemplo de la estructura de logs_data en formato JSON es el siguiente:

```json
[
    {
        "timestamp": "2025-03-10 00:42:45",
        "memory_total": "15236 MB",
        "memory_free": "2440 MB",
        "memory_used": "12796 MB",
        "cpu_usage": "30%",
        "cpu": [
            {
                "id": "220406a5bc0f",
                "fecha_creacion": "2025-03-10 00:41:14",
                "fecha_eliminacion": "2025-03-10 00:41:17",
                "metric": "21%"
            },
            ...
    }
]
```


## 6. Explicación de Partes Importantes del Código en main.rs

El archivo **main.rs** es el punto de entrada del Servicio de Gestión de Contenedores en Rust. A continuación, se explican algunas secciones clave:

### 1. Función `main`

La función `main` es el punto de inicio del programa. Aquí se inicializan las variables, se configuran los parámetros del servicio y se llaman a las funciones principales que gestionan los contenedores y logs.

```rust
fn main() {
    // Inicialización del servicio
    println!("🚀 Iniciando servicio de gestión de contenedores...");
    
    // Llamada a la función que gestiona los contenedores
    gestionar_contenedores();

    // Recopilación y envío de logs finales
    let registro_final = obtener_registro_logs();
    if let Err(e) = enviar_logs_api(&registro_final) {
        eprintln!("Error al enviar logs finales: {}", e);
    }
}
```

- Se muestra un mensaje de inicio para confirmar la ejecución.
- Se llama a la función gestionar_contenedores() para manejar el ciclo de vida de los contenedores.
- Se obtiene la información final de logs mediante obtener_registro_logs() y se procede a enviarlo a un API mediante enviar_logs_api().

### 2. Función gestionar_contenedores
Esta función se encarga de evaluar el estado de los contenedores y tomar acciones (como eliminar los marcados) según las métricas recolectadas. Esto permite mantener un entorno limpio y controlado durante la ejecución del sistema.

```rust
fn gestionar_contenedores() {
    // Ejemplo de evaluación de métricas y decisión de eliminación de contenedores
    for contenedor in obtener_lista_contenedores() {
        if contenedor.marcado_para_eliminar() {
            eliminar_contenedor(contenedor);
        }
    }
}
```

- Se recorre una lista de contenedores.
- Se evalúa un criterio (por ejemplo, si está marcado para eliminarse) y se invoca la función eliminar_contenedor() según corresponda.

### 3. Función enviar_logs_api
Esta función es responsable de enviar la estructura logs_data al servicio remoto. Se utiliza una estructura de control if let para capturar y manejar posibles errores que ocurran durante el envío.

```rust
fn enviar_logs_api(logs: &LogsData) -> Result<(), Error> {
    // Código que interactúa con la API para enviar los logs
    // ...
    Ok(())
}
```
- Se recibe una referencia a la estructura LogsData.
- Se intenta enviar la información y, en caso de error, se retorna un Result con el error detectado.
- En la función main, al fallar este envío se imprime un mensaje de error detallando el problema.


## 7. Anexo Gráficas:
1. Grafica de estádisticas de la computadora host:<br>
![pc](/imgs/pc.png)

2. Grafica de estádisticas de CPU de los contenedores:<br>
![grafana](/imgs/grafana1.png)

3. Grafica de estádisticas de RAM de los contenedores:<br>
![grafana](/imgs/grafana2.png)

4. Grafica de estádisticas de I/O de los contenedores:<br>
![grafana](/imgs/grafana3.png)

5. Grafica de estádisticas de DISCO de los contenedores:<br>
![grafana](/imgs/grafana4.png)


## 8. Conclusión y Próximos Pasos

Este proyecto integra de manera efectiva diversas tecnologías para el monitoreo y gestión de contenedores, combinando un módulo Kernel en C y un Servicio en Rust, con el apoyo de gráficos en Grafana para la exhibición de métricas en tiempo real.

Se recomienda:
- Revisar detenidamente los comentarios y la documentación interna para comprender a fondo cada módulo.
- Ejecutar los ejemplos prácticos y el script `start_project.sh` para familiarizarse con el entorno.
- Monitorear los logs generados y utilizar la sección de **logs_data** para diagnosticar posibles incidencias.

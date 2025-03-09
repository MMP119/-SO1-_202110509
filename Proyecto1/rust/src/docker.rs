use std::process::Command;
use std::collections::HashMap;

/// Crea el contenedor que administrará los logs.
/// Ejecuta el comando `docker run` y retorna el ID del contenedor si se creó correctamente.
pub fn crear_contenedor_logs() -> Option<String> {
    let output = Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--name")
        .arg("logs_manager")
        .arg("-p")
        .arg("8000:8000")
        .arg("logs_container") // Se asume que esta imagen ya existe.
        .output()
        .expect("Error al crear el contenedor de logs");

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        eprintln!("❌ Error al crear el contenedor de logs");
        None
    } else {
        println!("📂 Contenedor de logs creado: {}", stdout);
        Some(stdout)
    }
}

/// Obtiene una lista de contenedores activos a través de `docker ps`.
/// Retorna un HashMap en el que la clave es el ID del contenedor y el valor es una tupla con el nombre y el comando.
pub fn obtener_contenedores_docker() -> HashMap<String, (String, String)> {
    let output = Command::new("docker")
        .arg("ps")
        .arg("--format")
        .arg("{{.ID}} {{.Names}} {{.Command}}")
        .output()
        .expect("Error al ejecutar docker ps");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut contenedores: HashMap<String, (String, String)> = HashMap::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let id = parts[0].to_string();
            let nombre = parts[1].to_string();
            let comando = parts[2..].join(" ");
            contenedores.insert(id, (nombre, comando));
        }
    }
    contenedores
}

/// Elimina los contenedores que se le pasan en el vector.
/// Para cada contenedor, ejecuta `docker rm -f` y muestra el resultado de la operación.
// pub fn eliminar_contenedores(contenedores: Vec<String>) {
//     for contenedor_id in contenedores {
//         let output = Command::new("docker")
//             .arg("rm")
//             .arg("-f")
//             .arg(&contenedor_id)
//             .output()
//             .expect("Error al eliminar el contenedor");

//         println!(
//             "🗑 Eliminando contenedor {}: {}",
//             contenedor_id,
//             String::from_utf8_lossy(&output.stdout)
//         );
//     }
// }


/// Elimina un contenedor dado su ID.
pub fn eliminar_contenedor(id: String) {
    let output = Command::new("docker")
        .arg("rm")
        .arg("-f")
        .arg(&id)
        .output()
        .expect("Error al eliminar el contenedor");
    println!(
        "🗑 Eliminando contenedor {}: {}",
        id,
        String::from_utf8_lossy(&output.stdout)
    );
}
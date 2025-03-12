use std::process::Command;
use std::collections::HashMap;

//crea el contenedor que administrará los logs usando docker-compose
pub fn crear_contenedor_logs() -> Option<String> {

    let dir = std::env::current_dir().expect("Error al obtener el directorio actual");
    let dir = dir.join("src");
    let dir = dir.to_str().expect("Error al convertir el path a string");
    println!("{}", dir);

    // Levanta el servicio "logs_manager" en modo detached
    let output = Command::new("docker-compose")
        .current_dir(dir)
        .arg("up")
        .arg("-d")
        .arg("logs_manager")
        .output()
        .expect("Error al ejecutar docker-compose up");

    if !output.status.success() {
        eprintln!(
            "❌ Error al crear el contenedor de logs: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return None;
    }

    // Obtenemos el ID del contenedor usando docker-compose ps
    let output_ps = Command::new("docker-compose")
        .current_dir(dir)
        .arg("ps")
        .arg("-q")
        .arg("logs_manager")
        .output()
        .expect("Error al ejecutar docker-compose ps");

    let stdout = String::from_utf8_lossy(&output_ps.stdout).trim().to_string();
    if stdout.is_empty() {
        eprintln!("❌ Error: no se obtuvo el ID del contenedor de logs.");
        None
    } else {
        println!("📂 Contenedor de logs creado: {}", stdout);
        Some(stdout)
    }
}


// Obtiene una lista de contenedores activos a través de `docker ps`, retorna un HashMap en el que la clave es el ID del contenedor y el valor es una tupla con el nombre y el comando.
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

//elimina un contenedor dado su ID.
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
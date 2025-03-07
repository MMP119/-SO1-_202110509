use serde::Deserialize;
use std::fs;
use std::process::Command;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Memory {
    #[serde(rename = "total_ram")]
    total_ram: String,
    #[serde(rename = "free_ram")]
    free_ram: String,
    #[serde(rename = "used_ram")]
    used_ram: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Container {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "pid")]
    pid: String,
    #[serde(rename = "memory_usage")]
    memory_usage: String,
    #[serde(rename = "cpu_usage")]
    cpu_usage: String,
    #[serde(rename = "io_usage")]
    io_usage: String,
    #[serde(rename = "disk_usage")]
    disk_usage: String,
}

#[derive(Debug, Deserialize)]
struct SysInfo {
    #[serde(rename = "Memory")]
    memory: Memory,
    #[serde(rename = "CPU_usage")]
    cpu_usage: String,
    #[serde(rename = "Containers")]
    containers: Vec<Container>,
}

// 📌 Función para leer el archivo de métricas del kernel
fn leer_metricas() -> Option<SysInfo> {
    let path = "/proc/sysinfo_202110509";

    match fs::read_to_string(path) {
        Ok(contents) => match serde_json::from_str::<SysInfo>(&contents) {
            Ok(data) => Some(data),
            Err(e) => {
                eprintln!("❌ Error al deserializar JSON: {}", e);
                None
            }
        },
        Err(e) => {
            eprintln!("❌ Error al leer el archivo: {}", e);
            None
        }
    }
}

// 📌 Función para obtener los contenedores activos de Docker
fn obtener_contenedores_docker() -> HashMap<String, (String, String)> {
    let output = Command::new("docker")
        .arg("ps")
        .arg("--format")
        .arg("{{.ID}} {{.Names}} {{.Command}}")  // Obtenemos el ID, el nombre de los contenedores y el comando ingresado
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
            contenedores.insert(id, (nombre, comando)); // ID -> (Nombre, Comando)
        }
    }

    contenedores
}


// 📌 Función para determinar qué contenedores eliminar
fn gestionar_contenedores(data: &SysInfo) -> Vec<String> {
    let contenedor_logs = "logs_manager"; // ⚠️ Nombre del contenedor de logs (no se debe eliminar)
    let mut eliminados: HashSet<String> = HashSet::new();
    
    let mut cpu_cont: Option<String> = None;
    let mut ram_cont: Option<String> = None;
    let mut io_cont: Option<String> = None;
    let mut disk_cont: Option<String> = None;

    // Obtener los contenedores activos de Docker (ID -> (Nombre, Comando))
    let contenedores_docker = obtener_contenedores_docker();

    for c in &data.containers {
        // 📌 Verificar si el contenedor está en ejecución y obtener su nombre y comando
        if let Some((nombre, comando)) = contenedores_docker.get(&c.id) {
            if nombre == contenedor_logs {
                continue; // 🚫 No eliminar el contenedor de logs
            }

            // Lógica de comparación para determinar qué contenedores mantener
            if comando.contains("cpu") {
                if cpu_cont.is_none() || c.id > *cpu_cont.as_ref().unwrap() {
                    cpu_cont = Some(c.id.clone());
                }
            } else if comando.contains("vm") {
                if ram_cont.is_none() || c.id > *ram_cont.as_ref().unwrap() {
                    ram_cont = Some(c.id.clone());
                }
            } else if comando.contains("io") {
                if io_cont.is_none() || c.id > *io_cont.as_ref().unwrap() {
                    io_cont = Some(c.id.clone());
                }
            } else if comando.contains("hdd") {
                if disk_cont.is_none() || c.id > *disk_cont.as_ref().unwrap() {
                    disk_cont = Some(c.id.clone());
                }
            }
        }
    }

    // Eliminar contenedores que no sean de tipo cpu, vm, io o hdd
    for c in &data.containers {
        if let Some((nombre, comando)) = contenedores_docker.get(&c.id) {
            if nombre == contenedor_logs {
                continue; // 🚫 No eliminar el contenedor de logs
            }
        }

        if Some(&c.id) != cpu_cont.as_ref()
            && Some(&c.id) != ram_cont.as_ref()
            && Some(&c.id) != io_cont.as_ref()
            && Some(&c.id) != disk_cont.as_ref()
        {
            eliminados.insert(c.id.clone());
        }
    }

    eliminados.into_iter().collect()
}


// 📌 Función para eliminar contenedores
fn eliminar_contenedores(contenedores: Vec<String>) {
    for contenedor_id in &contenedores {
        let output = Command::new("docker")
            .arg("rm")
            .arg("-f")
            .arg(contenedor_id)
            .output()
            .expect("Error al eliminar el contenedor");

        println!(
            "🗑 Eliminando contenedor {}: {}",
            contenedor_id,
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

fn main() {
    loop {
        println!("📌 Leyendo métricas del sistema...");

        if let Some(data) = leer_metricas() {
            println!("✅ Memoria Total: {}", data.memory.total_ram);
            println!("✅ Memoria Libre: {}", data.memory.free_ram);
            println!("✅ Memoria Usada: {}", data.memory.used_ram);
            println!("✅ Uso de CPU: {}", data.cpu_usage);

            let contenedores_a_eliminar = gestionar_contenedores(&data);

            if contenedores_a_eliminar.is_empty() {
                println!("✅ No se eliminaron contenedores.");
            } else {
                println!("🗑 Contenedores eliminados: {:?}", contenedores_a_eliminar);
                eliminar_contenedores(contenedores_a_eliminar);
            }
        }

        println!("⏳ Esperando 10 segundos...");
        thread::sleep(Duration::from_secs(10)); // ⏳ Espera 10 segundos antes de la siguiente iteración
    }
}
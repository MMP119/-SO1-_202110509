use serde::Deserialize;
use std::fs;
use std::process::Command;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use chrono::prelude::*;
use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    // Variable para indicar cuando finalizar el programa (se activa con Ctrl+C)
    static ref TERMINAR: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    static ref LOG_REGISTRO: Mutex<RegistroLogs> = Mutex::new(RegistroLogs::default());
}

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


#[derive(Debug)]
struct MemoryLog {
    total: String,
    free: String,
    used: String,
    timestamp: String,
}

#[derive(Debug, Clone)]
struct LogContainer {
    id: String,
    fecha_creacion: String,
    // Si el contenedor llega a eliminarse se actualizará este campo
    fecha_eliminacion: Option<String>,
}

#[derive(Debug, Default)]
struct RegistroLogs {
    memory_info: Option<MemoryLog>,
    cpu: Vec<LogContainer>,
    ram: Vec<LogContainer>,
    io: Vec<LogContainer>,
    disco: Vec<LogContainer>,
    eliminados: Vec<LogContainer>,
}

//función para crear el contenedor de logs
fn crear_contenedor_logs() -> Option<String> {
    let output = Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--name")
        .arg("logs_manager")
        .arg("-p")
        .arg("8000:8000")
        .arg("logs_container") // Dockerfile.logs
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



//función para leer el archivo de métricas del kernel
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


//función para obtener los contenedores activos de Docker
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



//función para determinar qué contenedores eliminar
fn gestionar_contenedores(data: &SysInfo, fecha: &str) -> Vec<String> {
    let contenedor_logs = "logs_manager"; // nombre del contenedor de logs (no se debe eliminar)
    let mut eliminados: HashSet<String> = HashSet::new();
    
    // Usamos variables para almacenar el contenedor "seleccionado" (el más nuevo) para cada categoría
    let mut cpu_cont: Option<String> = None;
    let mut ram_cont: Option<String> = None;
    let mut io_cont: Option<String> = None;
    let mut disk_cont: Option<String> = None;

    // Obtener los contenedores activos de Docker (ID -> (Nombre, Comando))
    let contenedores_docker = obtener_contenedores_docker();

    // Registrar la creación en los logs para cada contenedor según la categoría
    {
        let mut reg = LOG_REGISTRO.lock().unwrap();
        for c in &data.containers {
            if let Some((nombre, comando)) = contenedores_docker.get(&c.id) {
                if nombre == contenedor_logs {
                    continue; // NO eliminar el contenedor de logs
                }
                // Nuevo log para actualizar o crear
                let nueva_fecha = fecha.to_string();
                if comando.contains("cpu") {
                    cpu_cont = Some(c.id.clone());
                    if let Some(log_activo) = reg.cpu.iter_mut().find(|l| l.id == c.id && l.fecha_eliminacion.is_none()) {
                        // Actualizamos la fecha_creacion del log activo
                        log_activo.fecha_creacion = nueva_fecha;
                    } else {
                        // No existe un log activo, crearlo
                        let log = LogContainer {
                            id: c.id.clone(),
                            fecha_creacion: nueva_fecha,
                            fecha_eliminacion: None,
                        };
                        reg.cpu.push(log);
                    }
                } else if comando.contains("vm") {
                    ram_cont = Some(c.id.clone());
                    if let Some(log_activo) = reg.ram.iter_mut().find(|l| l.id == c.id && l.fecha_eliminacion.is_none()) {
                        log_activo.fecha_creacion = nueva_fecha;
                    } else {
                        let log = LogContainer {
                            id: c.id.clone(),
                            fecha_creacion: nueva_fecha,
                            fecha_eliminacion: None,
                        };
                        reg.ram.push(log);
                    }
                } else if comando.contains("io") {
                    io_cont = Some(c.id.clone());
                    if let Some(log_activo) = reg.io.iter_mut().find(|l| l.id == c.id && l.fecha_eliminacion.is_none()) {
                        log_activo.fecha_creacion = nueva_fecha;
                    } else {
                        let log = LogContainer {
                            id: c.id.clone(),
                            fecha_creacion: nueva_fecha,
                            fecha_eliminacion: None,
                        };
                        reg.io.push(log);
                    }
                } else if comando.contains("hdd") {
                    disk_cont = Some(c.id.clone());
                    if let Some(log_activo) = reg.disco.iter_mut().find(|l| l.id == c.id && l.fecha_eliminacion.is_none()) {
                        log_activo.fecha_creacion = nueva_fecha;
                    } else {
                        let log = LogContainer {
                            id: c.id.clone(),
                            fecha_creacion: nueva_fecha,
                            fecha_eliminacion: None,
                        };
                        reg.disco.push(log);
                    }
                }
            }
        }
    }
    
    // Eliminar contenedores que no sean los seleccionados en cada categoría
    for c in &data.containers {
        if let Some((nombre, _)) = contenedores_docker.get(&c.id) {
            if nombre == contenedor_logs {
                continue; // No eliminar el contenedor de logs
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



// Función para eliminar contenedores
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


fn manejar_ctrlc(_eliminados: Arc<Mutex<Vec<String>>>) {
    // Se utiliza TERMINAR.clone() porque TERMINAR es un Arc<AtomicBool>
    signal_hook::flag::register(signal_hook::consts::SIGINT, TERMINAR.clone())
        .expect("Error al registrar manejador de Ctrl+C");
    
    thread::spawn(move || {
        while !TERMINAR.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(100));
        }
        
        println!("\n🛑 Ctrl + C detectado. Cerrando programa...");
        
        let reg = LOG_REGISTRO.lock().unwrap();
        println!("──────────────────────────────────────────");
        println!("         Registro de Logs Final           ");
        println!("──────────────────────────────────────────");
        if let Some(mem) = &reg.memory_info {
            println!("Información de Memoria:");
            println!("  Total RAM: {}", mem.total);
            println!("  Free RAM: {}", mem.free);
            println!("  Used RAM: {}", mem.used);
            println!("  (Registrado a las: {})", mem.timestamp);
        }
        println!("──────────────────────────────────────────");
        println!();
        println!("Contenedores por categoría:");
        println!();

        // Función anónima para imprimir de forma formateada cada categoría
        let print_category = |name: &str, logs: &Vec<LogContainer>| {
            println!("  {}:", name);
            println!("    [");
            for log in logs {
                println!("        LogContainer ");
                println!("            {{");
                println!("                id: \"{}\",", log.id);
                println!("                fecha_creacion: \"{}\",", log.fecha_creacion);
                println!("                fecha_eliminacion: {}",
                    match &log.fecha_eliminacion {
                        Some(fe) => format!("Some(\"{}\")", fe),
                        None => "None".to_string(),
                    }
                );
                println!("            }},");
            }
            println!("    ]");
            println!();
        };

        print_category("CPU", &reg.cpu);
        print_category("RAM", &reg.ram);
        print_category("I/O", &reg.io);
        print_category("Disco", &reg.disco);

        println!("──────────────────────────────────────────");
        println!("Contenedores eliminados:");
        println!("    [");
        for log in &reg.eliminados {
            println!("        LogContainer ");
            println!("            {{");
            println!("                id: \"{}\",", log.id);
            println!("                fecha_creacion: \"{}\",", log.fecha_creacion);
            println!("                fecha_eliminacion: {}",
                match &log.fecha_eliminacion {
                    Some(fe) => format!("Some(\"{}\")", fe),
                    None => "None".to_string(),
                }
            );
            println!("            }},");
        }
        println!("    ]");
        println!("──────────────────────────────────────────");
        
        println!("📤 Enviando logs al servicio logs_manager...");
        // Aquí se implementaría la lógica para enviar los logs
        
        println!("✅ Programa finalizado correctamente.");
        std::process::exit(0);
    });
}



fn main() {

    println!("🚀 Iniciando servicio de gestión de contenedores...");

    let eliminados = Arc::new(Mutex::new(Vec::new()));
    manejar_ctrlc(eliminados.clone());

    let _ = match crear_contenedor_logs() { //id_contenedor_logs
        Some(id) => id,
        None => {
            eprintln!("⚠️ No se pudo crear el contenedor de logs. Abortando...");
            return;
        }
    };

    loop {

        if TERMINAR.load(Ordering::Relaxed) {
            break;
        }

        println!("📌 Leyendo métricas del sistema...");

        if let Some(data) = leer_metricas() {

            // Actualizar registro de logs con la info de memoria
            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            {
                let mut reg = LOG_REGISTRO.lock().unwrap();
                reg.memory_info = Some(MemoryLog{
                    total: data.memory.total_ram.clone(),
                    free: data.memory.free_ram.clone(),
                    used: data.memory.used_ram.clone(),
                    timestamp: now.clone(),
                });
            }
            
            println!("✅ Memoria Total: {}", data.memory.total_ram);
            println!("✅ Memoria Libre: {}", data.memory.free_ram);
            println!("✅ Memoria Usada: {}", data.memory.used_ram);
            println!("✅ Uso de CPU: {}", data.cpu_usage);

            // Se llama a gestionar_contenedores para obtener id's a eliminar
            // Además se agruparán los contenedores en cada categoría
            let contenedores_a_eliminar = gestionar_contenedores(&data, &now);

            if contenedores_a_eliminar.is_empty() {
                println!("✅ No se eliminaron contenedores.");
            } else {
                println!("🗑 Contenedores eliminados: {:?}", contenedores_a_eliminar);

                // Marcar fecha de eliminación en los logs antes de eliminarlos
                {
                    let mut reg = LOG_REGISTRO.lock().unwrap();
                
                    // Para CPU
                    let mut temp_cpu = Vec::new();
                    for log in reg.cpu.iter_mut() {
                        if contenedores_a_eliminar.contains(&log.id) {
                            log.fecha_eliminacion = Some(now.clone());
                            temp_cpu.push(log.clone());
                        }
                    }
                    reg.eliminados.extend(temp_cpu);
                
                    // Para RAM
                    let mut temp_ram = Vec::new();
                    for log in reg.ram.iter_mut() {
                        if contenedores_a_eliminar.contains(&log.id) {
                            log.fecha_eliminacion = Some(now.clone());
                            temp_ram.push(log.clone());
                        }
                    }
                    reg.eliminados.extend(temp_ram);
                
                    // Para I/O
                    let mut temp_io = Vec::new();
                    for log in reg.io.iter_mut() {
                        if contenedores_a_eliminar.contains(&log.id) {
                            log.fecha_eliminacion = Some(now.clone());
                            temp_io.push(log.clone());
                        }
                    }
                    reg.eliminados.extend(temp_io);
                
                    // Para Disco
                    let mut temp_disco = Vec::new();
                    for log in reg.disco.iter_mut() {
                        if contenedores_a_eliminar.contains(&log.id) {
                            log.fecha_eliminacion = Some(now.clone());
                            temp_disco.push(log.clone());
                        }
                    }
                    reg.eliminados.extend(temp_disco);
                }

                eliminar_contenedores(contenedores_a_eliminar);
            }
        }

        println!("⏳ Esperando 10 segundos...");
        thread::sleep(Duration::from_secs(10)); //Espera 10 segundos antes de la siguiente iteración
    }
}
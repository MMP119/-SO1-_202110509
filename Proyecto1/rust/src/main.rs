use std::fs;
use serde::{Deserialize};



#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Silencia las advertencias de campos no utilizados
struct Memory {
    #[serde(rename = "total_ram")] // Nombre en el JSON
    total_ram: String,
    #[serde(rename = "free_ram")] // Nombre en el JSON
    free_ram: String,
    #[serde(rename = "used_ram")] // Nombre en el JSON
    used_ram: String,
}



#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Silencia las advertencias de campos no utilizados
struct Container {
    #[serde(rename = "id")] // Nombre en el JSON
    id: String,
    #[serde(rename = "name")] // Nombre en el JSON
    name: String,
    #[serde(rename = "pid")] // Nombre en el JSON
    pid: String,
    #[serde(rename = "memory_usage")] // Nombre en el JSON
    memory_usage: String,
    #[serde(rename = "cpu_usage")] // Nombre en el JSON
    cpu_usage: String,
    #[serde(rename = "io_usage")] // Nombre en el JSON
    io_usage: String,
    #[serde(rename = "disk_usage")] // Nombre en el JSON
    disk_usage: String,
}



#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Silencia las advertencias de campos no utilizados
struct SysInfo {
    #[serde(rename = "Memory")] // Nombre en el JSON
    memory: Memory,
    #[serde(rename = "CPU_usage")] // Nombre en el JSON
    cpu_usage: String,
    #[serde(rename = "Containers")] // Nombre en el JSON
    containers: Vec<Container>,
}



fn clasificar_contenedores(contenedores: &Vec<Container>) {
    let mut contenedores_cpu = Vec::new();
    let mut contenedores_ram = Vec::new();
    let mut contenedores_io = Vec::new();
    let mut contenedores_disco = Vec::new();

    for cont in contenedores {
        let cpu_uso = cont.cpu_usage.replace("%", "").parse::<u32>().unwrap_or(0);
        let mem_uso = cont.memory_usage.replace(" MiB", "").parse::<u32>().unwrap_or(0);
        let io_uso = cont.io_usage.replace(" ops", "").parse::<u32>().unwrap_or(0);
        let disk_uso = cont.disk_usage.replace(" MiB", "").parse::<u32>().unwrap_or(0);

        if cpu_uso > 0 {
            contenedores_cpu.push(cont);
        }
        if mem_uso > 0 {
            contenedores_ram.push(cont);
        }
        if io_uso > 0 {
            contenedores_io.push(cont);
        }
        if disk_uso > 0 {
            contenedores_disco.push(cont);
        }
    }

    println!("=== Contenedores CPU ===");
    for c in &contenedores_cpu {
        println!("{:?}", c);
    }

    println!("\n=== Contenedores RAM ===");
    for c in &contenedores_ram {
        println!("{:?}", c);
    }

    println!("\n=== Contenedores I/O ===");
    for c in &contenedores_io {
        println!("{:?}", c);
    }

    println!("\n=== Contenedores Disco ===");
    for c in &contenedores_disco {
        println!("{:?}", c);
    }
}





fn main() {
    let path = "/proc/sysinfo_202110509"; // Para pruebas, luego cambiar a "/proc/sysinfo_hcarnet"

    match fs::read_to_string(path) {
        Ok(contents) => {
            match serde_json::from_str::<SysInfo>(&contents) {
                Ok(data) => {
                    println!("Datos obtenidos: {:#?}", data);

                    clasificar_contenedores(&data.containers);
                }
                Err(e) => eprintln!("Error al deserializar JSON: {}", e),
            }
        }
        Err(e) => eprintln!("Error al leer el archivo: {}", e),
    }
}





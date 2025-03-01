use std::fs;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Memory {
    total_ram: String,
    free_ram: String,
    used_ram: String,
}

#[derive(Debug, Deserialize)]
struct Container {
    id: String,
    name: String,
    pid: String,
    memory_usage: String,
    cpu_usage: String,
    io_usage: String,
    disk_usage: String,
}

#[derive(Debug, Deserialize)]
struct SysInfo {
    Memory: Memory,
    CPU_usage: String,
    Containers: Vec<Container>,
}

fn main() {
    let path = "/proc/sysinfo_202110509"; // Para pruebas, luego cambiar a "/proc/sysinfo_hcarnet"

    match fs::read_to_string(path) {
        Ok(contents) => {
            match serde_json::from_str::<SysInfo>(&contents) {
                Ok(data) => {
                    println!("Datos obtenidos: {:#?}", data);
                }
                Err(e) => eprintln!("Error al deserializar JSON: {}", e),
            }
        }
        Err(e) => eprintln!("Error al leer el archivo: {}", e),
    }
}
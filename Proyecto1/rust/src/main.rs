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
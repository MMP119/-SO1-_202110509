use std::fs;
use serde::Deserialize;

// Representa la información de memoria deserializada desde el JSON
#[derive(Debug, Deserialize)]
pub struct Memory {
    #[serde(rename = "total_ram")]
    pub total_ram: String,
    #[serde(rename = "free_ram")]
    pub free_ram: String,
    #[serde(rename = "used_ram")]
    pub used_ram: String,
    
}

#[derive(Debug, Deserialize)]
pub struct CPU {
    pub cpu_usage: String,
}

// Representa la información de un contenedor extraída del JSON
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Container {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pid")]
    pub pid: String,
    #[serde(rename = "memory_usage")]
    pub memory_usage: String,
    #[serde(rename = "cpu_usage")]
    pub cpu_usage: String,
    #[serde(rename = "io_usage")]
    pub io_usage: String,
    #[serde(rename = "disk_usage")]
    pub disk_usage: String,
}

// Estructura principal que agrupa las métricas del sistema
#[derive(Debug, Deserialize)]
pub struct SysInfo {
    #[serde(rename = "Memory")]
    pub memory: Memory,
    #[serde(rename = "CPU_usage")]
    pub cpu_usage: CPU,
    #[serde(rename = "Containers")]
    pub containers: Vec<Container>,
}

// Lee el archivo de métricas ubicado en `/proc/sysinfo_202110509` y lo deserializa en una estructura SysInfo
pub fn leer_metricas() -> Option<SysInfo> {
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

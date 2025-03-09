use crate::metrics::SysInfo;
use std::collections::HashSet;
use serde::Serialize; // Asegúrate de tener la dependencia serde con el feature "derive" en Cargo.toml

/// Estructura para almacenar la información de la memoria junto con el timestamp.
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct MemoryLog {
    pub total: String,
    pub free: String,
    pub used: String,
    pub cpu_usage: String,
    pub timestamp: String,
}


#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CpuLog{
    pub cpu_usage: String,
}

/// Estructura para almacenar la información de cada contenedor en el log.
/// Se conserva la fecha de creación original y se actualiza la fecha de eliminación cuando corresponda.
#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct LogContainer {
    pub id: String,
    pub fecha_creacion: String,
    pub fecha_eliminacion: Option<String>,
    pub metric: Option<String>,
}

/// Registro global de logs, que agrupa la información de memoria y los contenedores por categoría.
#[derive(Debug, Default, Serialize)]
pub struct RegistroLogs {
    pub memory_info: Option<MemoryLog>,
    pub cpu: Vec<LogContainer>,
    pub ram: Vec<LogContainer>,
    pub io: Vec<LogContainer>,
    pub disco: Vec<LogContainer>,
    pub eliminados: Vec<LogContainer>,
}

impl RegistroLogs {
    /// Actualiza la información de memoria en el log.
    pub fn actualizar_memoria(&mut self, mem: &crate::metrics::Memory, cpu: &crate::metrics::CPU, now: String) {
        self.memory_info = Some(MemoryLog {
            total: mem.total_ram.clone(),
            free: mem.free_ram.clone(),
            used: mem.used_ram.clone(),
            cpu_usage: cpu.cpu_usage.clone(),
            timestamp: now,
        });
    }
}

/// Trait para marcar en el log la fecha de eliminación de los contenedores.
pub trait MarcarEliminacion {
    fn marcar_eliminacion(&mut self, ids: &Vec<String>, fecha: &str);
}

impl MarcarEliminacion for RegistroLogs {
    fn marcar_eliminacion(&mut self, ids: &Vec<String>, fecha: &str) {
        // Itera sobre todos los logs de contenedores en las categorías CPU, RAM, I/O y Disco.
        for log in self.cpu.iter_mut()
            .chain(self.ram.iter_mut())
            .chain(self.io.iter_mut())
            .chain(self.disco.iter_mut())
        {
            if ids.contains(&log.id) && log.fecha_eliminacion.is_none() {
                log.fecha_eliminacion = Some(fecha.to_string());
                self.eliminados.push(log.clone());
            }
        }
    }
}

/// Estructura que se enviará a la API de logs.
#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub memory_total: String,
    pub memory_free: String,
    pub memory_used: String,
    pub cpu_usage: String,
    pub cpu: Vec<LogContainer>,
    pub ram: Vec<LogContainer>,
    pub io: Vec<LogContainer>,
    pub disco: Vec<LogContainer>,
}

impl RegistroLogs {
    /// Convierte el RegistroLogs en un LogEntry para enviar a la API.
    /// Recibe el timestamp final y el uso de CPU (puedes ajustarlo según lo requieras).
    pub fn to_log_entry(&self, timestamp: String) -> LogEntry {
        let (total, free, used, cpu) = if let Some(ref mem) = self.memory_info {
            (mem.total.clone(), mem.free.clone(), mem.used.clone(), mem.cpu_usage.clone())
        } else {
            ("".to_string(), "".to_string(), "".to_string(), "".to_string())
        };

        LogEntry {
            timestamp,
            memory_total: total,
            memory_free: free,
            memory_used: used,
            cpu_usage: cpu,
            cpu: self.cpu.clone(),
            ram: self.ram.clone(),
            io: self.io.clone(),
            disco: self.disco.clone(),
        }
    }
}

/// Función para gestionar la agrupación de contenedores y determinar cuáles eliminar.
/// Se actualiza o crea el log para cada contenedor según su categoría, conservando la fecha de creación si ya existe.
pub fn gestionar_contenedores(data: &SysInfo, fecha: &str) -> Vec<String> {
    let contenedor_logs = "logs_manager"; // Contenedor de logs, no se debe eliminar
    let mut eliminados = HashSet::new();

    // Variables para almacenar el contenedor "más nuevo" por categoría.
    let mut cpu_cont: Option<String> = None;
    let mut ram_cont: Option<String> = None;
    let mut io_cont: Option<String> = None;
    let mut disk_cont: Option<String> = None;

    // Obtener la lista de contenedores activos a través del módulo docker.
    let contenedores_docker = crate::docker::obtener_contenedores_docker();

    {
        // Accedemos al registro global de logs para actualizar o crear entradas.
        let reg = &mut crate::LOG_REGISTRO.lock().unwrap();
        for c in &data.containers {
            if let Some((nombre, comando)) = contenedores_docker.get(&c.id) {
                if nombre == contenedor_logs {
                    continue; // No procesamos el contenedor de logs.
                }
                // Según la métrica (determinada por el comando) se agrupa el contenedor.
                if comando.contains("cpu") {
                    cpu_cont = Some(c.id.clone());
                    if !reg.cpu.iter().any(|l| l.id == c.id && l.fecha_eliminacion.is_none()) {
                        reg.cpu.push(LogContainer {
                            id: c.id.clone(),
                            fecha_creacion: fecha.to_string(),
                            fecha_eliminacion: None,
                            metric: Some(c.cpu_usage.clone()),
                        });
                    }
                } else if comando.contains("vm") {
                    ram_cont = Some(c.id.clone());
                    if !reg.ram.iter().any(|l| l.id == c.id && l.fecha_eliminacion.is_none()) {
                        reg.ram.push(LogContainer {
                            id: c.id.clone(),
                            fecha_creacion: fecha.to_string(),
                            fecha_eliminacion: None,
                            metric: Some(c.memory_usage.clone()),
                        });
                    }
                } else if comando.contains("io") {
                    io_cont = Some(c.id.clone());
                    if !reg.io.iter().any(|l| l.id == c.id && l.fecha_eliminacion.is_none()) {
                        reg.io.push(LogContainer {
                            id: c.id.clone(),
                            fecha_creacion: fecha.to_string(),
                            fecha_eliminacion: None,
                            metric: Some(c.io_usage.clone()),
                        });
                    }
                } else if comando.contains("hdd") {
                    disk_cont = Some(c.id.clone());
                    if !reg.disco.iter().any(|l| l.id == c.id && l.fecha_eliminacion.is_none()) {
                        reg.disco.push(LogContainer {
                            id: c.id.clone(),
                            fecha_creacion: fecha.to_string(),
                            fecha_eliminacion: None,
                            metric: Some(c.disk_usage.clone()),
                        });
                    }
                }
            }
        }
    }

    // Determinar contenedores a eliminar: aquellos que no sean el "más nuevo" de cada categoría.
    for c in &data.containers {
        if let Some((nombre, _)) = contenedores_docker.get(&c.id) {
            if nombre == contenedor_logs {
                continue;
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

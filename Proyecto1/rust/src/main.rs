mod docker;
mod metrics;
mod logs;

use crate::logs::{RegistroLogs, MarcarEliminacion};
use chrono::prelude::*;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;


lazy_static! {
    // Variable para indicar cuándo finalizar el programa (se activa con Ctrl+C)
    pub static ref TERMINAR: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    pub static ref LOG_REGISTRO: Mutex<RegistroLogs> = Mutex::new(RegistroLogs::default());
}



fn enviar_logs_api(log: &logs::LogEntry) -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://localhost:8000/logs";
    let client = reqwest::blocking::Client::new();
    let response = client.post(url).json(log).send()?;

    if response.status().is_success() {
        println!("Logs enviados correctamente.");
    } else {
        eprintln!("Error al enviar logs. Código de estado: {}", response.status());
    }

    Ok(())
}



fn main() {

    println!("🚀 Iniciando servicio de gestión de contenedores...");

    signal_hook::flag::register(signal_hook::consts::SIGINT, TERMINAR.clone())
        .expect("Error al registrar manejador Ctrl+C");

    let _id_logs = match docker::crear_contenedor_logs() {
        Some(id) => id,
        None => {
            eprintln!("⚠️ No se pudo crear el contenedor de logs. Abortando...");
            return;
        }
    };

    loop {

        if TERMINAR.load(Ordering::Relaxed) {
            println!("\n🛑 Ctrl + C detectado. Cerrando programa...");
            
            let final_timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            let registro_final = {
                let reg = LOG_REGISTRO.lock().unwrap();
                reg.to_log_entry(final_timestamp)
            };

            println!("──────────────────────────────────────────");
            println!("         Registro de Logs Final           ");
            println!("──────────────────────────────────────────");
            println!("{:#?}", registro_final);
            println!("📤 Enviando logs al servicio logs_manager...");

            if let Err(e) = enviar_logs_api(&registro_final) {
                eprintln!("Error al enviar logs finales: {}", e);
            } 

            println!("✅ Programa finalizado correctamente.");
            break;
        }

        println!("LEYENDO METRICAS...");

        if let Some(data) = metrics::leer_metricas() {
            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            {
                let mut reg = LOG_REGISTRO.lock().unwrap();
                reg.actualizar_memoria(&data.memory, &data.cpu_usage, now.clone());
            }

            println!("--Memoria Total: {}", data.memory.total_ram);
            println!("--Memoria Libre: {}", data.memory.free_ram);
            println!("--Memoria Usada: {}", data.memory.used_ram);
            println!("--Uso de CPU: {}", data.cpu_usage.cpu_usage);

            let contenedores_a_eliminar = logs::gestionar_contenedores(&data, &now);

            if !contenedores_a_eliminar.is_empty() {
                let elimination_time = (Local::now() + chrono::Duration::seconds(3))
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string();

                {
                    let mut reg = LOG_REGISTRO.lock().unwrap();
                    reg.marcar_eliminacion(&contenedores_a_eliminar, &elimination_time);
                }

                let handles: Vec<_> = contenedores_a_eliminar.into_iter().map(|container_id| {
                    thread::spawn(move || {
                        docker::eliminar_contenedor(container_id);
                    })
                }).collect();

                for handle in handles {
                    handle.join().expect("Error en hilo de eliminación");
                }
            }
        }

        println!("Esperando 10 segundos...");
        
        for _ in 0..100 {
            if TERMINAR.load(Ordering::Relaxed) {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
}
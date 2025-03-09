mod docker;
mod metrics;
mod logs;
mod signal_handler;

use crate::logs::MarcarEliminacion;
use crate::logs::RegistroLogs;
use chrono::prelude::*;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;


lazy_static! {
    // Variable para indicar cuando finalizar el programa (se activa con Ctrl+C)
    static ref TERMINAR: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    static ref LOG_REGISTRO: Mutex<RegistroLogs> = Mutex::new(RegistroLogs::default());
}

fn main() {
    println!("🚀 Iniciando servicio de gestión de contenedores...");

    // Iniciar el manejador de señal (Ctrl+C)
    signal_handler::iniciar_manejador_ctrlc();

    // Crear el contenedor de logs y abortar si falla
    let _id_logs = match docker::crear_contenedor_logs() {
        Some(id) => id,
        None => {
            eprintln!("⚠️ No se pudo crear el contenedor de logs. Abortando...");
            return;
        }
    };

    // Bucle principal que se ejecuta cada 10 segundos
    loop {
        // Verificar si se ha solicitado la terminación del servicio
        if TERMINAR.load(Ordering::Relaxed) {
            break;
        }

        println!("📌 Leyendo métricas del sistema...");

        // Leer métricas desde el archivo generado por el módulo del kernel
        if let Some(data) = metrics::leer_metricas() {
            // Obtener el timestamp actual
            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            // Actualizar la información de memoria en el registro global de logs
            {
                let mut reg = LOG_REGISTRO.lock().unwrap();
                reg.actualizar_memoria(&data.memory, now.clone());
            }

            // Imprimir algunas métricas para monitoreo
            println!("✅ Memoria Total: {}", data.memory.total_ram);
            println!("✅ Memoria Libre: {}", data.memory.free_ram);
            println!("✅ Memoria Usada: {}", data.memory.used_ram);
            println!("✅ Uso de CPU: {}", data.cpu_usage);

            // Gestionar la agrupación de logs y determinar qué contenedores eliminar
            let contenedores_a_eliminar = logs::gestionar_contenedores(&data, &now);

            if !contenedores_a_eliminar.is_empty() {

                let elimination_time = (Local::now() + chrono::Duration::seconds(3)).format("%Y-%m-%d %H:%M:%S").to_string();

                {
                    // Marcar en los logs la fecha de eliminación antes de borrar los contenedores
                    let mut reg = LOG_REGISTRO.lock().unwrap();
                    reg.marcar_eliminacion(&contenedores_a_eliminar, &elimination_time);
                }

                // Ejecutar la eliminación de contenedores en hilos separados
                let handles: Vec<_> = contenedores_a_eliminar.into_iter().map(|container_id| {

                    thread::spawn(move || {

                        docker::eliminar_contenedor(container_id);

                    })
                    
                }).collect();

                // Esperar a que todos los hilos terminen
                for handle in handles {
                    handle.join().expect("Error en hilo de eliminación");
                }
            }
        }

        println!("⏳ Esperando 10 segundos...");
        thread::sleep(Duration::from_secs(10));
    }
}
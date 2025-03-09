use std::sync::atomic::{Ordering};
use std::thread;
use std::time::Duration;
use crate::LOG_REGISTRO;

/// Inicia el manejador para la señal Ctrl+C (SIGINT).
/// Se registra el flag global `TERMINAR` y se lanza un hilo que monitorea dicho flag.
/// Cuando se detecta la señal, se imprime el registro final de logs, se simula el envío al servicio
/// de logs y se finaliza el programa.
pub fn iniciar_manejador_ctrlc() {
    // Clona el flag global de terminación
    let flag = crate::TERMINAR.clone();
    // Registra el flag para la señal SIGINT
    signal_hook::flag::register(signal_hook::consts::SIGINT, flag)
        .expect("Error al registrar el manejador de Ctrl+C");

    // Lanzar un hilo que monitorea el flag y actúa al detectar la señal
    thread::spawn(|| {
        let flag = crate::TERMINAR.clone();
        // Monitorea el flag cada 100 ms
        while !flag.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(100));
        }
        
        // Una vez detectada la señal, se procede a la finalización ordenada
        println!("\n🛑 Ctrl + C detectado. Cerrando programa...");
        let reg = LOG_REGISTRO.lock().unwrap();
        println!("──────────────────────────────────────────");
        println!("         Registro de Logs Final           ");
        println!("──────────────────────────────────────────");
        println!("{:#?}", *reg);
        println!("📤 Enviando logs al servicio logs_manager...");
        println!("✅ Programa finalizado correctamente.");
        std::process::exit(0);
    });
}

#!/bin/bash
# start_project.sh

#función que crea contenedores cada 10 segundos durante 60 segundos
function iniciar_creacion_contenedores() {
    local end_time=$(( $(date +%s) + 30 ))
    while [ $(date +%s) -lt $end_time ]; do
        #ejecuta el script de creación de contenedores ubicado en la carpeta 'contenedores'
        ./contenedores/create.sh
        sleep 10
    done
}

echo "Iniciando la creación de contenedores durante 30 segundos..."
iniciar_creacion_contenedores &
PID_CREACION=$!
echo "PID de creación de contenedores: $PID_CREACION"

echo "Esperando 10 segundos..."
sleep 10

#inicia el servicio Rust
echo "Iniciando el servicio Rust..."
cd rust || { echo "Directorio rust no encontrado"; exit 1; }
cargo run &
PID_RUST=$!
echo "PID del servicio Rust: $PID_RUST"
cd ..

#espera a que el servicio Rust finalice
wait $PID_RUST

echo "Servicio Rust finalizado. Deteniendo la creación de contenedores..."
kill $PID_CREACION

echo "Proyecto finalizado."

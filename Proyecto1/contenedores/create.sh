#!/bin/bash

# Definir el número de contenedores de estrés
NUM_CONTAINERS=6

# Imagen base de Docker
IMAGE="containerstack/alpine-stress"

# Comandos de carga para cada tipo de contenedor
STRESS_RAM="--vm 1 --vm-bytes 128M"
STRESS_CPU="--cpu 1"
STRESS_IO="--io 1"
STRESS_DISK="--hdd 1 --hdd-bytes 128M"

# Lista de tipos de contenedores
STRESS_TYPES=("$STRESS_RAM" "$STRESS_CPU" "$STRESS_IO" "$STRESS_DISK")


# CREAR UN CONTENEDOR DE CADA TIPO DE ESTRÉS
for STRESS_CMD in "${STRESS_TYPES[@]}"; do
    CONTAINER_NAME="container_$(date +%s%N | cut -c1-13)"
    docker run -d --cpus="0.2" --memory="128m" --name "$CONTAINER_NAME" "$IMAGE" stress $STRESS_CMD &
    echo "Contenedor creado: $CONTAINER_NAME ($STRESS_CMD)"
done

wait # Esperar a que todos los contenedores del primer bloque se hayan creado


# CREAR 6 NUEVOS CONTENEDORES DE ESTRÉS
for ((i=0; i<NUM_CONTAINERS; i++)); do
    # Seleccionar aleatoriamente un tipo de estrés
    STRESS_CMD=${STRESS_TYPES[$RANDOM % ${#STRESS_TYPES[@]}]}
    
    # Generar un nombre único basado en la fecha
    CONTAINER_NAME="container_$(date +%s%N | cut -c1-13)"

    # Crear el contenedor
    docker run -d --cpus="0.2" --memory="128m" --name "$CONTAINER_NAME" "$IMAGE" stress $STRESS_CMD &

    echo "Contenedor creado: $CONTAINER_NAME ($STRESS_CMD)"

done

wait # Esperar a que todos los contenedores del segundo bloque se hayan creado

echo "Se han creado 10 nuevos contenedores de estrés."

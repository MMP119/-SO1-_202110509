#!/bin/bash

# Definir el número de contenedores de estrés
NUM_CONTAINERS=10

# Imagen base de Docker
IMAGE="containerstack/alpine-stress"

# Contenedor especial para logs (no se elimina)
LOGS_CONTAINER_NAME="logs_manager"
LOGS_IMAGE="python:3.9"

# Comandos de carga para cada tipo de contenedor
STRESS_RAM="--vm 1 --vm-bytes 128M"
STRESS_CPU="--cpu 1"
STRESS_IO="--io 1"
STRESS_DISK="--hdd 1 --hdd-bytes 128M"

# Lista de tipos de contenedores
STRESS_TYPES=("$STRESS_RAM" "$STRESS_CPU" "$STRESS_IO" "$STRESS_DISK")

# -------------------------------
# 1️⃣ CREAR CONTENEDOR DE LOGS SI NO EXISTE
# -------------------------------
if ! docker ps -a --format '{{.Names}}' | grep -q "$LOGS_CONTAINER_NAME"; then
    echo "📂 Creando contenedor de logs: $LOGS_CONTAINER_NAME"
    docker run -d --name "$LOGS_CONTAINER_NAME" -v $(pwd)/logs:/app/logs "$LOGS_IMAGE" tail -f /dev/null
else
    echo "📂 Contenedor de logs ya está en ejecución."
fi

# -------------------------------
# 2️⃣ ELIMINAR TODOS LOS CONTENEDORES DE ESTRÉS
# -------------------------------
echo "🗑 Eliminando contenedores de estrés antiguos..."
docker ps -aq --filter "name=container_" | xargs -r docker rm -f

# -------------------------------
# 3️⃣ CREAR 10 NUEVOS CONTENEDORES DE ESTRÉS
# -------------------------------
for ((i=0; i<NUM_CONTAINERS; i++)); do
    # Seleccionar aleatoriamente un tipo de estrés
    STRESS_CMD=${STRESS_TYPES[$RANDOM % ${#STRESS_TYPES[@]}]}
    
    # Generar un nombre único basado en la fecha
    CONTAINER_NAME="container_$(date +%s%N | cut -c1-13)"

    # Crear el contenedor
    docker run -d --cpus="0.2" --memory="128m" --name "$CONTAINER_NAME" "$IMAGE" stress $STRESS_CMD

    echo "📦 Contenedor creado: $CONTAINER_NAME ($STRESS_CMD)"
done

echo "✅ Se han creado 10 nuevos contenedores de estrés."

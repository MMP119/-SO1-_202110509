Iniciar el script:
<br>
./create_containers.sh
<br>
<br>

Eliminar todos los contenedores:
<br>
docker rm -f $(docker ps -aq) 
<br>
<br>

Detener solo los contenedores de estrés y no el log:
<br>
docker rm -f $(docker ps -aq --filter "name=container_")
<br>
<br>

AUTOMATIZACION CON CRONJOB
<br>
crontab -e
<br>
Ruta al final del archivo:
<br>
* * * * * /bin/bash /home/mario/Escritorio/GitHub/-SO1-_202110509/Proyecto1/contenedores/create_containers.sh
<br>
Para cada 30 segundos:
<br>
* * * * * sleep 30; /bin/bash /home/mario/Escritorio/GitHub/-SO1-_202110509/Proyecto1/contenedores/create_containers.sh



* * * * * /bin/bash /home/mario/Escritorio/GitHub/-SO1-_202110509/Proyecto1/contenedores/create_containers.sh
* * * * * sleep 30; /bin/bash /home/mario/Escritorio/GitHub/-SO1-_202110509/Proyecto1/contenedores/create_containers.sh


## para el contenedor logs_manager

curl -X POST "http://localhost:8000/logs" -H "Content-Type: application/json" -d '{"test": "funciona"}'


docker exec logs_manager cat /app/logs/metrics.json






──────────────────────────────────────────
         Registro de Logs Final           
──────────────────────────────────────────
Información de Memoria:
  Total RAM: 15236 MB
  Free RAM: 3146 MB
  Used RAM: 12090 MB
  (Registrado a las: 2025-03-08 12:24:59)
──────────────────────────────────────────

Contenedores por categoría:

  CPU:
    [
        LogContainer 
            {
                id: "3df588b3a45b",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: None
            },
        LogContainer 
            {
                id: "3df588b3a45b",
                fecha_creacion: "2025-03-08 12:24:59",
                fecha_eliminacion: None
            },
    ]

  RAM:
    [
        LogContainer 
            {
                id: "e737760261dc",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
        LogContainer 
            {
                id: "38a5da70a223",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
        LogContainer 
            {
                id: "fe633c2f56ad",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: None
            },
        LogContainer 
            {
                id: "fe633c2f56ad",
                fecha_creacion: "2025-03-08 12:24:59",
                fecha_eliminacion: None
            },
    ]

  I/O:
    [
        LogContainer 
            {
                id: "2c2eace94199",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
        LogContainer 
            {
                id: "98ca1753f02a",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
        LogContainer 
            {
                id: "e716e4942c28",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: None
            },
        LogContainer 
            {
                id: "e716e4942c28",
                fecha_creacion: "2025-03-08 12:24:59",
                fecha_eliminacion: None
            },
    ]

  Disco:
    [
        LogContainer 
            {
                id: "0e65b7302a46",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
        LogContainer 
            {
                id: "38f5954940a6",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
        LogContainer 
            {
                id: "3f6bf4f8d9f5",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: None
            },
        LogContainer 
            {
                id: "3f6bf4f8d9f5",
                fecha_creacion: "2025-03-08 12:24:59",
                fecha_eliminacion: None
            },
    ]

──────────────────────────────────────────
Contenedores eliminados:
    [
        LogContainer 
            {
                id: "e737760261dc",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
        LogContainer 
            {
                id: "38a5da70a223",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
        LogContainer 
            {
                id: "2c2eace94199",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
        LogContainer 
            {
                id: "98ca1753f02a",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
        LogContainer 
            {
                id: "0e65b7302a46",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
        LogContainer 
            {
                id: "38f5954940a6",
                fecha_creacion: "2025-03-08 12:24:42",
                fecha_eliminacion: Some("2025-03-08 12:24:42")
            },
    ]
──────────────────────────────────────────
📤 Enviando logs al servicio logs_manager...
✅ Programa finalizado correctamente.
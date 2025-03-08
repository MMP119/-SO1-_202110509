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
  Free RAM: 6361 MB
  Used RAM: 8875 MB
  (Registrado a las: 2025-03-08 16:47:50)
──────────────────────────────────────────

Contenedores por categoría:

  CPU:
    [
        LogContainer 
            {
                id: "ded64587eb51",
                fecha_creacion: "2025-03-08 16:47:50",
                metric: "21%",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
        LogContainer 
            {
                id: "912e5c0899bd",
                fecha_creacion: "2025-03-08 16:47:50",
                metric: "20%",
                fecha_eliminacion: None
            },
    ]

  RAM:
    [
        LogContainer 
            {
                id: "8ab6e28b0f60",
                fecha_creacion: "2025-03-08 16:47:50",
                metric: "128 MiB",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
        LogContainer 
            {
                id: "446caee341f6",
                fecha_creacion: "2025-03-08 16:47:50",
                metric: "7 MiB",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
        LogContainer 
            {
                id: "78b5ce0aa10d",
                fecha_creacion: "2025-03-08 16:47:50",
                metric: "127 MiB",
                fecha_eliminacion: None
            },
    ]

  I/O:
    [
        LogContainer 
            {
                id: "2dd0986e151a",
                fecha_creacion: "2025-03-08 16:47:50",
                metric: "3228 ops",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
        LogContainer 
            {
                id: "d9ad85e4ae12",
                fecha_creacion: "2025-03-08 16:47:50",
                metric: "3663 ops",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
        LogContainer 
            {
                id: "f547d672e2a8",
                fecha_creacion: "2025-03-08 16:47:50",
                metric: "3737 ops",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
        LogContainer 
            {
                id: "96c9b9826394",
                fecha_creacion: "2025-03-08 16:47:50",
                metric: "3697 ops",
                fecha_eliminacion: None
            },
    ]

  Disco:
    [
        LogContainer 
            {
                id: "5232938348d7",
                fecha_creacion: "2025-03-08 16:47:50",
                metric: "3982 MiB",
                fecha_eliminacion: None
            },
    ]

──────────────────────────────────────────
Contenedores eliminados:
    [
        LogContainer 
            {
                id: "ded64587eb51",
                fecha_creacion: "2025-03-08 16:47:50",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
        LogContainer 
            {
                id: "8ab6e28b0f60",
                fecha_creacion: "2025-03-08 16:47:50",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
        LogContainer 
            {
                id: "446caee341f6",
                fecha_creacion: "2025-03-08 16:47:50",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
        LogContainer 
            {
                id: "2dd0986e151a",
                fecha_creacion: "2025-03-08 16:47:50",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
        LogContainer 
            {
                id: "d9ad85e4ae12",
                fecha_creacion: "2025-03-08 16:47:50",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
        LogContainer 
            {
                id: "f547d672e2a8",
                fecha_creacion: "2025-03-08 16:47:50",
                fecha_eliminacion: Some("2025-03-08 16:47:50")
            },
    ]
──────────────────────────────────────────
📤 Enviando logs al servicio logs_manager...
✅ Programa finalizado correctamente.
mario@mario-pc:~/Escritorio/GitHub/-SO1-_202110509/Proyecto1/rust$ 
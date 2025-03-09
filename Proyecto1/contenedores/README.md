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
LogEntry {
    timestamp: "2025-03-09 13:15:50",
    memory_total: "15236 MB",
    memory_free: "6274 MB",
    memory_used: "8962 MB",
    cpu_usage: "7%",
    cpu: [
        LogContainer {
            id: "106beb997565",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "21%",
            ),
        },
        LogContainer {
            id: "df6b3a6e4f14",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: None,
            metric: Some(
                "20%",
            ),
        },
    ],
    ram: [
        LogContainer {
            id: "1fe21733ebf5",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: None,
            metric: Some(
                "127 MiB",
            ),
        },
    ],
    io: [
        LogContainer {
            id: "b7224e1d14cf",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "1155 ops",
            ),
        },
        LogContainer {
            id: "b2c69352ba02",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "1728 ops",
            ),
        },
        LogContainer {
            id: "ea97f78b67e1",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "1940 ops",
            ),
        },
        LogContainer {
            id: "887a8b41faa7",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: None,
            metric: Some(
                "2127 ops",
            ),
        },
    ],
    disco: [
        LogContainer {
            id: "d34300418f06",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "2245 MiB",
            ),
        },
        LogContainer {
            id: "f79c35cdb6a2",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "3919 MiB",
            ),
        },
        LogContainer {
            id: "1b30a54d58ac",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: None,
            metric: Some(
                "4768 MiB",
            ),
        },
    ],
    eliminados: [
        LogContainer {
            id: "106beb997565",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "21%",
            ),
        },
        LogContainer {
            id: "b7224e1d14cf",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "1155 ops",
            ),
        },
        LogContainer {
            id: "b2c69352ba02",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "1728 ops",
            ),
        },
        LogContainer {
            id: "ea97f78b67e1",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "1940 ops",
            ),
        },
        LogContainer {
            id: "d34300418f06",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "2245 MiB",
            ),
        },
        LogContainer {
            id: "f79c35cdb6a2",
            fecha_creacion: "2025-03-09 13:15:15",
            fecha_eliminacion: Some(
                "2025-03-09 13:15:18",
            ),
            metric: Some(
                "3919 MiB",
            ),
        },
    ],
}
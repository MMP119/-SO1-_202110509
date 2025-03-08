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
  Free RAM: 6871 MB
  Used RAM: 8365 MB
  (Registrado a las: 2025-03-08 16:18:45)
──────────────────────────────────────────

Contenedores por categoría:

  CPU:
    [
        LogContainer 
            {
                id: "62f8c941135c",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "bee02fc039f5",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "76f165b42250",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "517b60b2eaa1",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "7809b1fa75f5",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "92d6fbb6ffc9",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "af4aa865cd5e",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "56247a91290f",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "57135cbbe641",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
        LogContainer 
            {
                id: "997cb44129de",
                fecha_creacion: "2025-03-08 16:18:45",
                fecha_eliminacion: None
            },
    ]

  RAM:
    [
        LogContainer 
            {
                id: "2c24d10057ac",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "e06734a2608d",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "5ea60123eeb5",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "eff034c4fc83",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "8440c8799af1",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "1f6c4c1d95dc",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "a50c43a73a08",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "371de4fd0231",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "baaeab48e54a",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "04d509c04701",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "139fcea49791",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
        LogContainer 
            {
                id: "4668dabc5978",
                fecha_creacion: "2025-03-08 16:18:45",
                fecha_eliminacion: None
            },
    ]

  I/O:
    [
        LogContainer 
            {
                id: "b63cdf6c3a34",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "03f3c56bd199",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "1de644880cb5",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "b97479c740ca",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "a0e6b8b9ec95",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "41742a773662",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "36eb4547d4c3",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "94680b2458c8",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
        LogContainer 
            {
                id: "e6147570328b",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
        LogContainer 
            {
                id: "ed8ece2d82fc",
                fecha_creacion: "2025-03-08 16:18:45",
                fecha_eliminacion: None
            },
    ]

  Disco:
    [
        LogContainer 
            {
                id: "abd0c00ec136",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "a0e1df577c4f",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "5a0477bd5925",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "3f848086d17a",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "766177d9d3a1",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "ebe4ceb66b76",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
        LogContainer 
            {
                id: "72319a2a2d04",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
        LogContainer 
            {
                id: "a756b3bd6445",
                fecha_creacion: "2025-03-08 16:18:45",
                fecha_eliminacion: None
            },
    ]

──────────────────────────────────────────
Contenedores eliminados:
    [
        LogContainer 
            {
                id: "62f8c941135c",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "2c24d10057ac",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "b63cdf6c3a34",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "03f3c56bd199",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "abd0c00ec136",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "a0e1df577c4f",
                fecha_creacion: "2025-03-08 16:15:18",
                fecha_eliminacion: Some("2025-03-08 16:15:18")
            },
        LogContainer 
            {
                id: "bee02fc039f5",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "76f165b42250",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "e06734a2608d",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "1de644880cb5",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "b97479c740ca",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "5a0477bd5925",
                fecha_creacion: "2025-03-08 16:15:42",
                fecha_eliminacion: Some("2025-03-08 16:15:42")
            },
        LogContainer 
            {
                id: "517b60b2eaa1",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "7809b1fa75f5",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "5ea60123eeb5",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "eff034c4fc83",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "8440c8799af1",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "a0e6b8b9ec95",
                fecha_creacion: "2025-03-08 16:16:12",
                fecha_eliminacion: Some("2025-03-08 16:16:12")
            },
        LogContainer 
            {
                id: "92d6fbb6ffc9",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "1f6c4c1d95dc",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "a50c43a73a08",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "371de4fd0231",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "41742a773662",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "3f848086d17a",
                fecha_creacion: "2025-03-08 16:16:52",
                fecha_eliminacion: Some("2025-03-08 16:16:52")
            },
        LogContainer 
            {
                id: "af4aa865cd5e",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "56247a91290f",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "baaeab48e54a",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "04d509c04701",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "36eb4547d4c3",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "766177d9d3a1",
                fecha_creacion: "2025-03-08 16:17:39",
                fecha_eliminacion: Some("2025-03-08 16:17:39")
            },
        LogContainer 
            {
                id: "57135cbbe641",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
        LogContainer 
            {
                id: "139fcea49791",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
        LogContainer 
            {
                id: "94680b2458c8",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
        LogContainer 
            {
                id: "e6147570328b",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
        LogContainer 
            {
                id: "ebe4ceb66b76",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
        LogContainer 
            {
                id: "72319a2a2d04",
                fecha_creacion: "2025-03-08 16:18:06",
                fecha_eliminacion: Some("2025-03-08 16:18:06")
            },
    ]
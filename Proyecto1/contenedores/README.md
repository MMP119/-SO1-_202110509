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
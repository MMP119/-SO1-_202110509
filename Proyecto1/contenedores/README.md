Iniciar el script:
./create_containers.sh


Eliminar todos los contenedores:
docker rm -f $(docker ps -aq) 


Detener solo los contenedores de estrés y no el log:
docker rm -f $(docker ps -aq --filter "name=container_")


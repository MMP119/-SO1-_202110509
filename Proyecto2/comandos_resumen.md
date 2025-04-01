# 🛠️ Resumen de Comandos — Proyecto Sopes Kubernetes

---

## Docker:

| Acción | Comando |
|--------|---------|
| Detener un contenedor | `docker stop <nombre>` |
| Iniciar un contenedor detenido | `docker start <nombre>` |
| Eliminar contenedor | `docker rm <nombre>` |
| Ejecutar con red del host | `docker run --rm --network=host <imagen>` |

---

## Construcción de Imágenes

| Acción | Comando |
|--------|---------|
| Build desde carpeta actual | `docker build -t <nombre_imagen> .` |
| Build desde subcarpeta con Dockerfile específico | `docker build -t <nombre> -f <ruta/Dockerfile> .` |

---

## Contenedores para Testing

| Servicio | Comando |
|----------|---------|
| Redis | `docker run -d --name redis -p 6379:6379 redis` |
| RabbitMQ + UI | `docker run -d --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3-management` |
| Kafka + ZooKeeper (docker-compose) | `docker-compose up -d` |

---

## Pruebas 

| Acción | Comando |
|--------|---------|
| Acceder a Redis CLI | `docker exec -it redis redis-cli` |

Enviar mensaje a RabbitMQ vía HTTP  
```bash
curl -u guest:guest -H "Content-Type: application/json" \
  -X POST -d '{"routing_key":"message","payload":"Hola Rabbit","payload_encoding":"string"}' \
  `http://localhost:15672/api/exchanges/%2f/amq.default/publish`
```

Enviar mensaje a gRPC (Kafka o Rabbit)   
```bash
grpcurl -plaintext -d '{"description": "Texto", "country": "GT", "weather": "Lluvioso"}' \
  localhost:50051 api.Publisher/PublishKafka
``` 

---

## Minikube

| Acción | Comando |
|--------|---------|
| Iniciar clúster | `minikube start` |
| Obtener IP del clúster | `minikube ip` |
| Cargar imagen local en Minikube | `minikube image load <nombre_imagen>` |
| Usar kubectl desde Minikube | `minikube kubectl -- <comando>` |
| Ver pods | `minikube kubectl -- get pods` |
| Ver servicios | `minikube kubectl -- get services` |
| Ver logs de un pod | `minikube kubectl -- logs <nombre_pod>` |

---

## Kubernetes Manifiestos

| Acción | Comando |
|--------|---------|
| Aplicar manifiesto | `minikube kubectl -- apply -f manifiestos/<archivo>.yaml` |
| Eliminar recurso | `minikube kubectl -- delete -f manifiestos/<archivo>.yaml` |

# [SO1]_202110509
 
# Manual de Uso e Instalación


### comandos para docker y el minikube:

docker build -t api_rust:latest .
docker build -f http/Dockerfile -t api_go_http:latest .
docker build -f grpc/Dockerfile -t api_go_grpc:latest .
docker build -f kafka_consumer/Dockerfile -t kafka_consumer:latest .
docker build -f rabbitmq_consumer/Dockerfile -t rabbitmq_consumer:latest .

docker tag api_rust:latest 34.69.137.65.nip.io/proyecto/api_rust:latest
docker push 34.69.137.65.nip.io/proyecto/api_rust:latest


minikube start
minikube addons enable ingress
minikube stop
minikube delete

minikube image load api_rust:latest 
minikube image load api_go_http:latest 
minikube image load api_go_grpc:latest 
minikube image load kafka_consumer:latest 
minikube image load rabbitmq_consumer:latest 

minikube kubectl -- apply -f api-rust.yaml
minikube kubectl -- apply -f api-http.yaml
minikube kubectl -- apply -f grpc.yaml
minikube kubectl -- apply -f kafka-consumer.yaml
minikube kubectl -- apply -f rabbitmq-consumer.yaml
minikube kubectl -- apply -f redis.yaml
minikube kubectl -- apply -f kafka.yaml
minikube kubectl -- apply -f rabbitmq.yaml
minikube kubectl -- apply -f grafana.yaml
minikube kubectl -- apply -f grafana-ingress.yaml
minikube kubectl -- apply -f ingress.yaml
minikube kubectl -- apply -f valkey.yaml


minikube kubectl -- get pods
minikube kubectl -- rollout restart deployment rabbitmq-consumer
minikube kubectl -- logs -l app=rabbitmq-consumer
minikube ip


docker rmi api_rust:latest
docker rmi rabbitmq_consumer:latest
docker rmi kafka_consumer:latest
docker rmi api_go_http:latest
docker rmi api_go_grpc:latest

docker image prune

Grafana:
<br>
redis://redis:6379
redis://valkey:6379
LRANGE mensajes 0 -1 //mostrar todos los mensajes en redis



Poner a funcionar locust:
<br>
locust -H http://192.168.49.2.nip.io


<br>
ENCENDER HARBOR
<br>
sudo docker-compose up -d
<br>
mario1234, Harbor12345

<br>
Kubectl:
<br>
kubectl rollout restart deployment rabbitmq-consumer
kubectl apply -f . -n ingress-nginx
kubectl apply -f grpc.yaml -n ingress-nginx



<br>
COMANDOS DESDE CERO:
<br>
helm install ingress-nginx ingress-nginx/ingress-nginx -n ingress-nginx --create-namespace

<br>
Verificar instalacion:
<br>
kubectl get pods -n ingress-nginx
<br>
kubectl get svc -n ingress-nginx
<br>
Poner la external-ip para el ingress.yaml con el .nip.io

<br>
kubectl create -f https://strimzi.io/install/latest?namespace=ingress-nginx

<br>
PARA BORROR STRIMZI: kubectl delete -f https://strimzi.io/install/latest?namespace=ingress-nginx

<br>
kubectl apply -f . -n ingress-nginx (dentro de mi carpeta de manifiestos-produccion, está todo en el namespace de ingress-nginx)
<br>
kubectl get pods -n ingress-nginx
<br>
kubectl get services -n ingress-nginx

<br>
grafana:
<br>
http://130.211.222.62/grafana (siempre cambiar la ip dependiendo del ingress)
<br>

Ver los logs:
<br>
kubectl logs -n ingress-nginx -l app=api-rust
<br>

Reiniciar deployment:
<br>
kubectl rollout restart deployment grafana -n ingress-nginx
<br>

Borrar un pod:
<br>
kubectl delete pod -l app=grafana -n ingress-nginx
<br>


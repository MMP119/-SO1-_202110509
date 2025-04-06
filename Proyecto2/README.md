# [SO1]_202110509
 
# Manual de Uso e Instalación


### comandos para docker y el minikube:

docker build -t api_rust:latest .
docker build -f http/Dockerfile -t api_go_http:latest .
docker build -f grpc/Dockerfile -t api_go_grpc:latest .
docker build -f kafka_consumer/Dockerfile -t kafka_consumer:latest .
docker build -f rabbitmq_consumer/Dockerfile -t rabbitmq_consumer:latest .

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


minikube kubectl -- get pods
minikube kubectl -- rollout restart deployment rabbitmq-consumer
minikube kubectl -- logs -l app=rabbitmq-consumer
minikube ip


docker rmi api_rust:lastest
docker rmi rabbitmq_consumer:lastest
docker rmi kafka_consumer:lastest
docker rmi api_go_http:lastest
docker rmi api_go_grpc:lastest

docker image prune



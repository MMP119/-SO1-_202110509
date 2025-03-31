package main

import (
	"context"
	"log"
	"net"
	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection" 
	"api_go/proto" // este es el paquete generado a partir del .proto
)

// server implementa el servicio Publisher.
type server struct {
	api.UnimplementedPublisherServer
}

func (s *server) PublishRabbit(ctx context.Context, in *api.WeatherInput) (*api.PublishResponse, error) {
	log.Printf("Recibido PublishRabbit: %+v", in)
	return &api.PublishResponse{Success: true, Message: "Publicado en RabbitMQ"}, nil
}

func (s *server) PublishKafka(ctx context.Context, in *api.WeatherInput) (*api.PublishResponse, error) {
	log.Printf("Recibido PublishKafka: %+v", in)
	return &api.PublishResponse{Success: true, Message: "Publicado en Kafka"}, nil
}

func main() {
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("Error al escuchar: %v", err)
	}
	grpcServer := grpc.NewServer()
	api.RegisterPublisherServer(grpcServer, &server{})

	// habilita la reflexión para que herramientas como grpcurl puedan descubrir los servicios.
	reflection.Register(grpcServer)

	log.Println("Servidor gRPC escuchando en el puerto 50051...")
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("Error al servir: %v", err)
	}
}

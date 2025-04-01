package main

import (
	"context"
	"log"
	"net"
	"time"
	"github.com/segmentio/kafka-go"
	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection" 
	"api_go/proto" // este es el paquete generado a partir del .proto
)

// server implementa el servicio Publisher
type server struct {
	api.UnimplementedPublisherServer
}

func (s *server) PublishRabbit(ctx context.Context, in *api.WeatherInput) (*api.PublishResponse, error) {
	log.Printf("Recibido PublishRabbit: %+v", in)
	return &api.PublishResponse{Success: true, Message: "Publicado en RabbitMQ"}, nil
}


// función para publicar un mensaje en Kafka
func publishToKafka(message string) error {
	writer := kafka.NewWriter(kafka.WriterConfig{
		Brokers:  []string{"localhost:9092"}, 
		Topic:    "message",                  
		Balancer: &kafka.LeastBytes{},
	})
	defer writer.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	err := writer.WriteMessages(ctx, kafka.Message{
		Value: []byte(message),
	})
	return err
}



// PublishKafka ahora integra la publicación real en Kafka.
func (s *server) PublishKafka(ctx context.Context, in *api.WeatherInput) (*api.PublishResponse, error) {
	log.Printf("Recibido PublishKafka: %+v", in)
	message := "Clima: " + in.Description + " - " + in.Country + " - " + in.Weather
	err := publishToKafka(message)
	if err != nil {
		log.Printf("Error publicando en Kafka: %v", err)
		return &api.PublishResponse{Success: false, Message: "Error al publicar en Kafka"}, err
	}
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

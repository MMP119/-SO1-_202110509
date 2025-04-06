package main

import (
	"context"
	"log"
	"net"
	"time"
	"os"
	"github.com/rabbitmq/amqp091-go"
	"github.com/segmentio/kafka-go"
	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection" 
	"api_go/proto" // este es el paquete generado a partir del .proto
)

// server implementa el servicio Publisher
type server struct {
	api.UnimplementedPublisherServer
}


func publishToRabbit(message string) error {
	rabbitAddr := os.Getenv("RABBITMQ_ADDR")
	if rabbitAddr == "" {
		rabbitAddr = "amqp://guest:guest@rabbitmq:5672/"
	}

	conn, err := amqp091.Dial(rabbitAddr)
	if err != nil {
		return err
	}
	defer conn.Close()

	ch, err := conn.Channel()
	if err != nil {
		return err
	}
	defer ch.Close()

	q, err := ch.QueueDeclare(
		"message",
		false,
		false,
		false,
		false,
		nil,
	)
	if err != nil {
		return err
	}

	return ch.Publish(
		"",
		q.Name,
		false,
		false,
		amqp091.Publishing{
			ContentType: "text/plain",
			Body:        []byte(message),
		},
	)
}


func (s *server) PublishRabbit(ctx context.Context, in *api.WeatherInput) (*api.PublishResponse, error) {
	log.Printf("Recibido PublishRabbit: %+v", in)
	message := "Clima: " + in.Description + " - " + in.Country + " - " + in.Weather

	err := publishToRabbit(message)
	if err != nil {
		log.Printf("Error publicando en RabbitMQ: %v", err)
		return &api.PublishResponse{Success: false, Message: "Error al publicar en RabbitMQ"}, err
	}

	return &api.PublishResponse{Success: true, Message: "Publicado en RabbitMQ"}, nil
}



// función para publicar un mensaje en Kafka
func publishToKafka(message string) error {
	kafkaAddr := os.Getenv("KAFKA_ADDR")
	if kafkaAddr == "" {
		kafkaAddr = "kafka:9092"
	}

	writer := kafka.NewWriter(kafka.WriterConfig{
		Brokers:  []string{kafkaAddr},
		Topic:    "message",
		Balancer: &kafka.LeastBytes{},
	})
	defer writer.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	return writer.WriteMessages(ctx, kafka.Message{
		Value: []byte(message),
	})
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

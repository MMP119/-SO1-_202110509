package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/segmentio/kafka-go"
	"github.com/redis/go-redis/v9"
)

var ctx = context.Background()

func main() {
	// Obtener dirección de Redis
	redisAddr := os.Getenv("REDIS_ADDR")
	if redisAddr == "" {
		redisAddr = "redis:6379"
	}

	// Conexión a Redis
	rdb := redis.NewClient(&redis.Options{
		Addr: redisAddr,
	})
	defer rdb.Close()

	// Obtener dirección de Kafka
	kafkaAddr := os.Getenv("KAFKA_ADDR")
	if kafkaAddr == "" {
		kafkaAddr = "kafka:9092"
	}

	// Conectar a Kafka
	reader := kafka.NewReader(kafka.ReaderConfig{
		Brokers: []string{kafkaAddr},
		Topic:   "message",
		GroupID: "kafka-consumer-group",
	})
	defer reader.Close()

	fmt.Println("Consumidor de Kafka iniciado...")

	for {
		m, err := reader.ReadMessage(ctx)
		if err != nil {
			log.Printf("Error leyendo mensaje: %v", err)
			continue
		}

		text := string(m.Value)

		err = rdb.LPush(ctx, "mensajes", text).Err()
		if err != nil {
			log.Printf("Error guardando en Redis: %v", err)
		} else {
			log.Printf("Mensaje guardado en Redis (lista): %s", text)
		}

		time.Sleep(1 * time.Second)
	}
}

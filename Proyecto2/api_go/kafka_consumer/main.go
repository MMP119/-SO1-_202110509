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
	// Obtener dirección de Redis desde variable de entorno
	redisAddr := os.Getenv("REDIS_ADDR")
	if redisAddr == "" {
		redisAddr = "localhost:6379"
	}

	// Conectar a Redis
	rdb := redis.NewClient(&redis.Options{
		Addr: redisAddr,
	})
	defer rdb.Close()

	// Obtener dirección de Kafka desde variable de entorno
	kafkaAddr := os.Getenv("KAFKA_ADDR")
	if kafkaAddr == "" {
		kafkaAddr = "localhost:9092"
	}

	// Conectar a Kafka
	reader := kafka.NewReader(kafka.ReaderConfig{
		Brokers: []string{kafkaAddr},
		Topic:   "message",
		GroupID: "kafka-consumer-group",
	})
	defer reader.Close()

	fmt.Println("Consumidor de Kafka iniciado...")

	counter := 1

	for {
		m, err := reader.ReadMessage(ctx)
		if err != nil {
			log.Printf("Error leyendo mensaje: %v", err)
			continue
		}

		text := string(m.Value)
		key := fmt.Sprintf("mensaje:%d", counter)

		err = rdb.Set(ctx, key, text, 0).Err()
		if err != nil {
			log.Printf("Error guardando en Redis: %v", err)
		} else {
			log.Printf("Mensaje guardado en Redis [%s]: %s", key, text)
			counter++
		}
		time.Sleep(1 * time.Second)
	}
}

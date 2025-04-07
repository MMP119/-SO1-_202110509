package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/segmentio/kafka-go"
	"github.com/redis/go-redis/v9"
)

var ctx = context.Background()

func main() {
	// Conectar a Redis
	rdb := redis.NewClient(&redis.Options{
		Addr: "redis:6379",
	})
	defer rdb.Close()

	// Conectar a Kafka
	reader := kafka.NewReader(kafka.ReaderConfig{
		Brokers: []string{"kafka:9092"},
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

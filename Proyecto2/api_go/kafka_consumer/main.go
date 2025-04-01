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
		Addr: "localhost:6379", // puerto por defecto
	})
	defer rdb.Close()

	// Conectar a Kafka
	reader := kafka.NewReader(kafka.ReaderConfig{
		Brokers: []string{"localhost:9092"},
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

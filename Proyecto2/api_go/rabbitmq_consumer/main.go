package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"strconv"
	"time"

	"github.com/rabbitmq/amqp091-go"
	"github.com/redis/go-redis/v9"
)

var ctx = context.Background()

func main() {
	// Obtener dirección de Redis
	redisAddr := os.Getenv("REDIS_ADDR")
	if redisAddr == "" {
		redisAddr = "localhost:6379"
	}

	// Conexión a Redis
	rdb := redis.NewClient(&redis.Options{
		Addr: redisAddr,
	})
	defer rdb.Close()

	// Obtener dirección de RabbitMQ
	rabbitAddr := os.Getenv("RABBITMQ_ADDR")
	if rabbitAddr == "" {
		rabbitAddr = "amqp://guest:guest@localhost:5672/"
	}
	log.Printf("Dirección de RabbitMQ: %s", rabbitAddr)

	// Conexión a RabbitMQ
	conn, err := amqp091.Dial(rabbitAddr)
	if err != nil {
		log.Fatalf("Error conectando a RabbitMQ: %v", err)
	}
	defer conn.Close()

	ch, err := conn.Channel()
	if err != nil {
		log.Fatalf("Error obteniendo canal: %v", err)
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
		log.Fatalf("Error declarando cola: %v", err)
	}

	msgs, err := ch.Consume(
		q.Name,
		"",
		true,
		false,
		false,
		false,
		nil,
	)
	if err != nil {
		log.Fatalf("Error al consumir: %v", err)
	}

	fmt.Println("RabbitMQ consumer corriendo...")

	counter := 1
	for msg := range msgs {
		text := string(msg.Body)
		key := "mensaje:" + strconv.Itoa(counter)

		err := rdb.Set(ctx, key, text, 0).Err()
		if err != nil {
			log.Printf("Error guardando en Redis: %v", err)
		} else {
			log.Printf("Mensaje guardado en Redis [%s]: %s", key, text)
			counter++
		}

		time.Sleep(1 * time.Second)
	}
}

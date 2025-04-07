package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/rabbitmq/amqp091-go"
	"github.com/redis/go-redis/v9"
)

var ctx = context.Background()

func main() {
	// Obtener dirección de Redis
	valkeyAddr := os.Getenv("VALKEY_ADDR")
	if valkeyAddr == "" {
		valkeyAddr = "valkey:6379"
	}

	// Conexión a Redis
	rdb := redis.NewClient(&redis.Options{
		Addr: valkeyAddr,
	})
	defer rdb.Close()

	// Obtener dirección de RabbitMQ
	rabbitAddr := os.Getenv("RABBITMQ_ADDR")
	if rabbitAddr == "" {
		rabbitAddr = "amqp://guest:guest@rabbitmq:5672/"
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

	q, err := ch.QueueDeclare("message", false, false, false, false, nil)
	if err != nil {
		log.Fatalf("Error declarando cola: %v", err)
	}

	msgs, err := ch.Consume(q.Name, "", true, false, false, false, nil)
	if err != nil {
		log.Fatalf("Error al consumir: %v", err)
	}

	fmt.Println("RabbitMQ consumer corriendo...")

	for msg := range msgs {
		text := string(msg.Body)

		err := rdb.LPush(ctx, "mensajes", text).Err()
		if err != nil {
			log.Printf("Error guardando en Redis: %v", err)
		} else {
			log.Printf("Mensaje guardado en Redis (lista): %s", text)
		}

		time.Sleep(1 * time.Second)
	}
}

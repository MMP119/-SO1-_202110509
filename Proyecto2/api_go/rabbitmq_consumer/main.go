package main

import (
	"context"
	"fmt"
	"log"
	"strconv"
	"time"

	"github.com/rabbitmq/amqp091-go"
	"github.com/redis/go-redis/v9"
)

var ctx = context.Background()

func main() {
	// Conexión a Redis
	rdb := redis.NewClient(&redis.Options{
		Addr: "localhost:6379",
	})
	defer rdb.Close()

	// Conexión a RabbitMQ
	conn, err := amqp091.Dial("amqp://guest:guest@localhost:5672/")
	if err != nil {
		log.Fatalf("Error conectando a RabbitMQ: %v", err)
	}
	defer conn.Close()

	ch, err := conn.Channel()
	if err != nil {
		log.Fatalf("Error obteniendo canal: %v", err)
	}
	defer ch.Close()

	// Asegurarse de que la cola exista
	q, err := ch.QueueDeclare(
		"message", // nombre de la cola
		false,     // durable
		false,     // auto-delete
		false,     // exclusive
		false,     // no-wait
		nil,       // argumentos
	)
	if err != nil {
		log.Fatalf("Error declarando cola: %v", err)
	}

	msgs, err := ch.Consume(
		q.Name,
		"",
		true,  // auto-ack
		false, // exclusive
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

package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"

	"google.golang.org/grpc"
	"api_go/proto" // Importa el paquete generado a partir del .proto
)

// weatherInput define la estructura que esperamos recibir en el JSON.
type WeatherInput struct {
	Description string `json:"description"`
	Country     string `json:"country"`
	Weather     string `json:"weather"`
}

func handler(w http.ResponseWriter, r *http.Request) {
	var input WeatherInput
	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, "Entrada inválida", http.StatusBadRequest)
		return
	}
	fmt.Printf("Recibido: %+v\n", input)

	// conectar al servidor gRPC
	conn, err := grpc.Dial("grpc-service:50051", grpc.WithInsecure(), grpc.WithBlock())
	if err != nil {
		http.Error(w, "No se pudo conectar al servidor gRPC", http.StatusInternalServerError)
		return
	}
	defer conn.Close()

	client := api.NewPublisherClient(conn)
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	// enviar a RabbitMQ
	respRabbit, errRabbit := client.PublishRabbit(ctx, &api.WeatherInput{
		Description: input.Description,
		Country:     input.Country,
		Weather:     input.Weather,
	})

	// enviar a Kafka
	respKafka, errKafka := client.PublishKafka(ctx, &api.WeatherInput{
		Description: input.Description,
		Country:     input.Country,
		Weather:     input.Weather,
	})

	// Construir respuesta
	response := map[string]interface{}{
		"input": input,
	}

	if errRabbit != nil {
		response["rabbit_error"] = errRabbit.Error()
	} else {
		response["rabbit_response"] = respRabbit
	}

	if errKafka != nil {
		response["kafka_error"] = errKafka.Error()
	} else {
		response["kafka_response"] = respKafka
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)
}


func main() {
	http.HandleFunc("/input", handler)
	fmt.Println("Servidor HTTP corriendo en el puerto 8081...")
	log.Fatal(http.ListenAndServe(":8081", nil))
}

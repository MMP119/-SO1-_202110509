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

	// llamar al método PublishRabbit con los datos recibidos
	grpcResp, err := client.PublishRabbit(ctx, &api.WeatherInput{
		Description: input.Description,
		Country:     input.Country,
		Weather:     input.Weather,
	})
	if err != nil {
		http.Error(w, "Error en llamada gRPC", http.StatusInternalServerError)
		return
	}

	// construir una respuesta que incluya la respuesta del servidor gRPC
	response := map[string]interface{}{
		"input":         input,
		"grpc_response": grpcResp,
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)
}

func main() {
	http.HandleFunc("/input", handler)
	fmt.Println("Servidor HTTP corriendo en el puerto 8081...")
	log.Fatal(http.ListenAndServe(":8081", nil))
}

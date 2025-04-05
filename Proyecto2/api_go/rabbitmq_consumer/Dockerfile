
FROM golang:1.24.1 AS builder
WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

# copiar el código fuente del consumidor
COPY rabbitmq_consumer/ ./rabbitmq_consumer/

# compilar el binario
WORKDIR /app/rabbitmq_consumer
RUN go build -o rabbit_consumer main.go

FROM debian:bookworm-slim
COPY --from=builder /app/rabbitmq_consumer/rabbit_consumer /usr/local/bin/rabbit_consumer

CMD ["/usr/local/bin/rabbit_consumer"]

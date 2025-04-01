
FROM golang:1.24.1 AS builder
WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY kafka_consumer/ ./kafka_consumer/

WORKDIR /app/kafka_consumer
RUN go build -o kafka_consumer main.go

FROM debian:bookworm-slim

COPY --from=builder /app/kafka_consumer/kafka_consumer /usr/local/bin/kafka_consumer

CMD ["/usr/local/bin/kafka_consumer"]

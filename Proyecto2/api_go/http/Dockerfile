FROM golang:1.24.1 as builder
WORKDIR /app

# copia los archivos de go.mod y go.sum y descarga las dependencias
COPY go.mod go.sum ./
RUN go mod download

# copia el código del servicio HTTP y el directorio proto si es necesario
COPY http/ ./http/
COPY proto/ ./proto/


WORKDIR /app/http

# compila solo main.go para obtener el binario "http_service"
RUN go build -o http main.go

# Imagen final
FROM debian:bookworm-slim
COPY --from=builder /app/http/http /usr/local/bin/http
EXPOSE 8081
CMD ["/usr/local/bin/http"]

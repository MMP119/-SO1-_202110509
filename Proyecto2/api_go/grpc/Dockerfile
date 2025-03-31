FROM golang:1.24.1 as builder
WORKDIR /app

# copiamos el archivo go.mod y go.sum del módulo completo 
COPY go.mod go.sum ./
RUN go mod download

# copiamos el código del servidor gRPC y el directorio proto (necesario para las dependencias)
COPY grpc/ ./grpc/
COPY proto/ ./proto/

# nos movemos al directorio del servidor gRPC
WORKDIR /app/grpc

# compilamos el binario
RUN go build -o grpc_server grpc_server.go


FROM debian:bookworm-slim
COPY --from=builder /app/grpc/grpc_server /usr/local/bin/grpc_server
EXPOSE 50051
CMD ["/usr/local/bin/grpc_server"]

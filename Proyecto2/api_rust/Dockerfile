# Etapa 1: Construir el binario con Rust
FROM rust:latest as builder
WORKDIR /app
# Copia todos los archivos del proyecto al contenedor
COPY . .
# Compila el proyecto en modo release
RUN cargo build --release

# Etapa 2: Crear una imagen ligera para ejecutar el binario
FROM debian:bookworm-slim
# Copia el binario compilado desde la etapa builder
COPY --from=builder /app/target/release/api_rust /usr/local/bin/api_rust
# Expone el puerto 8080 (el que usa la API)
EXPOSE 8080
# Comando para ejecutar el binario
CMD ["api_rust"]

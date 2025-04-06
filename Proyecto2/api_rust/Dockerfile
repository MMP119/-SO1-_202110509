# Etapa 1: Compilar usando musl para generar binario estático
FROM clux/muslrust:stable AS builder

# Crear carpeta de trabajo
WORKDIR /app

# Copiar todos los archivos del proyecto
COPY . .

# Compilar en release usando musl como target
RUN cargo build --release

# Etapa 2: Crear imagen final ultra liviana
FROM debian:bookworm-slim

# Copiar el binario compilado 
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api_rust /usr/local/bin/api_rust

# Ejecutar el binario directamente
CMD ["/usr/local/bin/api_rust"]

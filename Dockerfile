# Etapa de construcción
FROM rust:alpine3.21 AS builder

# Establece el directorio de trabajo
WORKDIR /usr/src/app

# Copia los archivos del proyecto
COPY . .

# Instala dependencias necesarias para compilar
RUN apk add --no-cache musl-dev

# Ejecuta los tests sin la opción --locked
RUN cargo test --release --verbose

# Compila el proyecto en modo release
RUN cargo build --release --verbose

# Etapa final
FROM alpine:latest

# Instala dependencias necesarias en la imagen ligera
RUN apk add --no-cache libssl3

# Copia el binario desde la etapa de construcción
COPY --from=builder /usr/src/app/target/release/test_rust_docker /usr/local/bin/test_rust_docker

# Define el comando de inicio
CMD ["test_rust_docker"]

# Usa una imagen base de Rust para la plataforma amd64
FROM --platform=$BUILDPLATFORM rust:alpine3.21 AS builder

# Establece el directorio de trabajo
WORKDIR /usr/src/app

# Copia los archivos del proyecto
COPY . .

# Compila el proyecto en modo release
RUN cargo build --release

# Usa una imagen ligera para la ejecución (sin Rust ni Cargo)
FROM alpine:latest

# Instala dependencias necesarias (si las necesitas, como OpenSSL, por ejemplo)
RUN apk add --no-cache libssl3

# Copia el binario compilado desde la etapa anterior
COPY --from=builder /usr/src/app/target/release/test_rust_docker /usr/local/bin/test_rust_docker

# Establece el binario como el comando de inicio
CMD ["test_rust_docker"]

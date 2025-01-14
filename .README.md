# Calculadora Simple en Docker

Este proyecto es una calculadora simple escrita en Rust y contenerizada con Docker para facilitar su ejecución.

## Requisitos Previos

- Docker instalado en tu sistema.
- (Opcional) Docker Compose para un manejo más avanzado de contenedores.

## Instrucciones para Construir y Ejecutar la Calculadora

### Construcción de la Imagen Docker

Para construir la imagen de Docker, utiliza el siguiente comando:

```bash
docker build -t calculadora .
```

Este comando creará una imagen de Docker llamada `calculadora` basada en el código fuente del proyecto.

### Ejecución del Contenedor

Para ejecutar el contenedor, usa el siguiente comando:

```bash
docker run --rm -it calculadora
```

- La opción `--rm` asegura que el contenedor se elimine automáticamente después de su ejecución.
- La opción `-it` permite interactuar con el contenedor en la terminal.

### Estructura del Dockerfile

El archivo `Dockerfile` se encarga de:

1. Usar una imagen base de Rust para compilar el proyecto.
2. Copiar los archivos del proyecto.
3. Construir el binario de Rust en modo release.
4. Usar una imagen ligera de Alpine para ejecutar el binario resultante.

```dockerfile
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
CMD ["/usr/local/bin/test_rust_docker"]
```

### Solución de Problemas

1. **Error de compilación:** Verifica que el nombre del binario en el Dockerfile coincida con el definido en `Cargo.toml`.
2. **Faltan dependencias en Alpine:** Asegúrate de instalar las dependencias necesarias en la etapa de ejecución.
3. **Permisos:** Si encuentras problemas de permisos, asegúrate de que el binario tenga permisos de ejecución: `chmod +x /usr/local/bin/test_rust_docker`.

## Autor

Este proyecto fue desarrollado por David DVP y con licencia [MIT](https://choosealicense.com/licenses/mit/). Siente libre de modificar y adaptar el proyecto según tus necesidades.

---

¡Gracias por usar la calculadora simple en Docker! 🚀

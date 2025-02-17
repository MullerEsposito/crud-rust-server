# Imagem base com Rust# Etapa 1: Construir o binário
FROM rust:1.79.0 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

# Etapa 2: Criar a imagem final
FROM debian:buster-slim

WORKDIR /usr/src/app

# Copiar o binário da etapa anterior
COPY --from=builder /usr/src/app/target/release/server .

# Expor a porta na qual o servidor está rodando
EXPOSE 3000

# Comando para iniciar a aplicação
CMD ["./server"]

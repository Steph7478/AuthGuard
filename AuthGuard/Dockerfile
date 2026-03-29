# ========================
# 🦀 BUILD STAGE
# ========================
FROM rust:1.75 as builder

WORKDIR /app

# cache de dependências
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# build real
COPY src ./src
RUN cargo build --release

# ========================
# 🚀 RUNTIME STAGE
# ========================
FROM debian:bookworm-slim

WORKDIR /app

# dependências mínimas
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# binário
COPY --from=builder /app/target/release/authguard /usr/local/bin/authguard

# config
COPY config.json .

# porta
EXPOSE 3000

# execução
CMD ["authguard"]
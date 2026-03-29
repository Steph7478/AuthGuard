# ========================
# 🦀 BUILD + RUNTIME
# ========================
FROM rustlang/rust:nightly-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

EXPOSE 3000

CMD ["./target/release/authguard"]
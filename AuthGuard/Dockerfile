# Stage 1: Build
FROM rustlang/rust:nightly-slim AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Build real
COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM gcr.io/distroless/cc-debian12
WORKDIR /app

COPY --from=builder /app/target/release/authguard .
COPY config.json . 

EXPOSE 3000
CMD ["./authguard"]
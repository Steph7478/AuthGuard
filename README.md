# AuthGuard

A high-performance authentication and authorization gateway built in Rust. AuthGuard acts as a reverse proxy that validates JWT tokens from Keycloak and enforces access control based on user groups.

## Features

- 🔐 JWT Validation: Validates tokens from Keycloak using JWKS
- 🛡️ Group-Based Authorization: Restrict access based on Keycloak groups
- ⚡ High Performance: Built with Rust and Axum
- 📊 Rate Limiting: Built-in rate limiting per IP address
- 📈 Prometheus Metrics: Exposes metrics for monitoring
- 🔄 Reverse Proxy: Forwards authenticated requests to upstream services
- 🐳 Docker Support: Ready-to-use Docker configuration

## Architecture

          [ Client ]
              │
              ▼
      ┌───────────────┐
      │   AuthGuard   │
      │ (Rust Proxy)  │
      └───────┬───────┘
              │
      ┌───────┴───────┐
      │               │
      ▼               ▼
[ Keycloak ]     [  Redis   ]
 (Identity)      (Rate Limit)
      │               │
      └───────┬───────┘
              │
              ▼
      [ Target Service ]
          (Backend)

## Project Structure

AuthGuard/
├── src/                 # Main Rust source (Proxy, Auth, Middleware)
├── keycloak/            # realm.json configuration
├── docker-compose.yml   # Docker services setup
├── config.json          # Policy configuration
├── Dockerfile           # Rust build configuration
└── test.sh              # Testing script

## Prerequisites

- Rust 1.70+
- Docker & Docker Compose

## Quick Start

### 1. Setup
git clone https://github.com/yourusername/authguard.git
cd authguard

### 2. Configure
Edit config.json:
{
  "require_auth": true,
  "block_ips": []
}

### 3. Run
docker-compose up -d
./test.sh

The test script handles: Client type selection, Auth flows, and Token acquisition.

## Configuration

| Variable | Description | Default |
| :--- | :--- | :--- |
| PORT | AuthGuard port | 3000 |
| REDIS_URL | Redis connection | redis://redis:6379 |
| JWKS_URL | Keycloak JWKS | http://keycloak:8080/realms/... |
| TARGET_SERVICE | Upstream URL | http://target_service:4000 |
| RATE_LIMIT | Requests/Window | 100 |

## API Endpoints

| Endpoint | Method | Description |
| :--- | :--- | :--- |
| /admin | ANY | Protected: Requires /TI/Infraestrutura group |
| /* | ANY | Proxies to target service after auth |
| /metrics | GET | Prometheus metrics endpoint |
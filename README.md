# AuthGuard

A high-performance authentication and authorization gateway built in Rust. AuthGuard acts as a reverse proxy that validates JWT tokens from Keycloak and enforces access control based on user groups.

## 🚀 Features

* 🔐 **JWT Validation**: Validates tokens from Keycloak using JWKS.
* 🛡️ **Group-Based Auth**: Restricts access based on Keycloak groups.
* ⚡ **High Performance**: Built with Rust, Axum, and Hyper.
* 📊 **Rate Limiting**: Built-in protection per IP address via Redis.
* 📈 **Observability**: Prometheus metrics endpoint for monitoring.
* 🔄 **Reverse Proxy**: Seamlessly forwards requests to upstream services.

## 🛠️ Getting Started

markdown
### 1. Installation
```bash
git clone git@github.com:Steph7478/AuthGuard.git
cd authguard
```

2. Execution
```bash
docker-compose up -d
```

3. Test
```bash
./test.sh
```

## ⚙️ Configuration Reference

| Variable         | Description                | Default Value                |
|:-----------------|:---------------------------|:-----------------------------|
| PORT             | Gateway listening port     | 3000                         |
| REDIS_URL        | Redis connection string    | redis://redis:6379           |
| JWKS_URL         | Keycloak JWKS endpoint     | http://keycloak:8080/...     |
| TARGET_SERVICE   | Upstream service URL       | http://target_service:4000   |
| RATE_LIMIT       | Max requests per window    | 100                          |

## 🛣️ API Endpoints

| Endpoint    | Method | Security Policy                               |
|:------------|:-------|:----------------------------------------------|
| /admin      | ANY    | Requires /TI/Infraestrutura group             |
| /metrics    | GET    | Public (Prometheus Format)                    |
| /* | ANY    | Valid JWT required -> Proxy to Target Service |

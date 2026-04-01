# AuthGuard

A high-performance authentication and authorization gateway built in Rust. AuthGuard acts as a reverse proxy that validates JWT tokens from Keycloak and enforces access control based on user groups, now integrated with Nginx for production-ready routing.

## 🚀 Features

* 🔐 **JWT Validation**: Validates tokens from Keycloak using JWKS via Axum/Hyper.
* 🛡️ **Group-Based Auth**: Restricts access based on Keycloak hierarchy (e.g., `/TI/Infraestrutura`).
* ⚡ **High Performance**: Built with Rust, Axum, and Redis for low-latency checks.
* 📊 **Rate Limiting**: Built-in protection per IP address via Redis.
* 📈 **Observability**: Prometheus metrics endpoint for monitoring.
* 🔄 **Google IDP**: Out-of-the-box support for Google Login with `kc_idp_hint`.

<br/>


## 🛠️ Getting Started

### 1. Installation
```bash
git clone git@github.com:Steph7478/AuthGuard.git
cd authguard
```

### 2. Execution
```bash
docker-compose --profile (dev/prod/local) up -d
```

### 3. Test
```bash
./test_auth.sh
```

### 4. Login Flow

AuthGuard provides clean, user-friendly login endpoints:

| Endpoint | Description |
|:---|:---|
| `/login` | Standard Keycloak login |
| `/login/google` | Direct login with Google (skips Keycloak selection screen) |

> **Note:** Ensure `http://localhost/callback` is added to **Valid Redirect URIs** in your Keycloak Client settings.

<br/>

## 🛣️ API Endpoints

| Endpoint | Method | Security Policy |
|:---|:---|:---|
| `/auth/*` | ANY | Handled by Nginx -> Proxy to Keycloak |
| `/admin` | ANY | Requires `/TI/Infraestrutura` group |
| `/metrics` | GET | Public (Prometheus Format) |
| `/*` | ANY | Valid JWT required -> Proxy to Target Service |

<br/>

## 🏗️ Architecture Overview

AuthGuard sits behind an **Nginx** instance. When a request hits `http://localhost`:
1. **Nginx** routes `/auth/*` directly to Keycloak.
2. **Nginx** routes all other calls to **AuthGuard (Rust)**.
3. **AuthGuard** validates the `Authorization` header using Keycloak's JWKS.
4. If the token is valid and the group is authorized (extracted from the `groups` claim), the request proceeds to the **Target Service**.

### Nginx Configuration Highlights

| Location | Purpose |
|:---|:---|
| `/login` | Redirects to Keycloak login page |
| `/login/google` | Redirects to Keycloak with Google IDP hint |
| `/callback` | OAuth2 callback handled by AuthGuard |
| `/validate` | Token validation endpoint |
| `/metrics` | Prometheus metrics endpoint |
| `/health` | Health check endpoint |
| `/auth/*` | Proxies directly to Keycloak |
| `/_validate` | Internal auth request endpoint with rate limiting |
| `/api/*` | Protected API endpoints with auth validation and rate limiting |

### Rate Limiting

- **API endpoints**: 100 requests/second with burst of 20
- **Validate endpoint**: 30 requests/second with burst of 10

All rate limits are applied per IP address using Nginx's `limit_req` module.
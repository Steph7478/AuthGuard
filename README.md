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
``bash
git clone git@github.com:Steph7478/AuthGuard.git
cd authguard
``

### 2. Execution
``bash
docker-compose up -d
``

### 3. Test
``bash
./test.sh
``

### 4. Login Flow (Standard OIDC)
To authenticate via Google and skip the Keycloak selection screen, redirect your frontend to:
http://localhost/auth/realms/authguard/protocol/openid-connect/auth?client_id=authguard-service&response_type=code&scope=openid%20profile%20email&redirect_uri=http://localhost/callback&kc_idp_hint=google

> **Note:** Ensure `http://localhost/callback` is added to **Valid Redirect URIs** in your Keycloak Client settings.

<br/>

## 🛣️ API Endpoints

| Endpoint | Method | Security Policy |
|:---|:---|:---|
| /auth/* | ANY | Handled by Nginx -> Proxy to Keycloak |
| /admin | ANY | Requires /TI/Infraestrutura group |
| /metrics | GET | Public (Prometheus Format) |
| /* | ANY | Valid JWT required -> Proxy to Target Service |

<br/>

## 🏗️ Architecture Overview

AuthGuard sits behind an **Nginx** instance. When a request hits `http://localhost`:
1. **Nginx** routes `/auth` directly to Keycloak.
2. **Nginx** routes all other calls to **AuthGuard (Rust)**.
3. **AuthGuard** validates the `Authorization` header using Keycloak's JWKS.
4. If the token is valid and the group is authorized (extracted from the `groups` claim), the request proceeds to the **Target Service**.

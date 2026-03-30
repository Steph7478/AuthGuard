#!/bin/bash
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}════════════════════════════════════════${NC}"
echo -e "${BLUE}        AuthGuard Test${NC}"
echo -e "${BLUE}════════════════════════════════════════${NC}"

echo -e "\n${YELLOW}O client é público ou confidencial?${NC}"
echo "1) Público (sem client secret)"
echo "2) Confidencial (com client secret)"
read -p "Escolha (1/2): " CLIENT_TYPE

if [ "$CLIENT_TYPE" = "2" ]; then
    read -p "Digite o client secret (padrão: qiQ8IospPnLNfL2y5x1hfs2Bpf10D0ky): " CLIENT_SECRET
    CLIENT_SECRET=${CLIENT_SECRET:-qiQ8IospPnLNfL2y5x1hfs2Bpf10D0ky}
    
    echo -e "\n${YELLOW}Escolha o tipo de fluxo:${NC}"
    echo "1) client_credentials (token do serviço - sem grupos)"
    echo "2) password (token do usuário - COM grupos)"
    read -p "Escolha (1/2): " FLOW
    
    if [ "$FLOW" = "2" ]; then
        read -p "Username (padrão: admin): " USERNAME
        USERNAME=${USERNAME:-admin}
        read -sp "Password (padrão: admin): " PASSWORD
        PASSWORD=${PASSWORD:-admin}
        echo ""
        
        RESPONSE=$(curl -s -X POST http://localhost:8080/realms/authguard/protocol/openid-connect/token \
          -H "Content-Type: application/x-www-form-urlencoded" \
          -d "client_id=authguard-service" \
          -d "client_secret=$CLIENT_SECRET" \
          -d "username=$USERNAME" \
          -d "password=$PASSWORD" \
          -d "grant_type=password")
    else
        RESPONSE=$(curl -s -X POST http://localhost:8080/realms/authguard/protocol/openid-connect/token \
          -H "Content-Type: application/x-www-form-urlencoded" \
          -d "client_id=authguard-service" \
          -d "client_secret=$CLIENT_SECRET" \
          -d "grant_type=client_credentials")
    fi
else
    read -p "Username (padrão: admin): " USERNAME
    USERNAME=${USERNAME:-admin}
    read -sp "Password (padrão: admin): " PASSWORD
    PASSWORD=${PASSWORD:-admin}
    echo ""
    
    RESPONSE=$(curl -s -X POST http://localhost:8080/realms/authguard/protocol/openid-connect/token \
      -H "Content-Type: application/x-www-form-urlencoded" \
      -d "client_id=authguard-service" \
      -d "username=$USERNAME" \
      -d "password=$PASSWORD" \
      -d "grant_type=password")
fi

echo -e "${YELLOW}Resposta do Keycloak:${NC}"
echo "$RESPONSE"
echo ""

# Extrair token se existir
TOKEN=$(echo "$RESPONSE" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)

if [ -n "$TOKEN" ]; then
    echo -e "${GREEN}✓ Token obtido${NC}"
    echo -e "${BLUE}Token: ${TOKEN:0:50}...${NC}"
    
    echo -e "\n${YELLOW}[2/3] Verificando grupos...${NC}"
    PAYLOAD=$(echo $TOKEN | cut -d. -f2)
    DECODED=$(echo $PAYLOAD | base64 -d 2>/dev/null)
    echo -e "${GREEN}Payload decodificado:${NC}"
    echo "$DECODED"
    echo ""
    
    echo -e "\n${YELLOW}[3/3] Testando AuthGuard...${NC}"
    curl -s -w "\n${BLUE}HTTP Status: %{http_code}${NC}\n" \
      -H "Authorization: Bearer $TOKEN" \
      http://localhost:3000/admin
else
    echo -e "\n${RED}✗ Erro ao obter token${NC}"
    echo -e "${YELLOW}Possíveis causas:${NC}"
    echo "  - Client não existe"
    echo "  - Client secret inválido"
    echo "  - Client não tem serviceAccountsEnabled (para client_credentials)"
    echo "  - Usuário não existe (para password)"
    echo "  - Senha incorreta (para password)"
fi

echo -e "\n${BLUE}════════════════════════════════════════${NC}"
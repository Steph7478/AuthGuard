#!/bin/bash
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}════════════════════════════════════════${NC}"
echo -e "${BLUE}        AuthGuard Test${NC}"
echo -e "${BLUE}════════════════════════════════════════${NC}"

echo -e "\n${YELLOW}Is the client public or confidential?${NC}"
echo "1) Public (no client secret)"
echo "2) Confidential (with client secret)"
read -p "Choose (1/2): " CLIENT_TYPE

if [ "$CLIENT_TYPE" = "2" ]; then
    read -p "Enter client secret (default: qiQ8IospPnLNfL2y5x1hfs2Bpf10D0ky): " CLIENT_SECRET
    CLIENT_SECRET=${CLIENT_SECRET:-qiQ8IospPnLNfL2y5x1hfs2Bpf10D0ky}
    
    echo -e "\n${YELLOW}Choose the flow type:${NC}"
    echo "1) client_credentials (service token - no groups)"
    echo "2) password (user token - WITH groups)"
    read -p "Choose (1/2): " FLOW
    
    if [ "$FLOW" = "2" ]; then
        read -p "Username (default: admin): " USERNAME
        USERNAME=${USERNAME:-admin}
        read -sp "Password (default: admin): " PASSWORD
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
    read -p "Username (default: admin): " USERNAME
    USERNAME=${USERNAME:-admin}
    read -sp "Password (default: admin): " PASSWORD
    PASSWORD=${PASSWORD:-admin}
    echo ""
    
    RESPONSE=$(curl -s -X POST http://localhost:8080/realms/authguard/protocol/openid-connect/token \
      -H "Content-Type: application/x-www-form-urlencoded" \
      -d "client_id=authguard-service" \
      -d "username=$USERNAME" \
      -d "password=$PASSWORD" \
      -d "grant_type=password")
fi

TOKEN=$(echo "$RESPONSE" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)

if [ -n "$TOKEN" ]; then
    echo -e "${GREEN}✓ Token obtained${NC}"
    
    echo -e "\n${YELLOW}Testing AuthGuard...${NC}"
    curl -s -w "\n${BLUE}HTTP Status: %{http_code}${NC}\n" \
      -H "Authorization: Bearer $TOKEN" \
      http://localhost:3000/admin
else
    echo -e "\n${RED}✗ Failed to obtain token${NC}"
    echo -e "${YELLOW}Response:${NC} $RESPONSE"
fi

echo -e "\n${BLUE}════════════════════════════════════════${NC}"
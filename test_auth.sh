#!/bin/bash

# Terminal Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}          AuthGuard Full Stack - Test Suite           ${NC}"
echo -e "${BLUE}══════════════════════════════════════════════════════${NC}"

# --- 1. ENVIRONMENT SETTINGS ---
# Default secret confirmed in your setup
DEFAULT_SECRET="qiQ8IospPnLNfL2y5x1hfs2Bpf10D0ky"

KEYCLOAK_URL="http://localhost/auth/realms/authguard/protocol/openid-connect/token"
API_GATEWAY_URL="http://localhost/api/users/me"
AUTHGUARD_HEALTH="http://localhost:3000/health"

# --- 2. AUTHGUARD HEALTH CHECK ---
echo -e "\n${YELLOW}[2/6] Testing AuthGuard Connectivity:${NC}"
HEALTH_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" "$AUTHGUARD_HEALTH" 2>/dev/null)

if [ "$HEALTH_RESPONSE" = "200" ]; then
    echo -e "${GREEN}✓ AuthGuard is Online and Public (200 OK)${NC}"
elif [ "$HEALTH_RESPONSE" = "401" ]; then
    echo -e "${YELLOW}! AuthGuard is Active but /health is Protected (401)${NC}"
    echo -e "${BLUE}  Note: Move /health outside the middleware layer in Rust to fix this.${NC}"
else
    echo -e "${RED}✗ Failed to connect to AuthGuard (Status: $HEALTH_RESPONSE)${NC}"
    echo "  Check if port 3000 is open and the container is running."
fi

# --- 3. CLIENT CONFIGURATION ---
echo -e "\n${YELLOW}[3/6] Keycloak Client Configuration:${NC}"
echo "1) Public (No Client Secret)"
echo "2) Confidential (With Client Secret)"
read -p "Select (1/2): " CLIENT_TYPE

# Clear any existing DATA variable
DATA=""

if [ "$CLIENT_TYPE" = "2" ]; then
    read -p "Enter Client Secret (Press Enter for default): " CLIENT_SECRET
    CLIENT_SECRET=${CLIENT_SECRET:-$DEFAULT_SECRET}
    
    echo -e "\n${YELLOW}[4/6] Authentication Flow Type:${NC}"
    echo "1) client_credentials (Service-to-Service)"
    echo "2) password (Real User - Supports Roles/Groups)"
    read -p "Select (1/2): " FLOW
    
    if [ "$FLOW" = "2" ]; then
        read -p "Username (default: admin): " USERNAME
        USERNAME=${USERNAME:-admin}
        read -sp "Password (default: admin): " PASSWORD
        PASSWORD=${PASSWORD:-admin}
        echo ""
        DATA="-d client_id=authguard-service -d client_secret=$CLIENT_SECRET -d username=$USERNAME -d password=$PASSWORD -d grant_type=password"
    else
        DATA="-d client_id=authguard-service -d client_secret=$CLIENT_SECRET -d grant_type=client_credentials"
    fi
else
    read -p "Username (default: admin): " USERNAME
    USERNAME=${USERNAME:-admin}
    read -sp "Password (default: admin): " PASSWORD
    PASSWORD=${PASSWORD:-admin}
    echo ""
    DATA="-d client_id=authguard-service -d username=$USERNAME -d password=$PASSWORD -d grant_type=password"
fi

# --- 4. TOKEN RETRIEVAL ---
echo -e "\n${BLUE}Requesting Token from Keycloak: ${KEYCLOAK_URL}...${NC}"
RESPONSE=$(curl -s -X POST "$KEYCLOAK_URL" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    $DATA)

# Extract token more reliably
TOKEN=$(echo "$RESPONSE" | grep -o '"access_token":"[^"]*"' | head -1 | cut -d'"' -f4)

if [ -n "$TOKEN" ] && [ "$TOKEN" != "null" ] && [ "$TOKEN" != "" ]; then
    echo -e "${GREEN}✓ Token obtained successfully!${NC}"
    
    # Display Claims if jq is installed
    if command -v jq &> /dev/null; then
        echo -e "\n${YELLOW}[5/6] Token Payload (Claims):${NC}"
        # Extract and decode the payload part (second segment)
        PAYLOAD=$(echo "$TOKEN" | cut -d'.' -f2)
        # Add padding to base64 if needed
        while [ $((${#PAYLOAD} % 4)) -ne 0 ]; do PAYLOAD="${PAYLOAD}="; done
        echo "$PAYLOAD" | base64 -d 2>/dev/null | jq '.' || echo "Error decoding token JSON."
    fi

    # --- 5. NGINX GATEWAY TEST ---
    echo -e "\n${YELLOW}[6/6] Testing Access via Nginx Gateway:${NC}"
    echo -e "${BLUE}Calling: ${API_GATEWAY_URL}${NC}"
    
    RESULT=$(curl -s -w "\nHTTP_CODE:%{http_code}" \
      -H "Authorization: Bearer $TOKEN" \
      "$API_GATEWAY_URL" 2>/dev/null)
    
    BODY=$(echo "$RESULT" | sed '/HTTP_CODE:/d')
    STATUS=$(echo "$RESULT" | grep "HTTP_CODE" | cut -d':' -f2)

    if [ "$STATUS" = "200" ]; then
        echo -e "${GREEN}✓ SUCCESS: Access Granted (200 OK)${NC}"
        echo -e "${BLUE}User-Service Response:${NC} $BODY"
    elif [ "$STATUS" = "401" ]; then
        echo -e "${RED}✗ ERROR: Unauthorized (401)${NC}"
        echo -e "${YELLOW}Tip: Check if AuthGuard correctly validated the Token Issuer (localhost vs keycloak_auth).${NC}"
    elif [ "$STATUS" = "403" ]; then
        echo -e "${RED}✗ ERROR: Forbidden (403)${NC}"
        echo -e "${YELLOW}Tip: Valid token, but insufficient permissions or Rate Limit hit.${NC}"
    elif [ "$STATUS" = "500" ]; then
        echo -e "${RED}✗ ERROR: Internal Server Error (500)${NC}"
        echo -e "${YELLOW}Tip: Check Nginx logs. It usually means AuthGuard returned 404 on the /validate route.${NC}"
    else
        echo -e "${RED}✗ ERROR: HTTP Status $STATUS${NC}"
        echo "Response Body: $BODY"
    fi
    
else
    echo -e "\n${RED}✗ Failed to obtain token from Keycloak${NC}"
    echo -e "${YELLOW}Server Response:${NC}"
    if command -v jq &> /dev/null; then
        echo "$RESPONSE" | jq '.'
    else
        echo "$RESPONSE"
    fi
    
    echo -e "\n${YELLOW}Quick Diagnosis:${NC}"
    echo "1. Does the 'authguard' Realm exist in Keycloak?"
    echo "2. Is the 'authguard-service' client configured correctly?"
    echo "3. Check Keycloak logs: docker logs keycloak_auth"
    echo "4. Verify nginx proxy is correctly forwarding to Keycloak: docker logs nginx_gateway"
fi
#!/bin/bash

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

show_menu() {
    echo -e "${BLUE}════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}           AuthGuard - Development Environment           ${NC}"
    echo -e "${BLUE}════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "${YELLOW}1) LOCAL${NC} - Run AuthGuard natively + Docker services"
    echo -e "${YELLOW}2) DEV${NC}   - Run everything in Docker with hot reload"
    echo -e "${YELLOW}3) PROD${NC}  - Run everything in Docker in production mode"
    echo -e "${YELLOW}4) STOP${NC}  - Stop all containers"
    echo -e "${YELLOW}5) LOGS${NC}  - View logs"
    echo -e "${RED}0) EXIT${NC}  - Exit"
    echo ""
}

start_local() {
    echo -e "${GREEN}Starting Docker services...${NC}"
    docker-compose --profile local up -d
    echo ""
    echo -e "${GREEN}✓ Services running!${NC}"
    echo -e "  • Nginx:      ${BLUE}http://localhost${NC}"
    echo -e "  • Keycloak:   ${BLUE}http://localhost:8080${NC}"
    echo -e "  • Redis:      ${BLUE}localhost:6379${NC}"
    echo ""
    echo -e "${GREEN}To run AuthGuard natively:${NC}"
    echo -e "  ${BLUE}cd AuthGuard && cargo run${NC}"
}

start_dev() {
    echo -e "${GREEN}Starting DEV environment with hot reload...${NC}"
    docker-compose --profile dev up -d
    echo ""
    echo -e "${GREEN}✓ DEV environment running!${NC}"
    echo -e "  • AuthGuard:  ${BLUE}http://localhost:3000${NC} (hot reload active)"
    echo -e "  • Nginx:      ${BLUE}http://localhost${NC}"
    echo -e "  • Keycloak:   ${BLUE}http://localhost:8080${NC}"
    echo ""
    echo -e "${GREEN}To view logs:${NC}"
    echo -e "  ${BLUE}docker-compose logs -f authguard-dev${NC}"
}

start_prod() {
    echo -e "${GREEN}Starting PROD environment...${NC}"
    docker-compose --profile prod up -d
    echo ""
    echo -e "${GREEN}✓ PROD environment running!${NC}"
    echo -e "  • AuthGuard:  ${BLUE}http://localhost:3000${NC}"
    echo -e "  • Nginx:      ${BLUE}http://localhost${NC}"
    echo -e "  • Keycloak:   ${BLUE}http://localhost:8080${NC}"
}

stop_all() {
    echo -e "${RED}Stopping all containers...${NC}"
    docker-compose --profile local down
    docker-compose --profile dev down
    docker-compose --profile prod down
    echo -e "${GREEN}✓ All containers stopped!${NC}"
}

show_logs() {
    echo -e "${YELLOW}Choose service:${NC}"
    echo "1) authguard-dev"
    echo "2) nginx"
    echo "3) keycloak"
    echo "4) redis"
    echo "5) all"
    read -p "Choose [1-5]: " log_choice
    
    case $log_choice in
        1) docker-compose logs -f authguard-dev ;;
        2) docker-compose logs -f nginx ;;
        3) docker-compose logs -f keycloak ;;
        4) docker-compose logs -f redis ;;
        5) docker-compose logs -f ;;
        *) echo -e "${RED}Invalid option${NC}" ;;
    esac
}

while true; do
    show_menu
    read -p "Choose [0-5]: " choice
    
    case $choice in
        1) start_local ;;
        2) start_dev ;;
        3) start_prod ;;
        4) stop_all ;;
        5) show_logs ;;
        0) echo -e "${GREEN}Goodbye!${NC}"; exit 0 ;;
        *) echo -e "${RED}Invalid option!${NC}" ;;
    esac
    
    echo ""
    read -p "Press Enter to continue..."
    clear
done
#!/bin/bash

# 🔮 Codex Control Engine - PostgreSQL Database Setup
# This script sets up the PostgreSQL database for the sacred transformation platform

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${PURPLE}"
echo "🔮 Codex Control Engine Database Setup"
echo "=================================="
echo -e "${NC}"

# Check if PostgreSQL is installed
check_postgres() {
    if command -v psql >/dev/null 2>&1; then
        echo -e "${GREEN}✓ PostgreSQL found${NC}"
        POSTGRES_VERSION=$(psql --version | awk '{print $3}' | head -n1)
        echo -e "${BLUE}  Version: $POSTGRES_VERSION${NC}"
    else
        echo -e "${RED}✗ PostgreSQL not found${NC}"
        echo -e "${YELLOW}Please install PostgreSQL:${NC}"
        echo -e "${YELLOW}  macOS: brew install postgresql${NC}"
        echo -e "${YELLOW}  Ubuntu: sudo apt-get install postgresql postgresql-contrib${NC}"
        echo -e "${YELLOW}  CentOS: sudo yum install postgresql postgresql-server${NC}"
        exit 1
    fi
}

# Check if PostgreSQL service is running
check_postgres_service() {
    if pg_isready >/dev/null 2>&1; then
        echo -e "${GREEN}✓ PostgreSQL service is running${NC}"
    else
        echo -e "${YELLOW}⚠ PostgreSQL service is not running${NC}"
        echo -e "${YELLOW}Starting PostgreSQL service...${NC}"
        
        # Try different methods to start PostgreSQL
        if command -v brew >/dev/null 2>&1; then
            brew services start postgresql || true
        elif command -v systemctl >/dev/null 2>&1; then
            sudo systemctl start postgresql || true
        elif command -v service >/dev/null 2>&1; then
            sudo service postgresql start || true
        fi
        
        # Check again
        sleep 2
        if pg_isready >/dev/null 2>&1; then
            echo -e "${GREEN}✓ PostgreSQL service started${NC}"
        else
            echo -e "${RED}✗ Failed to start PostgreSQL service${NC}"
            echo -e "${YELLOW}Please start PostgreSQL manually and run this script again${NC}"
            exit 1
        fi
    fi
}

# Create database and user
setup_database() {
    echo -e "${BLUE}🔧 Setting up database and user...${NC}"
    
    # Database configuration from .env.example
    DB_NAME="codex_sacred"
    DB_USER="codex_user"
    DB_PASSWORD="sacred_password"
    
    echo -e "${YELLOW}Creating database: $DB_NAME${NC}"
    echo -e "${YELLOW}Creating user: $DB_USER${NC}"
    
    # Create user and database
    psql -h localhost -U postgres -c "DROP DATABASE IF EXISTS $DB_NAME;" 2>/dev/null || true
    psql -h localhost -U postgres -c "DROP USER IF EXISTS $DB_USER;" 2>/dev/null || true
    
    psql -h localhost -U postgres -c "CREATE USER $DB_USER WITH PASSWORD '$DB_PASSWORD';"
    psql -h localhost -U postgres -c "CREATE DATABASE $DB_NAME OWNER $DB_USER;"
    psql -h localhost -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;"
    
    echo -e "${GREEN}✓ Database and user created successfully${NC}"
}

# Run migrations
run_migrations() {
    echo -e "${BLUE}🔧 Running database migrations...${NC}"
    
    if [ -f "migrations/001_initial_schema.sql" ]; then
        echo -e "${YELLOW}Applying initial schema...${NC}"
        PGPASSWORD=sacred_password psql -h localhost -U codex_user -d codex_sacred -f migrations/001_initial_schema.sql
        echo -e "${GREEN}✓ Initial schema applied${NC}"
    else
        echo -e "${RED}✗ Migration file not found: migrations/001_initial_schema.sql${NC}"
        exit 1
    fi
}

# Test database connection
test_connection() {
    echo -e "${BLUE}🔧 Testing database connection...${NC}"
    
    PGPASSWORD=sacred_password psql -h localhost -U codex_user -d codex_sacred -c "SELECT 'Sacred connection established!' as message;" 2>/dev/null
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Database connection test successful${NC}"
    else
        echo -e "${RED}✗ Database connection test failed${NC}"
        exit 1
    fi
}

# Display connection info
display_connection_info() {
    echo -e "${PURPLE}"
    echo "🔮 Database Setup Complete!"
    echo "=========================="
    echo -e "${NC}"
    echo -e "${GREEN}Database Name:${NC} codex_sacred"
    echo -e "${GREEN}Database User:${NC} codex_user"
    echo -e "${GREEN}Database Password:${NC} sacred_password"
    echo -e "${GREEN}Connection URL:${NC} postgresql://codex_user:sacred_password@localhost:5432/codex_sacred"
    echo ""
    echo -e "${YELLOW}Your .env file is already configured with these settings.${NC}"
    echo -e "${YELLOW}You can now start the Rust backend server:${NC}"
    echo -e "${BLUE}  cargo run --bin codex-server${NC}"
    echo ""
}

# Main execution
main() {
    echo -e "${BLUE}Checking system requirements...${NC}"
    check_postgres
    check_postgres_service
    
    echo -e "${BLUE}Setting up sacred database...${NC}"
    setup_database
    run_migrations
    test_connection
    
    display_connection_info
    
    echo -e "${PURPLE}🔮 Sacred database is ready for transformation! 🔮${NC}"
}

# Handle errors gracefully
trap 'echo -e "${RED}✗ Setup failed. Please check the error above and try again.${NC}"' ERR

# Run main function
main "$@"
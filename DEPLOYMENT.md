# Codex Control Engine - Production Deployment Guide

A comprehensive guide for deploying the Codex Control Engine - Sacred Digital Transformation Platform.

## 🎯 Overview

The Codex Control Engine is a full-stack spiritual transformation platform consisting of:
- **Backend**: Rust + Axum web server with PostgreSQL database
- **Frontend**: Next.js 15 with TypeScript and Tailwind CSS
- **AI Integration**: OpenRouter API with Claude 3.5 Sonnet for archetypal analysis
- **WASM Runtime**: WebAssembly support for custom ritual modules

## 📋 Prerequisites

### System Requirements
- **OS**: Ubuntu 20.04+ / macOS 10.15+ / Windows 10+
- **Memory**: 4GB+ RAM (8GB+ recommended)
- **Storage**: 20GB+ free space
- **Network**: HTTPS-capable domain with SSL certificate

### Required Software
- **Rust**: 1.75+ (via rustup)
- **Node.js**: 18+ with npm
- **PostgreSQL**: 14+
- **Docker** (optional but recommended)
- **Nginx** (for production reverse proxy)

## 🚀 Quick Start (Development)

### 1. Clone and Setup
```bash
git clone https://github.com/your-org/codex-control-engine.git
cd codex-control-engine

# Copy environment configuration
cp .env.example .env

# Run automated database setup
./setup_database.sh
```

### 2. Configure Environment
Edit `.env` file with your configuration:

```bash
# Database Configuration  
DATABASE_URL=postgresql://codex_user:sacred_password@localhost:5432/codex_sacred

# AI Oracle Configuration (Get key from OpenRouter.ai)
OPENROUTER_API_KEY=sk-or-your-openrouter-api-key-here
DEFAULT_AI_MODEL=anthropic/claude-3-haiku

# JWT Authentication (Generate 256-bit secret)
JWT_SECRET=your-256-bit-secret-key-change-in-production

# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=3001

# CORS Configuration  
CORS_ALLOWED_ORIGINS=http://localhost:3000,https://yourdomain.com

# Logging
RUST_LOG=info,codex_control_engine=debug

# WASM Configuration
WASM_MAX_MEMORY_MB=256
WASM_EXECUTION_TIMEOUT_MS=30000
```

### 3. Start Services
```bash
# Terminal 1: Start backend
cargo run --bin codex-server

# Terminal 2: Start frontend  
cd web && npm install && npm run dev
```

Access at: http://localhost:3000

## 🏭 Production Deployment

### Option 1: Docker Deployment (Recommended)

Create `docker-compose.prod.yml`:
```yaml
version: '3.8'
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: codex_sacred
      POSTGRES_USER: codex_user
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./migrations:/docker-entrypoint-initdb.d
    networks:
      - codex_network
    restart: unless-stopped

  backend:
    build:
      context: .
      dockerfile: Dockerfile.backend
    environment:
      DATABASE_URL: postgresql://codex_user:${DB_PASSWORD}@postgres:5432/codex_sacred
      OPENROUTER_API_KEY: ${OPENROUTER_API_KEY}
      JWT_SECRET: ${JWT_SECRET}
      SERVER_HOST: 0.0.0.0
      SERVER_PORT: 3001
      RUST_LOG: info
    depends_on:
      - postgres
    networks:
      - codex_network
    restart: unless-stopped

  frontend:
    build:
      context: ./web
      dockerfile: Dockerfile
    environment:
      NEXT_PUBLIC_API_URL: https://api.yourdomain.com
      NEXT_PUBLIC_APP_NAME: "Codex Control Engine"
    networks:
      - codex_network
    restart: unless-stopped

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - backend
      - frontend
    networks:
      - codex_network
    restart: unless-stopped

volumes:
  postgres_data:

networks:
  codex_network:
    driver: bridge
```

#### Create Backend Dockerfile
```dockerfile
# Dockerfile.backend
FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release --bin codex-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/codex-server /usr/local/bin/

EXPOSE 3001
CMD ["codex-server"]
```

#### Create Frontend Dockerfile
```dockerfile
# web/Dockerfile
FROM node:18-alpine as builder

WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

FROM node:18-alpine
WORKDIR /app
COPY --from=builder /app/.next ./.next
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./package.json

EXPOSE 3000
CMD ["npm", "start"]
```

#### Deploy with Docker
```bash
# Create production environment file
cp .env.example .env.prod

# Edit .env.prod with production values
vim .env.prod

# Deploy with Docker Compose
docker-compose -f docker-compose.prod.yml --env-file .env.prod up -d
```

### Option 2: Manual Production Deployment

#### 1. Server Setup (Ubuntu)
```bash
# Install dependencies
sudo apt update && sudo apt upgrade -y
sudo apt install -y postgresql postgresql-contrib nginx certbot python3-certbot-nginx

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs
```

#### 2. Database Setup
```bash
# Create database and user
sudo -u postgres psql << EOF
CREATE DATABASE codex_sacred;
CREATE USER codex_user WITH ENCRYPTED PASSWORD 'secure_production_password';
GRANT ALL PRIVILEGES ON DATABASE codex_sacred TO codex_user;
\q
EOF

# Run migrations
cargo run --bin codex-server
```

#### 3. Build and Deploy Backend
```bash
# Build backend in release mode
cargo build --release --bin codex-server

# Create systemd service
sudo tee /etc/systemd/system/codex-backend.service > /dev/null << EOF
[Unit]
Description=Codex Control Engine Backend
After=network.target postgresql.service

[Service]
Type=simple
User=codex
WorkingDirectory=/opt/codex
Environment=DATABASE_URL=postgresql://codex_user:secure_production_password@localhost:5432/codex_sacred
Environment=OPENROUTER_API_KEY=your_api_key_here
Environment=JWT_SECRET=your_256_bit_secret_here
Environment=SERVER_HOST=0.0.0.0
Environment=SERVER_PORT=3001
Environment=RUST_LOG=info
ExecStart=/opt/codex/target/release/codex-server
Restart=always

[Install]
WantedBy=multi-user.target
EOF

# Start service
sudo systemctl daemon-reload
sudo systemctl enable codex-backend
sudo systemctl start codex-backend
```

#### 4. Build and Deploy Frontend
```bash
cd web

# Install dependencies and build
npm ci --only=production
npm run build

# Create systemd service for Next.js
sudo tee /etc/systemd/system/codex-frontend.service > /dev/null << EOF
[Unit]
Description=Codex Control Engine Frontend
After=network.target

[Service]
Type=simple
User=codex
WorkingDirectory=/opt/codex/web
Environment=NODE_ENV=production
Environment=NEXT_PUBLIC_API_URL=https://api.yourdomain.com
ExecStart=/usr/bin/npm start
Restart=always

[Install]
WantedBy=multi-user.target
EOF

# Start service
sudo systemctl daemon-reload
sudo systemctl enable codex-frontend
sudo systemctl start codex-frontend
```

#### 5. Configure Nginx Reverse Proxy
```nginx
# /etc/nginx/sites-available/codex-control-engine
server {
    listen 80;
    server_name yourdomain.com www.yourdomain.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name yourdomain.com www.yourdomain.com;

    ssl_certificate /etc/letsencrypt/live/yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/yourdomain.com/privkey.pem;

    # Security headers
    add_header X-Content-Type-Options nosniff;
    add_header X-Frame-Options SAMEORIGIN;
    add_header X-XSS-Protection "1; mode=block";
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    # Frontend
    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Backend API
    location /api/ {
        proxy_pass http://localhost:3001;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # WebSocket support for future features
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

Enable site and get SSL certificate:
```bash
sudo ln -s /etc/nginx/sites-available/codex-control-engine /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx

# Get SSL certificate
sudo certbot --nginx -d yourdomain.com -d www.yourdomain.com
```

## 🔒 Security Considerations

### Environment Security
- **Never commit `.env` files to version control**
- Use strong, unique passwords for database and JWT secrets
- Rotate API keys regularly
- Use environment-specific configurations

### Database Security
```sql
-- Create read-only user for monitoring
CREATE USER codex_monitor WITH ENCRYPTED PASSWORD 'monitor_password';
GRANT CONNECT ON DATABASE codex_sacred TO codex_monitor;
GRANT USAGE ON SCHEMA public TO codex_monitor;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO codex_monitor;

-- Enable row-level security (example)
ALTER TABLE practitioners ENABLE ROW LEVEL SECURITY;
CREATE POLICY practitioners_isolation ON practitioners
    USING (id = current_user_id());
```

### Network Security
- Use firewall to restrict access to database port (5432)
- Enable fail2ban for SSH protection
- Use strong SSL/TLS configuration
- Implement rate limiting in Nginx

### Application Security
- Validate all user inputs
- Use parameterized queries (already implemented with SQLx)
- Implement proper CORS policies
- Sanitize file uploads for WASM modules

## 📊 Monitoring and Maintenance

### Health Checks
```bash
# Backend health
curl https://api.yourdomain.com/api/health

# Database health  
psql -h localhost -U codex_user -d codex_sacred -c "SELECT version();"

# Service status
sudo systemctl status codex-backend codex-frontend postgresql nginx
```

### Log Management
```bash
# Backend logs
sudo journalctl -u codex-backend -f

# Frontend logs
sudo journalctl -u codex-frontend -f

# Nginx access logs
sudo tail -f /var/log/nginx/access.log

# Database logs
sudo tail -f /var/log/postgresql/postgresql-14-main.log
```

### Backup Strategy
```bash
#!/bin/bash
# /opt/codex/backup.sh

# Database backup
pg_dump -h localhost -U codex_user codex_sacred | gzip > /backup/codex_db_$(date +%Y%m%d_%H%M%S).sql.gz

# Application backup
tar -czf /backup/codex_app_$(date +%Y%m%d_%H%M%S).tar.gz /opt/codex

# Keep only 7 days of backups
find /backup -name "codex_*" -mtime +7 -delete

# Add to crontab for daily backups:
# 0 2 * * * /opt/codex/backup.sh
```

### Performance Tuning

#### PostgreSQL
```sql
-- /etc/postgresql/14/main/postgresql.conf
shared_buffers = 256MB
effective_cache_size = 1GB
work_mem = 4MB
maintenance_work_mem = 64MB
```

#### Nginx
```nginx
# /etc/nginx/nginx.conf
worker_processes auto;
worker_connections 1024;

# Enable gzip compression
gzip on;
gzip_vary on;
gzip_min_length 1024;
gzip_comp_level 6;
gzip_types text/plain text/css application/json application/javascript text/xml application/xml+rss text/javascript;
```

## 🔧 Troubleshooting

### Common Issues

#### Database Connection Issues
```bash
# Check PostgreSQL status
sudo systemctl status postgresql

# Check connection
psql -h localhost -U codex_user -d codex_sacred -c "SELECT 1;"

# Check logs
sudo tail -f /var/log/postgresql/postgresql-14-main.log
```

#### Backend Service Issues  
```bash
# Check service status
sudo systemctl status codex-backend

# Check logs
sudo journalctl -u codex-backend -f --lines=50

# Restart service
sudo systemctl restart codex-backend
```

#### Frontend Issues
```bash
# Check build errors
cd web && npm run build

# Check service
sudo systemctl status codex-frontend

# Restart
sudo systemctl restart codex-frontend
```

#### SSL Certificate Issues
```bash
# Renew certificate
sudo certbot renew

# Test SSL
sudo certbot certificates
```

### Performance Issues
- Monitor with `htop`, `iotop`, and `nethogs`
- Check PostgreSQL slow query log
- Review Nginx access patterns
- Monitor memory usage of WASM runtime

## 🚀 Scaling Considerations

### Database Scaling
- Configure connection pooling
- Set up read replicas for heavy read workloads
- Consider partitioning large tables (ritual_sessions, archetypal_states)
- Implement database monitoring with pgAdmin or similar

### Application Scaling
- Use multiple backend instances with load balancer
- Implement Redis for session storage
- Add CDN for static assets
- Consider containerization with Kubernetes

### Monitoring at Scale
- Implement structured logging (JSON format)
- Use monitoring tools (Prometheus + Grafana)
- Set up alerting for critical metrics
- Monitor WASM execution resources

## 📝 Maintenance Checklist

### Daily
- [ ] Check service status
- [ ] Review error logs
- [ ] Monitor disk usage
- [ ] Verify SSL certificate validity

### Weekly  
- [ ] Update system packages
- [ ] Review backup integrity
- [ ] Check database performance
- [ ] Analyze user activity patterns

### Monthly
- [ ] Update dependencies (cargo update, npm update)
- [ ] Review and rotate API keys
- [ ] Audit user permissions
- [ ] Performance optimization review
- [ ] Security audit

### Quarterly
- [ ] Major version updates
- [ ] Infrastructure review
- [ ] Disaster recovery testing
- [ ] Security penetration testing

## 🆘 Support

### Getting Help
- **Documentation**: [docs.sacred.dev](https://docs.sacred.dev)
- **Issues**: [GitHub Issues](https://github.com/your-org/codex-control-engine/issues)
- **Community**: [Discord Server](https://discord.gg/codex-sacred)
- **Email**: support@sacred.dev

### Contributing
See [CONTRIBUTING.md](./CONTRIBUTING.md) for development guidelines.

### License
This project is licensed under the MIT License - see [LICENSE](./LICENSE) for details.

---

*May this sacred technology serve the highest good of all practitioners on their journey of spiritual transformation.* 🔮✨
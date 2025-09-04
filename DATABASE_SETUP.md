# 🔮 PostgreSQL Database Setup Guide

This guide will help you set up the PostgreSQL database for the Codex Control Engine sacred transformation platform.

## 🚀 **Quick Setup Options**

### Option 1: Automated Script (Recommended)
```bash
# Run the automated setup script
./setup_database.sh
```

### Option 2: Docker Compose (Easiest)
```bash
# Start PostgreSQL with Docker
docker-compose up -d postgres

# Wait for database to be ready (about 10 seconds)
docker-compose logs postgres
```

### Option 3: Manual Installation
Follow the manual steps below if you prefer full control.

---

## 📋 **Manual Installation Steps**

### Step 1: Install PostgreSQL

**macOS (Homebrew):**
```bash
brew install postgresql
brew services start postgresql
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

**CentOS/RHEL:**
```bash
sudo yum install postgresql postgresql-server
sudo postgresql-setup initdb
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

### Step 2: Create Database and User

```bash
# Connect to PostgreSQL as superuser
sudo -u postgres psql

# Create user and database
CREATE USER codex_user WITH PASSWORD 'sacred_password';
CREATE DATABASE codex_sacred OWNER codex_user;
GRANT ALL PRIVILEGES ON DATABASE codex_sacred TO codex_user;
\q
```

### Step 3: Apply Schema Migrations

```bash
# Run the initial schema migration
PGPASSWORD=sacred_password psql -h localhost -U codex_user -d codex_sacred -f migrations/001_initial_schema.sql
```

### Step 4: Test Connection

```bash
# Test the database connection
PGPASSWORD=sacred_password psql -h localhost -U codex_user -d codex_sacred -c "SELECT 'Sacred connection established!' as message;"
```

---

## 🔧 **Database Configuration**

### Environment Variables
Your `.env` file should contain:
```bash
DATABASE_URL=postgresql://codex_user:sacred_password@localhost:5432/codex_sacred
```

### Connection Details
- **Database Name:** `codex_sacred`
- **Username:** `codex_user`
- **Password:** `sacred_password`
- **Host:** `localhost`
- **Port:** `5432`

---

## 🐳 **Docker Development Environment**

### Database Only
```bash
# Start just the PostgreSQL database
docker-compose up -d postgres

# Check database logs
docker-compose logs postgres

# Connect to database
docker-compose exec postgres psql -U codex_user -d codex_sacred
```

### Full Stack Development
```bash
# Start database, backend, and frontend
docker-compose --profile full-stack up -d

# View all services
docker-compose ps

# View logs
docker-compose logs -f
```

### Docker Commands
```bash
# Stop services
docker-compose down

# Reset database (removes all data!)
docker-compose down -v
docker-compose up -d postgres

# Backup database
docker-compose exec postgres pg_dump -U codex_user codex_sacred > backup.sql

# Restore database
docker-compose exec -T postgres psql -U codex_user codex_sacred < backup.sql
```

---

## 🔍 **Troubleshooting**

### Connection Issues

**Error: `role "codex_user" does not exist`**
```bash
# Recreate the user
sudo -u postgres psql -c "CREATE USER codex_user WITH PASSWORD 'sacred_password';"
```

**Error: `database "codex_sacred" does not exist`**
```bash
# Recreate the database
sudo -u postgres psql -c "CREATE DATABASE codex_sacred OWNER codex_user;"
```

**Error: `connection refused`**
```bash
# Check if PostgreSQL is running
pg_isready

# Start PostgreSQL service
# macOS: brew services start postgresql
# Linux: sudo systemctl start postgresql
```

### Permission Issues

**Error: `permission denied for schema public`**
```bash
# Grant schema permissions
sudo -u postgres psql -d codex_sacred -c "GRANT ALL ON SCHEMA public TO codex_user;"
sudo -u postgres psql -d codex_sacred -c "GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO codex_user;"
sudo -u postgres psql -d codex_sacred -c "GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO codex_user;"
```

### Reset Everything
```bash
# Complete reset (removes all data!)
sudo -u postgres psql -c "DROP DATABASE IF EXISTS codex_sacred;"
sudo -u postgres psql -c "DROP USER IF EXISTS codex_user;"

# Then run setup again
./setup_database.sh
```

---

## 📊 **Database Schema Overview**

The sacred database includes these core tables:

- **`practitioners`** - User accounts and spiritual profiles
- **`archetypal_states`** - Sacred state transformations
- **`sacred_rituals`** - Ritual definitions and WASM modules  
- **`ritual_sessions`** - Execution history and outcomes
- **`oracle_insights`** - AI reflections and interpretations

### Sample Queries
```sql
-- View all practitioners
SELECT spiritual_name, email, sacred_path FROM practitioners;

-- Check ritual library
SELECT name, tradition, difficulty_level, usage_count FROM sacred_rituals;

-- View recent transformations
SELECT created_at, transformation_intensity FROM ritual_sessions ORDER BY created_at DESC LIMIT 5;
```

---

## ✅ **Verification Steps**

After setup, verify everything works:

1. **Database Connection:**
   ```bash
   PGPASSWORD=sacred_password psql -h localhost -U codex_user -d codex_sacred -c "SELECT version();"
   ```

2. **Schema Loaded:**
   ```bash
   PGPASSWORD=sacred_password psql -h localhost -U codex_user -d codex_sacred -c "\dt"
   ```

3. **Sample Data:**
   ```bash
   PGPASSWORD=sacred_password psql -h localhost -U codex_user -d codex_sacred -c "SELECT COUNT(*) FROM sacred_rituals;"
   ```

4. **Backend Connection:**
   ```bash
   # Start the Rust server
   cargo run --bin codex-server
   
   # Should see: "🔮 Sacred server listening on http://127.0.0.1:3000"
   ```

---

## 🔮 **Ready for Sacred Transformation!**

Once your database is set up, you can:

1. Start the **Rust backend**: `cargo run --bin codex-server`
2. Start the **Next.js frontend**: `cd web && npm run dev`
3. Open **http://localhost:3000** for the sacred web interface
4. Begin your archetypal transformation journey! ✨
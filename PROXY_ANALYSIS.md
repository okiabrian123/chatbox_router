# Proxy Handler - Detailed Analysis

## Overview

Proxy handler adalah **central router** yang mengatur semua traffic antara frontend dan microservices. Ia berfungsi sebagai **gateway** yang melakukan routing, authentication, dan load balancing.

## Architecture

### **1. Central Gateway Pattern**
```
Frontend → Proxy Handler (Port 8080/443) → Microservices
                                      ├── chatbox-chat (AI API)
                                      ├── chatbox-user (User Management)
                                      ├── chatbox-email (Email Service)
                                      ├── chatbox-image (Image Processing)
                                      └── chatbox-tools (AI Tools)
```

### **2. Port Configuration**
- **8080**: HTTP traffic
- **443**: HTTPS traffic (dengan SSL certificates)

## Detailed Workflow

### **Initialization Process**

#### **Step 1: Configuration Loading**
```rust
// Baca konfigurasi dari .config/px.toml
let credentials = read_credentials(".config/px.toml")?;
// Format: sn=service_name, us=username, ky=encryption_key
```

#### **Step 2: SSL Certificate Setup**
```rust
let certs_path = exe_dir.join("certs/mycert.pem");
let key_path = exe_dir.join("certs/mykey.pem");
// Load SSL certificates untuk HTTPS
```

#### **Step 3: Password Manager Initialization**
```rust
let password_manager = initialize_password_manager(&sn, &us, &ky);
// Generate encryption key untuk session management
```

### **Request Routing Logic**

#### **Public Routes (No Authentication)**
```rust
// Routes yang bisa diakses tanpa login
"/" → Main AI interface
"/login" → Login page
"/register" → Registration page
"/privacy" → Privacy policy
"/terms" → Terms of service
"/subscriptions" → Subscription plans
"/notif_payment" → Payment notification (webhook)
```

#### **Protected Routes (Require Authentication)**
```rust
// Routes yang memerlukan authentication
.wrap(actix_web_lab::middleware::from_fn(check_auth_mw))
"/api/ai/*" → AI API endpoints
"/api/image/*" → Image processing API
"/payment" → Payment processing
"/check_payment" → Payment verification
"/show_user_data" → User data
```

### **Service Integration**

#### **AI Service Proxy**
```rust
// Forward ke AI service
"/" → proxy_ai_handler → chatbox-chat:8080
"/api/ai/*" → proxy_ai_handler → chatbox-chat:8080
```

#### **Image Service Proxy**
```rust
// Forward ke image service
"/api/image/*" → proxy_image_handler → chatbox-image:8080
```

#### **User Service Proxy**
```rust
// Forward ke user service
"/login" → proxy_user_post_login_handler → chatbox-user:8080
"/register" → proxy_user_post_handler → chatbox-user:8080
```

## Deployment Scripts Analysis

### **1. run.sh - Local Development**
```bash
# Usage: ./run.sh [IP] [PORT]
./run.sh 127.0.0.1 8080

# Process:
1. Extract package name dari Cargo.toml
2. Check binary existence, build jika belum ada
3. Validate config file (.config/px.toml)
4. Run dengan IP dan port specified
```

### **2. run_the_server.sh - Remote Deployment**
```bash
# Process:
1. Prompt untuk password SSH
2. Upload config file ke server (45.158.126.130:31111)
3. Kill existing process
4. Upload dan run binary baru
5. Run dengan nohup di background
```

## Security Features

### **1. SSL/TLS Encryption**
- Menggunakan certificates di `certs/mycert.pem` dan `certs/mykey.pem`
- Enforce HTTPS dengan `cookie_secure(true)`
- Hanya allow origins tertentu:
  - `https://testcasemanager.my.id`
  - `https://chatpintar.my.id`
  - `https://local3.testcasemanager.my.id`

### **2. Session Management**
```rust
SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
    .cookie_secure(true)
    .cookie_http_only(true)
    .cookie_name("login-session")
    .session_ttl(Duration::days(30))
```

### **3. Authentication Middleware**
```rust
// check_auth_mw validates session before accessing protected routes
.wrap(actix_web_lab::middleware::from_fn(check_auth_mw))
```

## Docker Deployment

### **docker-compose.prod.yml Configuration**
```yaml
services:
  chatbox-router:
    image: docker.io/okiabrian/chatbox-router:latest
    ports:
      - "8080:8080"  # HTTP
      - "443:443"    # HTTPS
    volumes:
      - ./.config:/app/.config:ro      # Config files
      - ./certs:/app/certs:ro          # SSL certificates
      - ./logs:/var/log/program        # Logs
```

## Complete Deployment Strategy

### **Step 1: Deploy All Microservices**
```bash
# Deploy setiap service
docker run -d --name chatbox-chat -p 8080:8080 docker.io/okiabrian123/chatbox-chat:latest
docker run -d --name chatbox-user -p 8081:8080 docker.io/okiabrian123/chatbox-user:latest
docker run -d --name chatbox-email -p 8082:8080 docker.io/okiabrian123/chatbox-email:latest
docker run -d --name chatbox-image -p 8083:8083 docker.io/okiabrian123/chatbox-image:latest
docker run -d --name chatbox-tools -p 8084:8084 docker.io/okiabrian123/chatbox-tools:latest
```

### **Step 2: Deploy Proxy Handler**
```bash
# Proxy handler sebagai gateway
docker run -d --name chatbox-router \
  -p 8080:8080 -p 443:443 \
  -v ./.config:/app/.config:ro \
  -v ./certs:/app/certs:ro \
  docker.io/okiabrian123/chatbox-router:latest
```

### **Step 3: Configure Proxy Routes**
Update proxy handler config untuk forward ke microservices:
```toml
# .config/px.toml
ai_service_url="http://chatbox-chat:8080"
user_service_url="http://chatbox-user:8080"
email_service_url="http://chatbox-email:8080"
image_service_url="http://chatbox-image:8080"
tools_service_url="http://chatbox-tools:8080"
```

## Traffic Flow Example

### **User Login Flow**
```
1. User → https://chatpintar.my.id/login
2. Proxy Handler (Port 443) → Route ke /login
3. Proxy Handler → Forward ke chatbox-user:8080/login
4. User Service → Validate credentials
5. User Service → Return session token
6. Proxy Handler → Set secure cookie
7. User → Redirect ke dashboard dengan authenticated session
```

### **AI Request Flow**
```
1. User → https://chatpintar.my.id/api/ai/chat
2. Proxy Handler → Check authentication middleware
3. Proxy Handler → Forward ke chatbox-chat:8080/api/ai/chat
4. AI Service → Process request
5. AI Service → Return response
6. Proxy Handler → Return ke user
```

## Monitoring & Logging

### **Log Locations**
- Proxy Handler: `/var/log/program`
- Microservices: Container logs
- Access logs: Nginx access logs (jika menggunakan nginx)

### **Health Checks**
```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
  interval: 30s
  timeout: 10s
  retries: 3
```

## Summary

Proxy handler adalah **central gateway** yang:
1. **Mengatur semua routing** antara frontend dan microservices
2. **Menangani authentication** dan session management
3. **Melakukan SSL termination** untuk HTTPS
4. **Forward requests** ke microservices yang tepat
5. **Menyediakan security layer** dengan CORS dan middleware

**Deployment Strategy**: Deploy semua microservices terlebih dahulu, kemudian deploy proxy handler sebagai gateway utama yang mengatur semua traffic.
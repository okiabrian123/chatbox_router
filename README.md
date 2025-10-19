# chatbox_router

A Rust-based proxy handler for chatbox applications with secure authentication and routing capabilities.

## Features

- Secure authentication with session management
- SSL/TLS support with Rustls
- Proxy routing for multiple AI services
- Key management and encryption
- CORS support
- Docker containerization

## Prerequisites

- Rust 1.75+ 
- Docker (optional, for containerized deployment)
- OpenSSL development libraries (for local builds)

## Installation

### Local Development

1. Clone the repository:
```bash
git clone https://github.com/okiabrian123/chatbox_router.git
cd chatbox_router
```

2. Install Rust dependencies:
```bash
cargo build
```

3. Create configuration file:
```bash
mkdir -p .config
# Create .config/px.toml with your credentials
```

## Configuration

Create a `.config/px.toml` file with the following format:

```toml
sn="your_service_name"
us="your_username" 
ky="your_encryption_key"
```

## Building

### Local Build
```bash
# Build for current platform
./build-local.sh

# Or using cargo directly
cargo build --release
```

### Cross-compilation for Linux
```bash
# Using cross-compilation tools
./build-linux.sh

# Using Docker (recommended for consistent builds)
./build-docker.sh
```

## Running

### Local Development
```bash
# Run with default settings (127.0.0.1:8080)
./run.sh

# Run with custom IP and port
./run.sh 0.0.0.0 8080

# Or using cargo directly
cargo run -- --ip 0.0.0.0 --port 8080
```

### Docker Deployment

#### Production Build
```bash
# Build production Docker image
docker build -f Dockerfile.prod -t chatbox-router .

# Run container
docker run -p 8080:8080 -p 443:443 \
  -v $(pwd)/.config:/app/.config:ro \
  -v $(pwd)/certs:/app/certs:ro \
  chatbox-router
```

#### Docker Compose
```bash
# Deploy with Docker Compose
docker-compose -f docker-compose.prod.yml up -d
```

## GitHub Actions

This project uses GitHub Actions for automated building and deployment:

- **Test**: Runs tests on Ubuntu
- **Build**: Creates Docker image and pushes to Docker Hub
- **Build Linux Binary**: Cross-compiles for Linux x86_64
- **Deploy**: Creates GitHub releases with deployment packages

### Required GitHub Secrets

Set these secrets in your GitHub repository:

- `DOCKER_USERNAME`: Docker Hub username
- `DOCKER_PASSWORD`: Docker Hub password or access token

## Project Structure

```
chatbox_router/
├── src/
│   ├── main.rs              # Main application entry point
│   ├── handlers.rs          # HTTP request handlers
│   ├── middleware.rs        # Authentication middleware
│   ├── routes.rs            # Route configuration
│   ├── config.rs            # Configuration management
│   ├── key_management.rs    # Encryption key handling
│   ├── ai_handlers.rs       # AI service handlers
│   ├── image_handlers.rs    # Image processing handlers
│   ├── tools.rs             # Utility functions
│   └── user_handlers.rs     # User management
├── .config/                 # Configuration files
├── certs/                   # SSL certificates
├── .github/workflows/       # GitHub Actions
├── Dockerfile.prod          # Production Dockerfile
├── docker-compose.prod.yml  # Docker Compose configuration
├── build-local.sh           # Local build script
├── build-linux.sh           # Linux cross-compilation script
├── build-docker.sh          # Docker build script
└── run.sh                   # Application run script
```

## API Endpoints

The application provides the following endpoints:

- `/static/*` - Static file serving
- `/api/*` - API endpoints (requires authentication)
- Health check endpoint

## Security

- SSL/TLS encryption with Rustls
- Session-based authentication
- CORS configuration
- Key-based encryption for sensitive data

## Deployment

### Production Deployment

1. Build the application:
```bash
./build-linux.sh
```

2. Transfer to server:
```bash
scp chatbox-proxy_handler-x86_64 user@server:/path/to/deployment/
scp -r .config user@server:/path/to/deployment/
scp -r certs user@server:/path/to/deployment/
```

3. Run on server:
```bash
chmod +x chatbox-proxy_handler-x86_64
./chatbox-proxy_handler-x86_64 --ip 0.0.0.0 --port 8080
```

### Docker Deployment

1. Build and push to registry:
```bash
docker build -f Dockerfile.prod -t okiabrian/chatbox-router:latest .
docker push okiabrian/chatbox-router:latest
```

2. Deploy on server:
```bash
docker-compose -f docker-compose.prod.yml up -d
```

## Monitoring

The application includes health checks and logging. Monitor:

- Application logs
- Health check endpoints
- System resource usage

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License.

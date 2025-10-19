# Docker Deployment Guide

## Overview

This guide explains how to deploy the Chatbox Router application using Docker containers on your server.

## Prerequisites

- Docker and Docker Compose installed on your server
- Access to the Docker Hub Registry
- Proper SSH access to your server

## Quick Start

### 1. SSH Connection

Connect to your server using SSH:

```bash
ssh user@your-server-ip
```

### 2. Deploy with Docker Compose

Once connected to your server:

```bash
# Clone the repository
git clone https://github.com/okiabrian123/chatbox_router.git
cd chatbox_router

# Create necessary directories
mkdir -p .config logs certs

# Deploy the application
docker-compose -f docker-compose.prod.yml up -d

# Check the logs
docker-compose -f docker-compose.prod.yml logs -f
```

## Configuration

### Environment Variables

The application uses these environment variables:

- `RUST_LOG`: Logging level (default: info)
- `ENVIRONMENT`: Set to `production` for production deployment

### Volumes

The following volumes are mounted:

- `./.config:/app/.config:ro` - Configuration files (read-only)
- `./certs:/app/certs:ro` - SSL certificates (read-only)
- `./logs:/var/log/program` - Application logs

### Ports

- `8080:8080` - Application HTTP port
- `443:443` - HTTPS port (if using SSL)

## Docker Images

### Registry

The application is published to Docker Hub:

```
docker.io/okiabrian/chatbox-router:latest
```

### Architecture Support

The Docker image is built for:

- `linux/amd64` - Standard x86_64 servers

### Build Process

The image uses a multi-stage build:

1. **Builder Stage**: Uses `rust:1.75-slim` to compile the application
2. **Runtime Stage**: Uses `debian:bookworm-slim` for the final image

## Deployment Options

### Option 1: Using Docker Compose (Recommended)

```bash
# Pull the latest image
docker-compose -f docker-compose.prod.yml pull

# Start the services
docker-compose -f docker-compose.prod.yml up -d

# Update the application
docker-compose -f docker-compose.prod.yml pull && docker-compose -f docker-compose.prod.yml up -d
```

### Option 2: Manual Docker Run

```bash
# Pull the image
docker pull docker.io/okiabrian/chatbox-router:latest

# Run the container
docker run -d \
  --name chatbox-router \
  --restart unless-stopped \
  -p 8080:8080 \
  -p 443:443 \
  -v $(pwd)/.config:/app/.config:ro \
  -v $(pwd)/certs:/app/certs:ro \
  -v $(pwd)/logs:/var/log/program \
  -e RUST_LOG=info \
  -e ENVIRONMENT=production \
  docker.io/okiabrian/chatbox-router:latest
```

## Configuration Files

### Application Configuration

Create the `.config/px.toml` file:

```bash
mkdir -p .config
nano .config/px.toml
```

Add the following content:
```toml
sn="your_service_name"
us="your_username"
ky="your_encryption_key"
```

### SSL Certificates

Place your SSL certificates in the `certs` directory:

```bash
mkdir -p certs
# Copy your certificates here
cp /path/to/certificate.pem certs/mycert.pem
cp /path/to/private.key certs/mykey.pem
```

## Monitoring

### Health Checks

The application includes a health check endpoint:

```bash
# Check health status
curl http://localhost:8080/health
```

### Logs

View application logs:

```bash
# Docker Compose logs
docker-compose -f docker-compose.prod.yml logs -f chatbox-router

# Direct container logs
docker logs -f chatbox-router
```

## SSL/TLS Configuration

### Using Certificates

Place your SSL certificates in the `certs` directory:

```
certs/
├── mycert.pem    # SSL certificate
└── mykey.pem     # Private key
```

### Nginx Reverse Proxy (Optional)

The provided docker-compose.prod.yml includes an optional Nginx reverse proxy:

1. Create an `nginx.conf` file
2. Uncomment the Nginx service in docker-compose.prod.yml
3. Restart the services

Example `nginx.conf`:
```nginx
events {
    worker_connections 1024;
}

http {
    upstream chatbox-router {
        server chatbox-router:8080;
    }

    server {
        listen 443 ssl;
        server_name your-domain.com;

        ssl_certificate /etc/nginx/certs/mycert.pem;
        ssl_certificate_key /etc/nginx/certs/mykey.pem;

        location / {
            proxy_pass http://chatbox-router;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }

    server {
        listen 80;
        server_name your-domain.com;
        return 301 https://$server_name$request_uri;
    }
}
```

## Troubleshooting

### Common Issues

1. **Port Conflicts**: Ensure ports 8080 and 443 are not in use
2. **Permission Issues**: Check file permissions for mounted volumes
3. **Memory Issues**: Monitor memory usage on your server
4. **Configuration Issues**: Verify `.config/px.toml` and SSL certificates

### Debug Commands

```bash
# Check container status
docker ps

# Inspect container
docker inspect chatbox-router

# Execute commands in container
docker exec -it chatbox-router /bin/bash

# Check resource usage
docker stats chatbox-router

# Check container logs
docker logs chatbox-router
```

## Updates

### Automated Updates

The GitHub Actions workflow automatically builds and pushes new images when:

- Code is pushed to the `main` branch
- A new release is created

### Manual Updates

```bash
# Pull latest image
docker pull docker.io/okiabrian/chatbox-router:latest

# Restart with new image
docker-compose -f docker-compose.prod.yml up -d --force-recreate
```

## Security Considerations

1. **Read-only Volumes**: Configuration and certificate files are mounted read-only
2. **Non-root User**: The application runs as a non-root user
3. **Minimal Base Image**: Uses `debian:bookworm-slim` for minimal attack surface
4. **Health Checks**: Automated health monitoring
5. **SSL/TLS**: Always use HTTPS in production
6. **Firewall**: Configure firewall to restrict access

## Performance Optimization

1. **Resource Limits**: Consider setting memory and CPU limits in docker-compose.yml
2. **Logging**: Configure appropriate log levels and rotation
3. **Network**: Use proper network configuration
4. **Load Balancing**: Use multiple instances behind a load balancer for high availability

## Backup and Recovery

### Backup Configuration

```bash
# Backup configuration and certificates
tar -czf chatbox-router-backup.tar.gz .config/ certs/

# Backup logs
tar -czf chatbox-router-logs.tar.gz logs/
```

### Recovery

```bash
# Restore configuration and certificates
tar -xzf chatbox-router-backup.tar.gz

# Restart services
docker-compose -f docker-compose.prod.yml restart
```

## Support

For issues related to:

- **Docker Deployment**: Check this guide and Docker documentation
- **Application Issues**: Check the application logs and GitHub issues
- **Infrastructure**: Check your server configuration and resources

## Advanced Configuration

### Custom Networks

You can create custom Docker networks for better isolation:

```yaml
networks:
  chatbox-network:
    driver: bridge
    ipam:
      config:
        - subnet: 172.20.0.0/16
```

### Resource Limits

Add resource limits to docker-compose.yml:

```yaml
services:
  chatbox-router:
    deploy:
      resources:
        limits:
          cpus: '1.0'
          memory: 512M
        reservations:
          cpus: '0.5'
          memory: 256M
```

### Environment-specific Configurations

Create different compose files for different environments:

- `docker-compose.yml` - Development
- `docker-compose.prod.yml` - Production
- `docker-compose.staging.yml` - Staging

Use them with:
```bash
docker-compose -f docker-compose.prod.yml up -d
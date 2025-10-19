# Setup GitHub Secrets for Deployment

## Required Secrets

Add these secrets to your GitHub repository at:
https://github.com/okiabrian123/chatbox_router/settings/secrets/actions

### Docker Hub Secrets (Required for automated builds):
- **DOCKER_USERNAME**: Docker Hub username (e.g., okiabrian)
- **DOCKER_PASSWORD**: Docker Hub password or access token

### Optional Server Connection Secrets:
- **SERVER_HOST**: Server IP address (e.g., 45.158.126.130)
- **SERVER_USER**: Server username (e.g., root)
- **SERVER_PORT**: SSH port (e.g., 31111)
- **SERVER_SSH_KEY**: Private SSH key content

### Optional Application Secrets:
- **RUST_LOG**: Log level (info, debug, warn, error)
- **ENVIRONMENT**: Environment (development, production)

## How to Add SSH Key (Optional):

1. Generate SSH key pair (if you don't have one):
```bash
ssh-keygen -t rsa -b 4096 -C "github-actions"
```

2. Add public key to server:
```bash
ssh-copy-id -i ~/.ssh/github_actions.pub root@YOUR_SERVER_IP -p YOUR_PORT
```

3. Add private key to GitHub secrets:
```bash
cat ~/.ssh/github_actions
# Copy the entire content and paste as SERVER_SSH_KEY
```

## Manual Setup Commands:

```bash
# On server, create necessary directories
mkdir -p /root/chatbox-router/{.config,certs,logs}

# Set permissions
chmod 755 /root/chatbox-router
chmod 644 /root/chatbox-router/.config/*
chmod 600 /root/chatbox-router/certs/*

# Install Docker on server (if not installed)
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh
systemctl start docker
systemctl enable docker

# Install docker-compose (if not installed)
curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose
```

## Deployment Process:

1. Push to main branch triggers automatic build
2. GitHub Actions will:
   - Build and test the application
   - Build Docker image for amd64
   - Push to Docker Hub
   - Create GitHub release

## Monitoring:

Check deployment status at:
https://github.com/okiabrian123/chatbox_router/actions

## Docker Hub Setup:

1. Create Docker Hub account at https://hub.docker.com/
2. Create access token:
   - Go to Account Settings → Security
   - Click "New Access Token"
   - Give it a name (e.g., "github-actions")
   - Select scopes: Read, Write
   - Copy the token

3. Add secrets to GitHub:
   - DOCKER_USERNAME: Your Docker Hub username
   - DOCKER_PASSWORD: Your access token (not password)

## Configuration Files:

### Application Configuration (.config/px.toml)
```toml
sn="your_service_name"
us="your_username"
ky="your_encryption_key"
```

### SSL Certificates (certs/)
```
certs/
├── mycert.pem    # SSL certificate
└── mykey.pem     # Private key
```

## Security Best Practices:

1. Use access tokens instead of passwords
2. Rotate secrets regularly
3. Use read-only file permissions where possible
4. Use SSH keys instead of passwords for server access
5. Keep your dependencies updated

## Troubleshooting:

### Build Failures:
- Check if Docker Hub credentials are correct
- Verify repository permissions
- Check build logs in Actions tab

### Deployment Issues:
- Verify server connectivity
- Check file permissions
- Review application logs

### Permission Errors:
- Ensure SSH key has proper permissions (600)
- Check directory permissions on server
- Verify Docker daemon is running
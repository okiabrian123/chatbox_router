# Server Deployment Guide

This guide explains how to deploy the chatbox-router application on your Linux server using the releases from GitHub.

## Automated Deployment

The easiest way to deploy the application on your server is using the automated download script:

```bash
# Download and run the deployment script
wget https://github.com/okiabrian123/chatbox_router/releases/latest/download/download-and-run.sh
chmod +x download-and-run.sh
./download-and-run.sh
```

This script will:
1. Download the latest release from GitHub
2. Extract the application files
3. Make the binary executable
4. Start the application

## Manual Deployment

If you prefer to deploy manually:

### 1. Download the Latest Release

```bash
# Find the latest release URL
API_URL="https://api.github.com/repos/okiabrian123/chatbox_router/releases/latest"
DOWNLOAD_URL=$(curl -s $API_URL | grep "browser_download_url.*-release.tar.gz" | cut -d '"' -f 4)

# Download the release package
wget -O release.tar.gz "$DOWNLOAD_URL"
```

### 2. Extract and Setup

```bash
# Extract the files
tar -xzf release.tar.gz

# Find the binary name
BINARY_NAME=$(ls *-x86_64 | head -n 1)

# Make it executable
chmod +x "$BINARY_NAME"

# Start the application
./"$BINARY_NAME"
```

## Running as a Service

To run the application as a system service:

### 1. Create a service file

```bash
sudo nano /etc/systemd/system/chatbox-router.service
```

Add the following content (adjust paths as needed):

```ini
[Unit]
Description=ChatBox Router Service
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/root/chatbox-router
ExecStart=/root/chatbox-router/chatbox-proxy_handler-x86_64
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### 2. Enable and start the service

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable the service to start on boot
sudo systemctl enable chatbox-router

# Start the service
sudo systemctl start chatbox-router

# Check status
sudo systemctl status chatbox-router
```

## Configuration

The application uses configuration files in the `.config/` directory:

- `px.toml` - Main application configuration

Make sure these files are present in the same directory as the binary or in the `.config/` subdirectory.

Example `.config/px.toml`:
```toml
sn="your_service_name"
us="your_username"
ky="your_encryption_key"
```

## SSL Certificates

The application requires SSL certificates for HTTPS. Place them in the `certs/` directory:

```
certs/
├── mycert.pem    # SSL certificate
└── mykey.pem     # Private key
```

## Updating the Application

To update to the latest version:

### Using the automated script

```bash
# Stop the service if running as a service
sudo systemctl stop chatbox-router

# Download and run the latest version
wget https://github.com/okiabrian123/chatbox_router/releases/latest/download/download-and-run.sh
chmod +x download-and-run.sh
./download-and-run.sh

# Start the service again
sudo systemctl start chatbox-router
```

### Manual update

```bash
# Stop the application
sudo systemctl stop chatbox-router
# or kill the process if running manually

# Download and extract the new version
# (follow the manual deployment steps above)

# Start the application again
sudo systemctl start chatbox-router
```

## Troubleshooting

### Application won't start

1. Check if the binary is executable: `ls -la *-x86_64`
2. Check configuration files: make sure `.config/` directory and files exist
3. Check logs: `journalctl -u chatbox-router -f` (if running as service)
4. Check SSL certificates: make sure `certs/` directory exists with valid certificates

### Permission denied errors

1. Make sure the binary is executable: `chmod +x chatbox-proxy_handler-x86_64`
2. Check file permissions: `ls -la`
3. If running as service, check the service user permissions

### Port already in use

1. Check what's using the port: `netstat -tlnp | grep :8080`
2. Kill the process: `kill [PID]`
3. Or change the port using command line arguments: `./chatbox-proxy_handler-x86_64 --ip 0.0.0.0 --port 8081`

## Firewall Configuration

If you have a firewall enabled, make sure to allow the application port:

```bash
# For ufw (Ubuntu)
sudo ufw allow 8080
sudo ufw allow 443

# For firewalld (CentOS/RHEL)
sudo firewall-cmd --permanent --add-port=8080/tcp
sudo firewall-cmd --permanent --add-port=443/tcp
sudo firewall-cmd --reload
```

## Monitoring

To monitor the application:

### Check logs

```bash
# If running as service
journalctl -u chatbox-router -f

# If running manually with output redirection
tail -f /path/to/log/file
```

### Check process

```bash
# Check if the process is running
ps aux | grep chatbox-proxy_handler

# Check network connections
netstat -tlnp | grep :8080
netstat -tlnp | grep :443
```

## Security Considerations

1. Run the application as a non-root user if possible
2. Use a firewall to restrict access to the application port
3. Keep the application updated by regularly checking for new releases
4. Monitor logs for suspicious activity
5. Use valid SSL certificates for HTTPS
6. Regularly rotate encryption keys in configuration

## Command Line Options

The application supports the following command line options:

```bash
./chatbox-proxy_handler-x86_64 --help
```

Common usage:
```bash
# Run with default settings (127.0.0.1:8080)
./chatbox-proxy_handler-x86_64

# Run with custom IP and port
./chatbox-proxy_handler-x86_64 --ip 0.0.0.0 --port 8080
```

## Health Check

The application provides a health check endpoint (if implemented):

```bash
curl http://localhost:8080/health
```

## Performance Tuning

For production deployments, consider:

1. **System Resources**: Ensure adequate CPU and memory
2. **File Descriptors**: Increase ulimit if needed
3. **Network Optimization**: Tune kernel parameters for high traffic
4. **Load Balancing**: Use multiple instances behind a load balancer
5. **Monitoring**: Set up application and system monitoring
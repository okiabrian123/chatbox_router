# Setup Docker Hub Secrets for GitHub Actions

## Problem
GitHub Container Registry (GHCR) might give issues when pushing images. We're using Docker Hub as an alternative.

## Solution: Configure Docker Hub Secrets

### 1. Create Docker Hub Account
If you don't have one, sign up at https://hub.docker.com/

### 2. Create GitHub Secrets
Go to your GitHub repository: https://github.com/okiabrian123/chatbox_router/settings/secrets/actions

Add these secrets:

#### DOCKER_USERNAME
- Your Docker Hub username
- Example: `okiabrian`

#### DOCKER_PASSWORD
- Your Docker Hub password or access token
- **Recommended**: Use Docker Hub Access Token instead of password

### 3. Create Docker Hub Access Token (Recommended)

1. Login to Docker Hub: https://hub.docker.com/
2. Go to Account Settings → Security
3. Click "New Access Token"
4. Give it a descriptive name (e.g., "github-actions-chatbox-router")
5. Select scopes: Read, Write
6. Copy the generated token

### 4. Add Secrets to GitHub

1. Go to: https://github.com/okiabrian123/chatbox_router/settings/secrets/actions
2. Click "New repository secret"
3. Add `DOCKER_USERNAME` with your Docker Hub username
4. Add `DOCKER_PASSWORD` with your Docker Hub password or access token

### 5. Verify Configuration

After adding secrets, the workflow will:
- Use `docker.io` registry (Docker Hub)
- Push to `okiabrian/chatbox-router:latest`
- Authenticate with your Docker Hub credentials

## Alternative: Use GitHub Container Registry with Personal Access Token

If you prefer to keep using GHCR:

1. Create a Personal Access Token (PAT) with `write:packages` scope
2. Add `GHCR_PAT` secret to GitHub
3. Update workflow to use PAT instead of GITHUB_TOKEN

## Current Configuration

The workflow is now configured to use:
- Registry: `docker.io` (Docker Hub)
- Image: `okiabrian/chatbox-router:latest`
- Authentication: Docker Hub username/password

## Next Steps

1. Add the Docker Hub secrets to GitHub
2. Push the updated workflow files
3. Test the build and push

## Troubleshooting

### Common Issues:
- **Authentication failed**: Verify DOCKER_USERNAME and DOCKER_PASSWORD
- **Permission denied**: Ensure Docker Hub account has push permissions
- **Rate limiting**: Docker Hub has rate limits for anonymous users

### Debug Commands:
```bash
# Test Docker Hub login locally
docker login docker.io
# Enter your username and password

# Test push manually
docker tag test-image okiabrian/chatbox-router:test
docker push okiabrian/chatbox-router:test
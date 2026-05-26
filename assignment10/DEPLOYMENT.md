# Docker Deployment Guide

## Overview

This document explains how to deploy and run the Rust backend application using Docker and Docker Compose.

The deployment setup includes:
- Multi-stage Docker builds
- Docker Compose orchestration
- Environment variable configuration
- Containerized API deployment

---

# Prerequisites

Install the following tools before deployment:

| Tool | Purpose |
|---|---|
| Docker | Container engine |
| Docker Compose | Multi-container orchestration |
| Git | Repository management |

---

# Verify Docker Installation

Run:

```bash
docker --version
docker compose version
```

Expected output:

```text
Docker version 27.x.x
Docker Compose version v2.x.x
```

---

# Project Structure

Deployment-related files:

```text
Dockerfile
docker-compose.yml
.dockerignore
.env
DEPLOYMENT.md
```

---

# Environment Variables

The `.env` file contains runtime configuration.

Example:

```text
APP_PORT=3000
RUST_ENV=production
```

---

# Building Docker Image

To build the container image:

```bash
docker build -t rust-backend-api .
```

This process:
1. Downloads Rust dependencies
2. Compiles the Rust application
3. Creates optimized release binaries
4. Builds the runtime container

---

# Running with Docker Compose

Start the application:

```bash
docker compose up --build
```

Docker Compose will:
- Build the image
- Create the container
- Expose ports
- Load environment variables

---

# Accessing the API

After startup, access the API at:

```text
http://localhost:3000
```

Swagger/OpenAPI endpoints may include:

```text
http://localhost:3000/swagger-ui
```

or

```text
http://localhost:3000/docs
```

---

# Stopping Containers

To stop running containers:

```bash
CTRL + C
```

Or run:

```bash
docker compose down
```

---

# Viewing Running Containers

List containers:

```bash
docker ps -a
```

---

# Docker Compose Configuration

The deployment uses the following configuration:

```yaml
services:
  rust-api:
    build: .
    container_name: rust-backend-container
    ports:
      - "3000:3000"
```

---

# Multi-Stage Build Benefits

The Dockerfile uses a multi-stage build process.

Benefits:
- Smaller production images
- Faster deployments
- Improved security
- Reduced unnecessary dependencies

---

# Deployment Workflow

## Step 1
Clone repository:

```bash
git clone https://github.com/YOUR_USERNAME/software-engineering-assignment.git
```

---

## Step 2
Open project:

```bash
cd software-engineering-assignment/assignment10
```

---

## Step 3
Build container:

```bash
docker build -t rust-backend-api .
```

---

## Step 4
Start services:

```bash
docker compose up --build
```

---

# Future Deployment Improvements

Planned enhancements:
- PostgreSQL container integration
- Redis caching container
- Kubernetes deployment
- CI/CD automated deployment
- Cloud hosting support
- Nginx reverse proxy
- HTTPS configuration

---

# Troubleshooting

## Docker daemon not running

Start Docker:

```bash
sudo systemctl start docker
```

---

## Permission denied

Run Docker commands with:

```bash
sudo
```

Or add user to docker group:

```bash
sudo usermod -aG docker $USER
```

---

## Container build failures

Verify:
- Cargo.toml exists
- Rust project builds successfully
- Docker is installed correctly

---

# Conclusion

This deployment setup provides a containerized, portable, and scalable environment for the Rust backend application.

Using Docker improves:
- consistency
- deployment reliability
- portability
- scalability
- DevOps readiness

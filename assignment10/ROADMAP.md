# Project Roadmap

This roadmap outlines planned improvements and future enhancements for the Software Engineering backend system.

---

# Current Features

Implemented features:
- Repository Pattern
- In-Memory Data Storage
- Service Layer Architecture
- REST API with Axum
- Swagger/OpenAPI Documentation
- GitHub Actions CI/CD
- Branch Protection Rules
- Automated Testing

---

# Phase 1 — Database Integration

## PostgreSQL Support
Status: Planned

Goals:
- Replace in-memory repositories
- Add persistent storage
- Improve scalability

Planned technologies:
- PostgreSQL
- SQLx or Diesel ORM

---

## MySQL Support
Status: Planned

Goals:
- Alternative relational database support
- Flexible storage options

---

# Phase 2 — Authentication & Security

## JWT Authentication
Status: Planned

Features:
- Secure login
- Token-based authentication
- Protected endpoints

---

## Role-Based Authorization
Status: Planned

Roles:
- Admin
- Librarian
- User

---

## Password Encryption
Status: Planned

Planned implementation:
- bcrypt password hashing

---

# Phase 3 — Performance Improvements

## Redis Caching
Status: Planned

Goals:
- Faster API responses
- Reduced database load

---

## Async Optimization
Status: Planned

Goals:
- Improve concurrency handling
- Better scalability

---

# Phase 4 — API Expansion

## Additional CRUD Endpoints
Status: Planned

Planned endpoints:
- PUT /api/books
- DELETE /api/books
- GET /api/users
- POST /api/loans

---

## Pagination & Filtering
Status: Planned

Goals:
- Efficient large dataset handling
- Improved API usability

---

# Phase 5 — DevOps & Deployment

## Docker Integration
Status: Planned

Goals:
- Containerized deployment
- Environment consistency

---

## Kubernetes Deployment
Status: Planned

Goals:
- Scalability
- Cloud-native deployment

---

## Cloud Deployment
Status: Planned

Potential platforms:
- AWS
- Azure
- Render
- Railway

---

# Phase 6 — Monitoring & Observability

## Logging System
Status: Planned

Features:
- Structured logs
- Request tracing
- Error tracking

---

## Metrics & Monitoring
Status: Planned

Tools:
- Prometheus
- Grafana

---

# Phase 7 — Frontend Integration

## Web Dashboard
Status: Planned

Features:
- Book management UI
- User management UI
- Loan tracking dashboard

Potential frameworks:
- React
- Next.js

---

# Phase 8 — Advanced Testing

## Code Coverage Reports
Status: Planned

Tools:
- cargo-tarpaulin

---

## End-to-End Testing
Status: Planned

Goals:
- Full API workflow testing
- Integration validation

---

# Long-Term Vision

The long-term goal is to evolve this project into a production-ready, scalable backend system demonstrating:
- clean architecture
- enterprise software engineering practices
- cloud-native deployment
- secure API development
- DevOps automation

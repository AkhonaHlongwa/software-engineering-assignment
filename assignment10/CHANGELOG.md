# CHANGELOG

## Assignment 10 Progress

### Added UML Class Implementations
- User
- Book
- Loan
- Reservation
- Membership
- FinePayment
- Librarian

---

## Implemented Creational Patterns

### Simple Factory
- VehicleFactory
- Car
- Bike
- Truck

### Factory Method
- PaymentProcessor
- CreditCardProcessor
- PayPalProcessor

### Abstract Factory
- GUIFactory
- WindowsButton
- MacOSButton

### Builder
- PizzaBuilder
- Pizza object construction

### Prototype
- Circle prototype cloning
- Rectangle prototype cloning

### Singleton
- Thread-safe DatabaseConnection singleton

---

## Added Unit Tests

### Test Coverage Includes
- Object creation validation
- Prototype cloning
- Singleton consistency
- Builder configuration validation
- Factory object generation

---

## Repository Improvements
- Added Rust project structure
- Added modular source files
- Added test organization
- Added documentation updates

---

# Assignment 11 Progress

## Added Repository Pattern

### Generic Repository Interface
- save()
- find_by_id()
- find_all()
- delete()

### Entity-Specific Repositories
- BookRepository
- UserRepository

---

## Added In-Memory Implementations

### HashMap-Based Storage
- InMemoryBookRepository
- InMemoryUserRepository

### CRUD Support
- Create
- Read
- Update
- Delete

---

## Added Factory Abstraction

### RepositoryFactory
Supports:
- MEMORY repositories
- Future DATABASE repositories

---

## Added Future Database Stub

### DatabaseBookRepository
Prepared for future:
- SQL database support
- External persistence

---

## Added Unit Tests

### CRUD Repository Tests
- Save validation
- Find by ID validation
- Delete validation
- Collection retrieval validation

---

## Added Updated Class Diagram

### Mermaid.js Repository Diagram
Includes:
- Repository interfaces
- In-memory implementations
- Factory abstraction
- Database stub


---

# Assignment 12 Progress

## Added Service Layer

### Implemented Services
- BookService
- UserService
- LoanService

### Business Logic
- Checkout validation
- Loan limit validation
- CRUD service operations

---

## Added REST API

### API Endpoints
- GET /api/books
- POST /api/books

### API Features
- JSON serialization
- Async routing with Axum
- In-memory persistence

---

## Added Swagger/OpenAPI

### Documentation Features
- Swagger UI
- Endpoint schemas
- Request/response examples

---

## Added Tests

### Service Tests
- Loan validation tests

### API Tests
- REST API placeholder integration tests

---

## Added Screenshots

### Evidence Files
- api-books-endpoint.png
- swagger-ui.png

---

## Added Layered Architecture

### Architecture Flow
Repository → Service → API

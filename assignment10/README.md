nano README.md# Assignment 10: From Class Diagrams to Code with Creational Patterns

## Programming Language Choice

Rust was selected because it provides:
- Strong type safety
- Memory safety without garbage collection
- High performance
- Excellent support for modular programming and testing

---

## UML Class Implementations

Implemented all core classes from Assignment 9:

- User
- Book
- Loan
- Reservation
- Membership
- FinePayment
- Librarian

Location:
```text
assignment10/src/
```

---

## Creational Design Patterns

### Implemented Patterns

| Pattern | Purpose |
|---|---|
| Simple Factory | Centralized vehicle object creation |
| Factory Method | Delegated payment processor creation |
| Abstract Factory | GUI component families |
| Builder | Step-by-step Pizza construction |
| Prototype | Efficient object cloning |
| Singleton | Single global database connection |

Location:
```text
assignment10/creational_patterns/
```

---

## Design Decisions

### Builder Pattern
Builder was used for Pizza because pizza objects contain optional ingredients and require flexible construction.

### Singleton Pattern
Singleton ensures only one database connection instance exists globally.

### Prototype Pattern
Prototype reduces expensive initialization by cloning existing objects.

### Factory Patterns
Factory patterns improve flexibility and reduce tight coupling between object creation and business logic.

---

## Unit Testing

Tests validate:
- Correct object creation
- Attribute initialization
- Prototype cloning
- Singleton consistency
- Builder edge cases

Location:
```text
assignment10/tests/
```

---

## Test Evidence

Test execution screenshot:

```text
screenshots/test-results.png
```

---

## CHANGELOG

Project progress tracked in:

```text
CHANGELOG.md
```

---

## Assignment 10 Repository Structure

```text
assignment10/
│
├── src/
├── tests/
├── creational_patterns/
│   ├── simple_factory/
│   ├── factory_method/
│   ├── abstract_factory/
│   ├── builder/
│   ├── prototype/
│   └── singleton/
│
└── screenshots/
```


# Assignment 11 — Repository Pattern and Storage Abstraction

## Repository Pattern

This assignment implements the Repository Pattern to separate business logic from storage implementation details.

The repository layer provides:
- CRUD operations
- Generic interfaces
- Storage abstraction
- Future scalability

---

## Generic Repository Design

A generic repository interface was implemented:

```rust
Repository<T, ID>
```

This approach avoids duplication across entity repositories.

Entity-specific repositories include:
- BookRepository
- UserRepository

Location:
```text
repositories/
```

---

## In-Memory Repository Implementation

Repositories were implemented using Rust HashMap collections.

Benefits:
- Fast testing
- No database dependency
- Simple development workflow
- Easy CRUD validation

Location:
```text
repositories/inmemory/
```

---

## Storage Abstraction Mechanism

The Factory Pattern was used to abstract repository creation.

### Why Factory Pattern?

The factory allows switching storage backends without changing business logic.

Current supported storage:
- MEMORY

Future planned storage:
- DATABASE

Location:
```text
factories/repository_factory.rs
```

---

## Future-Proofing

A future database repository stub was added:

```text
repositories/database/database_book_repository.rs
```

This design allows easy migration to:
- PostgreSQL
- MySQL
- MongoDB
- File-based persistence

without changing application logic.

---

## Updated Repository Diagram

Updated Mermaid.js diagram:

```text
assignment11/class-diagram/repository-class-diagram.md
```

---

## Testing

Unit tests validate:
- Save operation
- Find by ID
- Find all
- Delete operation

Location:
```text
tests/repository_tests.rs
```


# Assignment 12 — Service Layer and REST API

## Overview

This assignment extends the repository architecture by adding:
- Service layer abstraction
- REST API endpoints
- Swagger/OpenAPI documentation
- Business logic validation

The implementation follows a layered architecture:

```text
Repository → Service → API
```

---

## Implemented Entities

Minimum required entities implemented:
- Book
- User
- Loan

---

## Service Layer

Service classes encapsulate business logic and repository access.

### Services Implemented

| Service | Responsibility |
|---|---|
| BookService | Book management and checkout validation |
| UserService | User management |
| LoanService | Loan limit validation |

Location:
```text
services/
```

---

## Business Rules

Implemented validations include:
- Users cannot borrow more than 5 books
- Prevent duplicate book checkout
- Basic CRUD validation

---

## REST API

Implemented REST endpoints:

| Method | Endpoint | Description |
|---|---|---|
| GET | /api/books | Fetch all books |
| POST | /api/books | Create a new book |

Location:
```text
api/
```

---

## API Framework Choice

Axum was selected because it provides:
- Modern async Rust support
- Fast performance
- Clean routing system
- Strong type safety

---

## Swagger/OpenAPI Documentation

Swagger UI available at:

```text
http://127.0.0.1:3000/docs
```

Documentation includes:
- Endpoint descriptions
- Request schemas
- Response schemas
- API testing support

Location:
```text
docs/
```

---

## Testing

### Service Tests
- Loan limit validation
- Business logic verification

### API Tests
- Endpoint testing
- Integration placeholder tests

Location:
```text
tests/
```

---

## Screenshot Evidence

### API Endpoint Screenshot
```text
screenshots/api-books-endpoint.png
```

### Swagger UI Screenshot
```text
screenshots/swagger-ui.png
```

---

## Future Improvements

Planned future enhancements:
- Database persistence
- Authentication & authorization
- Full CRUD endpoints
- Structured error handling
- JWT security
- PostgreSQL integration

# Assignment 13 — CI/CD Pipeline with GitHub Actions

## Overview

This assignment introduces Continuous Integration and Continuous Deployment (CI/CD) practices using GitHub Actions.

The implementation includes:
- Branch protection rules
- Automated testing pipelines
- Pull request validation
- Release artifact generation
- Continuous Integration workflows

---

## CI/CD Pipeline

GitHub Actions automatically performs:

### Continuous Integration
- Build validation
- Automated testing
- Dependency caching
- Pull request verification

### Continuous Deployment
- Release artifact generation
- Automated release builds

Workflow location:

```text
.github/workflows/ci.yml
```

---

## GitHub Actions Workflow

### Trigger Events

The CI/CD pipeline runs automatically on:

| Event | Description |
|---|---|
| Push | Any branch push |
| Pull Request | PRs targeting main |

---

## Automated Workflow Steps

### Build Pipeline
- Checkout repository
- Install Rust toolchain
- Cache Cargo dependencies
- Build project
- Execute tests

### Release Pipeline
- Build optimized release binaries
- Upload build artifacts

---

## Branch Protection Rules

Configured branch protection features:

| Protection | Purpose |
|---|---|
| Pull Request Reviews | Prevent unreviewed merges |
| Required Status Checks | Block failing code |
| Restricted Direct Pushes | Protect main branch |

Documentation:

```text
PROTECTION.md
```

---

## Pull Request Workflow

A failing test was intentionally introduced to demonstrate:
- CI failure detection
- Merge blocking
- Automated PR validation

After fixing the test:
- CI passed successfully
- Merge became allowed

---

## Screenshots

### Branch Protection
```text
screenshots/branch-protection-rules.png
```

### GitHub Actions Success
```text
screenshots/github-actions-tests.png
```

### Release Artifact
```text
screenshots/release-artifact.png
```

### Pull Request Failure
```text
screenshots/pr-blocked-failing-tests.png
```

### Pull Request Success
```text
screenshots/pr-passing-after-fix.png
```

---

## Benefits of CI/CD

Implemented CI/CD provides:
- Faster feedback
- Higher code quality
- Safer deployments
- Automated testing
- Better collaboration
- Reduced manual errors

---

## Future Improvements

Potential future enhancements:
- Docker deployment
- Kubernetes integration
- Cloud deployment automation
- Security scanning
- Code coverage reporting
- Automated releases

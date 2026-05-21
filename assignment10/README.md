# Assignment 10: From Class Diagrams to Code with Creational Patterns

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

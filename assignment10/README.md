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

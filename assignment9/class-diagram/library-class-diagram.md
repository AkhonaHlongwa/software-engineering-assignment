# Library Management System Class Diagram

```mermaid
classDiagram

class User {
    -String userId
    -String name
    -String email
    -String membershipStatus

    +borrowBook()
    +returnBook()
    +reserveBook()
    +payFine()
}

class Book {
    -String bookId
    -String title
    -String ISBN
    -String status

    +checkOut()
    +returnBook()
    +reserve()
}

class Loan {
    -String loanId
    -Date dueDate
    -Date returnDate

    +calculateFine()
    +closeLoan()
}

class Reservation {
    -String reservationId
    -Date reservationDate
    -String status

    +confirmReservation()
    +cancelReservation()
}

class Membership {
    -String membershipId
    -Date expiryDate
    -String status

    +renewMembership()
    +cancelMembership()
}

class FinePayment {
    -String paymentId
    -Double amount
    -String paymentStatus

    +processPayment()
    +validatePayment()
}

class Librarian {
    -String librarianId
    -String username
    -String password

    +approveLoan()
    +generateReport()
}

User "1" --> "0..*" Loan : borrows

Book "1" --> "0..1" Loan : associatedWith

User "1" --> "0..*" Reservation : places

Book "1" --> "0..*" Reservation : reservedFor

User "1" --> "1" Membership : owns

Loan "1" --> "0..1" FinePayment : generates

Librarian "1" --> "0..*" Loan : approves

Librarian "1" --> "0..*" Book : manages
```

# Explanation of Design Decisions

## Core Design

The class diagram models the core business entities of the Library Management System developed in previous assignments.

## Relationships

- A User may borrow many Loans.
- A Book may only be linked to one active Loan at a time.
- Users can place multiple Reservations.
- Membership is directly associated with a User.
- Loans may generate Fine Payments.

## Object-Oriented Principles

The design follows object-oriented principles by:
- Encapsulating attributes and methods within classes.
- Separating responsibilities across entities.
- Using associations to model real-world interactions.

## Multiplicity

Multiplicity was added to show:
- One-to-many relationships.
- Optional relationships.
- Ownership between objects.

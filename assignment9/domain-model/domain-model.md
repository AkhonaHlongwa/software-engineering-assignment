# Domain Model Documentation

## Overview

This domain model represents the Library Management System developed throughout previous assignments. The model identifies the key entities, their attributes, methods, relationships, and business rules.

---

# Core Domain Entities

| Entity | Attributes | Methods | Relationships |
|---|---|---|---|
| User | userId, name, email, membershipStatus | borrowBook(), returnBook(), reserveBook() | Associated with Loan, Reservation |
| Book | bookId, title, ISBN, status | checkOut(), returnBook(), reserve() | Associated with Loan |
| Loan | loanId, dueDate, returnDate | calculateFine(), closeLoan() | Linked to User and Book |
| Reservation | reservationId, reservationDate, status | confirmReservation(), cancelReservation() | Linked to User and Book |
| Membership | membershipId, expiryDate, status | renewMembership(), cancelMembership() | Associated with User |
| FinePayment | paymentId, amount, paymentStatus | processPayment(), validatePayment() | Associated with Loan |
| Librarian | librarianId, username, password | approveLoan(), generateReport() | Manages Loans and Reports |

---

# Entity Descriptions

## User

Represents library members who can borrow, reserve, and return books.

### Responsibilities
- Borrow books
- Return books
- Reserve unavailable books
- Pay fines

---

## Book

Represents library inventory items available for borrowing.

### Responsibilities
- Track availability
- Support reservations
- Update checkout status

---

## Loan

Represents active borrowing transactions between users and books.

### Responsibilities
- Track due dates
- Monitor overdue books
- Calculate fines

---

## Reservation

Represents book reservation requests.

### Responsibilities
- Reserve unavailable books
- Confirm reservations
- Cancel reservations

---

## Membership

Represents user membership details.

### Responsibilities
- Track membership validity
- Renew memberships
- Cancel memberships

---

## FinePayment

Represents overdue fine transactions.

### Responsibilities
- Process payments
- Validate payment status
- Reactivate suspended accounts

---

## Librarian

Represents administrative staff managing the system.

### Responsibilities
- Approve book loans
- Generate reports
- Manage inventory

---

# Business Rules

1. A user may borrow a maximum of 5 books simultaneously.
2. Users with overdue books may have suspended accounts.
3. Books cannot be checked out if already reserved.
4. Reservations expire after 3 days if not collected.
5. Membership renewal is required before borrowing new books.
6. Fine payments must be confirmed before account reactivation.

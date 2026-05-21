# Loan State Transition Diagram

```mermaid
stateDiagram-v2
    [*] --> Requested

    Requested --> Approved : Librarian approves loan

    Approved --> Active : Book issued

    Active --> Overdue : Due date passed

    Active --> Returned : User returns book

    Overdue --> Returned : Book returned late

    Returned --> Closed : Loan completed
```

# Explanation

## Key States

- Requested: User has requested a loan.
- Approved: Loan approved by librarian.
- Active: Book currently borrowed.
- Overdue: Return deadline missed.
- Returned: Book returned by user.
- Closed: Loan process completed.

## Key Transitions

- Loans are approved by librarians.
- Loans become overdue if the return date passes.
- Returned books close the loan process.

## Functional Requirement Mapping

- FR-007: Users can request book loans.
- FR-008: System tracks overdue books.
- FR-009: Users return borrowed books.

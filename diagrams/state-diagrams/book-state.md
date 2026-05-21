# Book State Transition Diagram

```mermaid
stateDiagram-v2
    [*] --> Available

    Available --> Reserved : User reserves book

    Reserved --> CheckedOut : Librarian approves checkout

    CheckedOut --> Returned : User returns book

    Returned --> Available : Book inspected

    Reserved --> Available : Reservation cancelled
```

# Explanation

## Key States

- Available: Book is ready for borrowing.
- Reserved: Book is reserved by a user.
- CheckedOut: Book has been borrowed.
- Returned: Book has been returned to library.

## Functional Requirement Mapping

- FR-001: Users can reserve books.
- FR-002: Librarians approve book checkout.
- FR-003: Users can return books.

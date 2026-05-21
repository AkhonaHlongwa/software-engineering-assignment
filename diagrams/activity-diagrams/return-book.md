# Return Book Activity Diagram

```mermaid
flowchart TD

    A([Start]) --> B[User submits returned book]

    B --> C[Librarian scans book]

    C --> D[System updates loan status]

    D --> E{Book overdue?}

    E -- Yes --> F[Calculate overdue fine]

    F --> G[Update user account]

    G --> H[Send fine notification]

    H --> I([End])

    E -- No --> J[Mark loan as completed]

    J --> I
```

# Explanation

## Workflow Summary

This workflow models the process of returning a borrowed book to the library.

## Key Actions

- Librarian scans returned book.
- System updates loan records.
- Overdue fines are calculated automatically.
- Users receive notifications if fines exist.

## Decisions

- The system checks whether the book is overdue.

## Stakeholder Concerns

- Automatic fine calculation improves efficiency.
- Real-time loan updates improve inventory tracking.
- Notifications improve communication with users.

# Reserve Book Activity Diagram

```mermaid
flowchart TD

    A([Start]) --> B[User searches for book]

    B --> C{Book available?}

    C -- No --> D[User submits reservation request]

    D --> E[System records reservation]

    E --> F[Send reservation confirmation]

    F --> G([End])

    C -- Yes --> H[Suggest direct checkout]

    H --> G
```

# Explanation

## Workflow Summary

This workflow models the process of reserving unavailable books.

## Key Actions

- Users search for books.
- Reservation requests are recorded automatically.
- Confirmation notifications are sent to users.

## Decisions

- The system checks whether the book is already available.

## Stakeholder Concerns

- Reservation tracking improves user experience.
- Automatic notifications improve communication.
- Direct checkout suggestions improve efficiency.

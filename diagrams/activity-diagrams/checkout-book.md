# Checkout Book Activity Diagram

```mermaid
flowchart TD

    A([Start]) --> B[User searches for book]

    B --> C{Book available?}

    C -- Yes --> D[User submits checkout request]

    D --> E[Librarian validates membership]

    E --> F{Membership valid?}

    F -- Yes --> G[Approve checkout]

    G --> H[Update inventory]

    G --> I[Send confirmation notification]

    H --> J([End])

    I --> J

    C -- No --> K[Notify user book unavailable]

    K --> J

    F -- No --> L[Reject checkout request]

    L --> J
```

# Explanation

## Workflow Summary

This workflow models the process of borrowing a book from the library system.

## Key Actions

- Users search for books.
- Librarians validate memberships.
- Inventory updates after approval.
- Notifications are sent automatically.

## Decisions

- The system checks whether the book is available.
- Membership validity determines approval.

## Parallel Actions

- Inventory update and notification sending happen simultaneously.

## Stakeholder Concerns

- Real-time inventory updates improve scalability.
- Membership validation improves security.
- Notifications improve communication with users.

# Cancel Reservation Activity Diagram

```mermaid
flowchart TD

    A([Start]) --> B[User views active reservations]

    B --> C[User selects reservation to cancel]

    C --> D[System validates cancellation request]

    D --> E{Cancellation allowed?}

    E -- Yes --> F[Cancel reservation]

    F --> G[Update reservation records]

    G --> H[Send cancellation confirmation]

    H --> I([End])

    E -- No --> J[Display cancellation restriction]

    J --> I
```

# Explanation

## Workflow Summary

This workflow models the cancellation of book reservations.

## Key Actions

- Users select active reservations.
- System validates cancellation eligibility.
- Reservation records are updated automatically.

## Decisions

- System checks whether cancellation is permitted.

## Stakeholder Concerns

- Validation prevents invalid cancellations.
- Automatic updates improve consistency.
- Notifications improve user communication.

# Pay Fine Activity Diagram

```mermaid
flowchart TD

    A([Start]) --> B[User views outstanding fines]

    B --> C[User selects payment option]

    C --> D[System processes payment]

    D --> E{Payment successful?}

    E -- Yes --> F[Update payment records]

    F --> G[Reactivate user account]

    G --> H[Send payment confirmation]

    H --> I([End])

    E -- No --> J[Display payment failure message]

    J --> C
```

# Explanation

## Workflow Summary

This workflow models how users pay overdue library fines.

## Key Actions

- Users select payment methods.
- System processes fine payments.
- User accounts are reactivated automatically.
- Payment confirmations are sent to users.

## Decisions

- The system checks whether payment was successful.

## Stakeholder Concerns

- Automatic payment validation improves efficiency.
- Account reactivation improves user experience.
- Notifications improve communication and transparency.

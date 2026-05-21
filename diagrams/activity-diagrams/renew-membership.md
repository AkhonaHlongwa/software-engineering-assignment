# Renew Membership Activity Diagram

```mermaid
flowchart TD

    A([Start]) --> B[User logs into system]

    B --> C[User selects membership renewal]

    C --> D[System checks membership status]

    D --> E{Membership eligible for renewal?}

    E -- Yes --> F[User submits renewal payment]

    F --> G[System updates membership expiry date]

    G --> H[Send renewal confirmation]

    H --> I([End])

    E -- No --> J[Display renewal restriction message]

    J --> I
```

# Explanation

## Workflow Summary

This workflow models how users renew library memberships.

## Key Actions

- Users request membership renewal.
- System validates renewal eligibility.
- Membership records are updated automatically.
- Confirmation notifications are sent to users.

## Decisions

- The system checks whether membership qualifies for renewal.

## Stakeholder Concerns

- Automated renewal improves efficiency.
- Eligibility validation improves security.
- Notifications improve communication with users.

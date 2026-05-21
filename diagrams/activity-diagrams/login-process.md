# Login Process Activity Diagram

```mermaid
flowchart TD

    A([Start]) --> B[User enters username and password]

    B --> C[System validates credentials]

    C --> D{Credentials valid?}

    D -- Yes --> E[Grant system access]

    E --> F[Load user dashboard]

    F --> G([End])

    D -- No --> H[Display login error message]

    H --> I[User retries login]

    I --> B
```

# Explanation

## Workflow Summary

This workflow models how users authenticate into the library management system.

## Key Actions

- Users enter authentication credentials.
- System validates login information.
- Successful logins load user dashboard.

## Decisions

- The system checks whether credentials are valid.

## Stakeholder Concerns

- Authentication improves system security.
- Error handling improves usability.
- Dashboard loading improves user experience.

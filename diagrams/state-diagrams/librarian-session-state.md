# Librarian Session State Transition Diagram

```mermaid
stateDiagram-v2
    [*] --> LoggedOut

    LoggedOut --> LoggedIn : Valid credentials entered

    LoggedIn --> Idle : No activity detected

    Idle --> LoggedIn : User activity resumes

    LoggedIn --> LoggedOut : User logs out

    Idle --> LoggedOut : Session timeout
```

# Explanation

## Key States

- LoggedOut: Librarian not authenticated.
- LoggedIn: Librarian actively using system.
- Idle: Session inactive temporarily.

## Key Transitions

- Successful login starts session.
- Inactive sessions become idle.
- Sessions end after logout or timeout.

## Functional Requirement Mapping

- FR-021: Librarians authenticate securely.
- FR-022: Sessions timeout after inactivity.

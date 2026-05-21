# Generate Report Activity Diagram

```mermaid
flowchart TD

    A([Start]) --> B[Librarian selects report type]

    B --> C[System retrieves library data]

    C --> D[System generates report]

    D --> E{Export required?}

    E -- Yes --> F[Export report as PDF]

    F --> G[Save report file]

    G --> H([End])

    E -- No --> I[Display report on screen]

    I --> H
```

# Explanation

## Workflow Summary

This workflow models report generation in the library management system.

## Key Actions

- Librarians choose report types.
- System retrieves and processes data.
- Reports may be exported or displayed directly.

## Decisions

- System checks whether export is requested.

## Stakeholder Concerns

- Reporting improves management oversight.
- Export functionality improves usability.
- Automated reporting improves efficiency.

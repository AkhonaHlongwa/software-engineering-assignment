# Branch Protection Rules

## Overview

Branch protection rules were configured for the `main` branch to improve code quality, enforce testing, and prevent unstable code from being merged directly into production.

---

## Enabled Rules

### 1. Require Pull Request Reviews

At least one reviewer approval is required before merging changes into `main`.

### Why This Matters

- Prevents accidental bugs
- Encourages peer review
- Improves code quality
- Ensures collaboration

---

### 2. Require Status Checks to Pass

GitHub Actions CI tests must pass before pull requests can be merged.

### Why This Matters

- Prevents failing code from reaching production
- Automates quality assurance
- Ensures all tests succeed
- Reduces manual testing effort

---

### 3. Disable Direct Pushes

Direct pushes to `main` are restricted.

### Why This Matters

- Protects production code
- Forces developers to use pull requests
- Maintains commit history integrity
- Enforces CI/CD workflow compliance

---

## CI/CD Integration

The branch protection rules integrate with GitHub Actions workflows:

- Pull requests automatically trigger tests
- Failed tests block merges
- Successful builds allow merging
- Release artifacts generate only after merging into `main`

---

## Screenshot Evidence

```text
screenshots/branch-protection-rules.png
```

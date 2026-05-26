# Contributing Guide

Thank you for contributing to this Software Engineering project.

This repository follows open-source collaboration practices using GitHub workflows, pull requests, and CI/CD pipelines.

---

# Project Overview

This project is a Rust-based backend application implementing:
- Repository Pattern
- Service Layer Architecture
- REST APIs
- Swagger/OpenAPI Documentation
- CI/CD with GitHub Actions

---

# Getting Started

## 1. Clone Repository

```bash
git clone https://github.com/YOUR_USERNAME/software-engineering-assignment.git
```

---

## 2. Open Project

```bash
cd software-engineering-assignment/assignment10
```

---

## 3. Install Rust

Install Rust from:

```text
https://www.rust-lang.org/tools/install
```

Verify installation:

```bash
rustc --version
cargo --version
```

---

## 4. Build Project

```bash
cargo build
```

---

## 5. Run Tests

```bash
cargo test
```

---

## 6. Run Application

```bash
cargo run
```

---

# Contribution Workflow

## Step 1 — Create Branch

Never work directly on `main`.

Create a feature branch:

```bash
git checkout -b feature-name
```

Example:

```bash
git checkout -b add-user-api
```

---

## Step 2 — Make Changes

Implement:
- bug fixes
- enhancements
- documentation updates
- tests

---

## Step 3 — Test Changes

Before committing:

```bash
cargo test
```

All tests must pass.

---

## Step 4 — Commit Changes

Use meaningful commit messages.

Example:

```bash
git commit -m "Added user repository validation"
```

---

## Step 5 — Push Branch

```bash
git push -u origin feature-name
```

---

## Step 6 — Create Pull Request

On GitHub:
- open Pull Request
- describe changes clearly
- wait for CI checks
- request review

---

# Coding Standards

## Rust Guidelines

- Use descriptive variable names
- Keep functions small and focused
- Avoid duplicated logic
- Follow Rust formatting standards

Run formatter:

```bash
cargo fmt
```

---

## Testing Requirements

New features should include:
- unit tests
- integration tests where appropriate

Example:

```bash
cargo test
```

---

# Pull Request Requirements

Pull requests should:
- include clear descriptions
- reference related issues
- pass GitHub Actions checks
- avoid unrelated code changes

---

# Issue Reporting

When opening issues:
- describe the problem clearly
- include reproduction steps
- provide screenshots if needed
- suggest possible solutions if known

---

# Code of Conduct

Contributors should:
- communicate respectfully
- provide constructive feedback
- collaborate professionally

---

# CI/CD Integration

This repository uses GitHub Actions for:
- automated testing
- build validation
- release artifact generation

Failed CI checks will block merges into `main`.

---

# Future Contributions

Potential future improvements:
- PostgreSQL integration
- JWT authentication
- Docker deployment
- Kubernetes support
- Redis caching

Thank you for contributing.

# AGENTS.md — Repo Guidance for Developer and AI Agents

Scope: Entire repository.

Goals
- Maintain a minimal, secure Rust codebase with strong defaults.
- Keep changes focused and reviewable; avoid unrelated edits.

Coding Conventions
- Rust 2021, `forbid(unsafe_code)` across the workspace.
- Prefer small modules and explicit types; no wildcard imports.
- Use `sqlx` with compile-time query checking and parameterized queries only.
- Axum/hyper/rustls for networking; TLS 1.3 only.

Security
- Validate and bound all inputs; enforce strict CSPs.
- No secrets committed; use env vars with secure storage.
- Favor least privilege; avoid broad capabilities.

Repository Hygiene
- Keep CI green (fmt, clippy, build). Fix lints rather than suppress.
- Use PR template and include security considerations for changes.
- Update migrations with clear, idempotent scripts; do not change past migrations.

PR Specification Standard (this project)
- Every PR must include a detailed SPEC in the PR body using the provided template.
- SPEC must contain at minimum: Goals, Architecture Overview, Acceptance Criteria, and at least one mermaid diagram.
- Prefer additional diagrams (sequence/ERD) when flows or schemas are introduced.

Agent Notes
- When editing files, make minimal, surgical changes.
- Do not introduce new dependencies casually; justify security impact.
- Avoid adding license headers or changing licensing unless explicitly requested.

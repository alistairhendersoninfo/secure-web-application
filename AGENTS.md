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
- Sensitive endpoints must not use conventional names (e.g., /login, /admin, /health). Use non-guessable slugs per deployment.
- Return uniform 404s for unknown routes; restrict health/readiness to loopback or authenticated contexts.

Routing & Health
- Sensitive endpoints must not use conventional names (e.g., /login, /admin, /health). Use non-guessable slugs per deployment.
- Return uniform 404s for unknown routes; restrict health/readiness to loopback or authenticated contexts.

Repository Hygiene
- Keep CI green (fmt, clippy, build). Fix lints rather than suppress.
- Use PR template and include security considerations for changes.
- Update migrations with clear, idempotent scripts; do not change past migrations.

Agent Notes
- When editing files, make minimal, surgical changes.
- Do not introduce new dependencies casually; justify security impact.
- Avoid adding license headers or changing licensing unless explicitly requested.
- Always consult `docs/INDEX.md` to locate the authoritative SPEC or PLAN for the area you’re modifying; keep PR bodies aligned with those docs.

Linear Naming Convention
- Use zero-padded numeric prefixes for phases and roadmap tasks to guarantee lexical sorting.
- Phase format: `NN.Phase — <title>` (examples: `00.Phase — Baseline Approvals and Repo Hygiene`, `01.Phase — Core Security Foundations`).
- Roadmap task format: `NN. <title>` (examples: `01. Auth Store ...`, `12. Site Gateway Topology`).
- Never use non-padded variants like `Phase 1` or `1. <title>`.

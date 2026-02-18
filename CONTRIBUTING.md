# Contributing

Thank you for contributing. This project follows a SPEC‑first process. Every PR must include a detailed specification suitable for LLM consumption and human review. See the template in `.github/PULL_REQUEST_TEMPLATE.md` and the standards below.

## PR Specification Standard

Required sections in every PR body:
- Goals — what the change achieves.
- Non‑Goals — what this PR will not address.
- Architecture Overview — components and interactions (include at least one mermaid diagram).
- Detailed Design — protocols, schemas, state machines, error handling.
- Security Posture — authn/z, encryption, input validation, abuse mitigation.
- Operations — deployment, config, rotation/rollout, failure domains.
- Acceptance Criteria — testable outcomes for reviewers.
- Open Questions — items needing input.

Diagram requirements:
- Include at least one mermaid diagram. Prefer sequence diagrams for flows, flowcharts for topology, and ERDs for data models.
- Place diagrams inline in the PR body using a fenced code block with `mermaid`.

Style and content:
- Write in clear, declarative language; avoid code snippets unless necessary to illustrate interfaces.
- If APIs are introduced/changed, document field names, types, and constraints explicitly.
- Call out security assumptions and attack surfaces explicitly.

## Coding Standards
- Rust stable; keep `rustfmt` and `clippy` clean.
- `forbid(unsafe_code)` across the workspace.
- Prefer minimal dependencies; justify additions with security impact.
- Use `sqlx` with compile‑time query checking and parameterized queries only.
- Axum/hyper/rustls for networking; TLS 1.3 only.

## Tests & Migrations
- Keep migrations additive and idempotent; never modify past migrations.
- Provide unit/integration tests where code changes behavior.

## Security
- Avoid storing secrets in repo; use environment variables and secret managers.
- Enforce least privilege and strict input validation.

## Process
- Open a PR early with a SPEC draft to gather feedback.
- Ensure CI is green (fmt, clippy, build, CodeQL, Security Audit, SPEC check).

# Contributing

- Use Rust stable, keep `rustfmt` and `clippy` clean.
- Prefer small, reviewed PRs with clear descriptions and tests.
- Security: avoid introducing unsafe code, prefer minimal dependencies, pin versions.
- Include threat model notes for features that touch authentication, authorization, crypto, or network boundaries.

## Routing & Naming Hardening
- Do not expose sensitive endpoints at conventional paths like `/login`, `/signin`, `/admin`, `/dashboard`, `/health`, or `/status`.
- Use deployment-specific, non-guessable slugs (>= 20 chars base32/62) for sensitive routes (auth entry, admin shell, readiness).
- Normalize responses for unknown paths (404 with identical body/latency) to resist enumeration.
- Keep health/readiness endpoints internal-only (loopback or mTLS + header token) and not under obvious names.

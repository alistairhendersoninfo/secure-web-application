# ADR-002: Alpine 3.22 Over Distroless Debian

## Context

SWAP container images need a minimal runtime base. Two candidates were evaluated:

1. **Distroless Debian** (`gcr.io/distroless/static-debian12`) — Google's minimal images with no shell or package manager
2. **Alpine 3.22** (`alpine:3.22`) — Minimal Linux distribution with musl libc

Both provide small image sizes and reduced attack surface. The key differentiator is the C library: Distroless uses glibc, Alpine uses musl.

## Decision

Use Alpine 3.22 as the runtime base image. Rust binaries are compiled with `RUSTFLAGS='-C target-feature=+crt-static'` to produce fully static musl binaries.

Reasons:

- **musl-static binaries** have zero runtime dependencies — the binary is self-contained
- **Alpine's apk** is available during debugging (not in production images, but useful for ephemeral debug containers)
- **Smaller base image** — Alpine 3.22 is ~7MB vs Distroless static at ~2MB, but the static binary makes the base layer irrelevant for security
- **Better Rust ecosystem support** — `x86_64-unknown-linux-musl` is a tier-2 Rust target with good CI support
- **Consistency** — K3s itself runs on Alpine-based nodes in many deployments

## Consequences

- **Positive**: Fully static binaries with no dynamic linking surprises
- **Positive**: Consistent musl toolchain across build and runtime
- **Negative**: Some C dependencies (e.g., OpenSSL) need musl-compatible builds — mitigated by using `rustls` instead of OpenSSL
- **Negative**: Alpine's musl may have subtle behavioral differences from glibc — not relevant for Rust's standard library

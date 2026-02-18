# Security Policy

Reporting a Vulnerability
- Please report suspected vulnerabilities privately to the maintainers.
- Include a proof-of-concept and impact assessment when possible.
- We will acknowledge receipt within 2 business days and provide regular status updates.

Hardening Principles
- Memory-safe Rust across the workspace, `forbid(unsafe_code)`.
- TLS 1.3 everywhere, client certs for agents.
- Principle of least privilege; minimal dependencies and features.
- Reproducible builds and pinned dependencies.

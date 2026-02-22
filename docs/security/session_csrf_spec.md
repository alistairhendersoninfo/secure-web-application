# SPEC: Sessions, Cookies, and CSRF Protection (SSR)

## Goals
- Define secure session handling for SSR pages and APIs.
- Specify CSRF protections for state‑changing requests.

## Non-Goals
- Client SPA token flows (not default for admin surfaces).

## Architecture Overview
- Sessions: short‑lived, HTTP‑only, SameSite=strict cookies; optional token binding to client cert/device.
- CSRF: synchronizer token in forms + SameSite cookies; double‑submit pattern for APIs.

```mermaid
sequenceDiagram
  participant U as User (Browser)
  participant S as Server
  U->>S: GET /<slug> (login)
  S-->>U: Set-Cookie: session; CSRF cookie + form token
  U->>S: POST /<slug>/action with CSRF token + cookie
  S-->>U: 200 OK (state change)
```

## Detailed Design
- Session cookie
  - Flags: HttpOnly, Secure, SameSite=strict, short TTL (e.g., 20m idle, 12h max), rotation on privilege changes.
  - Optional binding: tie session to client TLS fingerprint and IP (configurable tolerance).
- CSRF token
  - Generated per session; embedded as hidden input in SSR forms; compared with CSRF cookie (double‑submit) and session state.
  - Token rotation on login and sensitive state changes.
- API writes: require CSRF header + cookie and authenticated session; reject missing/invalid pairs.

## Security Posture
- Cookies never accessible to JS; nonces for any allowed scripts; strict CSP.
- Fail‑closed on CSRF validation; uniform error responses.

## Operations
- Session store expiry and cleanup; rolling upgrade without logging out users unnecessarily.

## Acceptance Criteria
- SSR forms include CSRF tokens; server validates tokens and SameSite cookies.
- Sessions have strict cookie flags and rotation rules; optional binding documented.

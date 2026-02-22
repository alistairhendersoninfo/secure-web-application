# SPEC: Observability — Metrics, Health, and Tracing

## Goals
- Define metrics surfaces (Prometheus), health/readiness model, and tracing scope.

## Non-Goals
- Vendor-specific dashboards.

## Architecture Overview
- Controller exposes `/metrics` (internal-only); health/readiness on obfuscated slugs and loopback-only.
- Structured logs with correlation IDs; optional OpenTelemetry tracing.

```mermaid
flowchart LR
  App[Controller] --> Metrics[/metrics (internal)/]
  App --> Health[Obfuscated Health/Ready]
  App --> Trace[Structured Logs / OTel]
```

## Detailed Design
- Metrics: request counts/latency, error codes, gRPC method durations, DB pool stats, queue backpressure.
- Health: component summaries; readiness gates DB connectivity and plugin registry.
- Tracing: sampling config; redact PII; correlation ID propagation.

## Security Posture
- Metrics and health restricted (loopback or mTLS + token); obfuscated paths; CSP remains strict.

## Operations
- Scrape configs; alerting thresholds; SLOs for API latency and error budget.

## Acceptance Criteria
- Metrics list documented; health/readiness semantics defined; tracing fields standardized.

## Open Questions
- Per-tenant metrics segmentation.

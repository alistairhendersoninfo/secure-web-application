# SPEC: Default Rate Limits and Backpressure Thresholds

## Goals
- Establish sensible default rate limits for IPs, agents, sessions, and live log streams.

## Defaults (Initial)
- HTTP/gRPC per IP: 60 req/min burst 120.
- Auth endpoints: 10 req/min per IP; lockout after 5 failed logins per account with exponential backoff.
- Agent gRPC streams: 200 msgs/sec per agent; 256 KiB/message; 2 MiB/sec per agent.
- SSE live tail: 200 events/sec per connection; 1 MiB/sec per connection; 30s idle timeout.
- WebSocket (if enabled): 100 msgs/sec; 512 KiB/message; sliding window 5s.

## Backpressure
- Drop oldest in-memory queue per connection when exceeding window; emit metric and client hint to reduce filters.
- Server degrades to coarser sampling before disconnecting.

## Rationale
- Defaults target medium deployments; all values are configurable per environment and per tenant.

## Acceptance Criteria
- Limits documented with rationale; configuration knobs defined; metrics and alerts for limit hits.

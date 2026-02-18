# Controller–Agent Protocol (v0)

- Transport: HTTP/2 over mTLS (TLS 1.3 only). Client cert required for agent endpoints. Certificates rotate frequently.
- Encoding: gRPC/Protobuf (proto files in `proto/swap/v1`). JSON examples provided for readability.
- Identity: Agent identity bound to client certificate; `agent_id` is a UUID assigned at enrollment.
- Replay/ordering: Each stream uses sequence numbers and monotonic timestamps; server enforces ordering per agent.

## Services

- EnrollmentService.Enroll
  - Input: CSR + optional bootstrap token + AgentHello.
  - Output: Signed agent certificate + CA chain + agent_id.
  - Note: In deployments without bootstrap tokens, enrollment is disabled; use pre-provisioned certs.
- HeartbeatService.Heartbeat
  - Input: agent_id, seq, monotonic_ms. Output: next_interval_ms.
- DesiredStateService.Pull
  - Input: agent_id, last_version. Output: DesiredStateUpdate (version + plugin configs).
- PlanService.Apply
  - Input: agent_id, plugin_id, plan_id, plan_json. Output: result.
- LogService.Stream
  - Client-streaming from agent to controller; includes optional Ed25519 signature per envelope.

## Security Notes

- Agents use outbound-only connections to the controller.
- Certificates are pinned to the controller CA; agents verify the server cert.
- Optional message-level signatures for logs to defend if TLS is terminated by middleboxes.
- Backoff with jitter; idempotent operations and resumable streams.

## Next

- Add SPIFFE/SPIRE integration for workload identities.
- Add TPM-backed key support on agents for CSR signing.

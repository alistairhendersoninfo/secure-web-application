# Plugin ABI (WASM/WASI) — v0 Draft

Goals
- Safe, capability-limited execution for plugins.
- Deterministic inputs/outputs with versioned schemas.
- No ambient network or host filesystem access by default.

Execution
- WASM runtime: wasmtime with WASI preview2.
- Resource limits: CPU time (fuel metering), memory cap (e.g., 64MiB), wall-clock deadline.
- Capabilities: provided per-plugin via hostcalls; default is none.

Exports (from plugin)
- `validate(input_json: string) -> result_json`
  - Validate desired config against schema; return errors/warnings.
- `render(input_json: string) -> artifacts_json`
  - Produce config artifacts (files/templates) for the target tool.
- `plan(current_json: string, desired_json: string) -> plan_json`
  - Compute apply plan (diff, steps, probes, rollbacks).
- `analyze(event_json: string) -> hints_json`
  - Consume a single parsed log/event and emit diagnostics/hints.

Hostcalls (provided by controller/agent)
- `host_log(level: u32, ptr: u32, len: u32)`
- `host_now_mono_ms() -> u64`
- `host_scratch_write(path_ptr: u32, path_len: u32, buf_ptr: u32, buf_len: u32) -> i32`
- `host_scratch_read(path_ptr: u32, path_len: u32, out_ptr: u32, out_len: u32) -> i32`

Validation & Signing
- Plugins are signed (detached signature over WASM SHA-256). Controller verifies signature + policy before load.
- Plugin declares metadata: name, version, category, required capabilities, schema versions.

I/O Format
- JSON canonicalized (UTF-8) for v0; Protobuf/FlatBuffers can be added later.
- Size limits enforced at host boundary.

Error Handling
- Functions return JSON with `{ "ok": bool, "errors": [..], "warnings": [..], "data": {..} }`.

Security Considerations
- No network access by default; no host FS access except scratch dir if enabled.
- Strict timeouts and memory limits; abort on violation.
- Inputs validated by host before passing to plugin.

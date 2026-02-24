# Linear Naming Conventions

Use these conventions for deterministic sorting in Linear.

## Phase Naming

- Format: `NN.Phase — <title>`
- Example: `01.Phase — Core Security Foundations`
- `NN` is always two digits.
- Allowed phase range for current roadmap: `00` to `05`.

## Task Naming

- Format: `NN. <title>`
- Example: `07. Rate Limits and Backpressure Defaults`
- `NN` is always two digits.
- Task order must follow `docs/ROADMAP.md` exactly.

## Lock Rules

- Phase issue and milestone must share the same `NN.Phase` key.
- Do not use variants such as `Phase 1`, `Phase 01`, or `1. <title>`.
- Do not derive sort order from issue creation timestamps.

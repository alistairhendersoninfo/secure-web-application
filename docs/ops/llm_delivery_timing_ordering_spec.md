# LLM Delivery Timing and Ordering Spec

This document defines how roadmap phases/tasks are named, ordered, and scheduled for LLM-led execution.

## Source of Truth

- Roadmap sequencing MUST come from `docs/ROADMAP.md`.
- Do not derive phase/task order from GitHub issue creation order.

## Deterministic Ordering

Use these exact title formats:

- Phase: `NN.Phase — <title>`
  - Example: `01.Phase — Core Security Foundations`
- Task: `NN. <title>`
  - Example: `07. Rate Limits and Backpressure Defaults`

Rules:

- `NN` is always two digits.
- Phase numbers are `00` to `05`.
- Task numbers follow `docs/ROADMAP.md` numbering (`01` to `12`).
- No alternative formats (`Phase 1`, `1.`, `Phase 01`) are allowed.

## Phase-Milestone Lock

Each phase issue MUST match exactly one milestone with the same title prefix:

- `00.Phase ...` issue -> `00.Phase ...` milestone
- `01.Phase ...` issue -> `01.Phase ...` milestone
- etc.

If mismatch occurs, rename and relink to restore one-to-one alignment.

## Date and Time Span Policy

Because this project is LLM-led, default durations are shorter than human-only plans.

- Milestone target dates should be sequential by phase order.
- Phase windows should be documented in each phase issue description as:
  - `WINDOW_START: YYYY-MM-DD`
  - `WINDOW_END: YYYY-MM-DD`

Recommended baseline:

- Phase 00: +1 day from planning date
- Phase 01: +2 days
- Phase 02: +3 days
- Phase 03: +4 days
- Phase 04: +5 days
- Phase 05: +6 days

## Task Duration Expectations

Use these planning classes in task descriptions:

- `ETA_CLASS: LLM_FAST` -> 2-6 hours (implementation + self-check)
- `ETA_CLASS: LLM_PLUS_TEST` -> 4-10 hours (implementation + automated tests)
- `ETA_CLASS: LLM_PLUS_HUMAN_REVIEW` -> 1-2 days
- `ETA_CLASS: LLM_PLUS_HUMAN_QA` -> 2-3 days

Human inspection/testing extends timelines and should be explicit.

## JSON Effort Model (Minutes)

Canonical machine-readable source:

- `docs/ops/llm_delivery_timing_ordering_model.json`

Canonical Linear planning docs:

- `docs/linear/README.md`
- `docs/linear/timing_classification.md`
- `docs/linear/task_template.md`

## Parallel Agent Model

Multiple agents may be assigned per task when scope allows:

- Claude: implementation and refactor pass
- Google plugin workflow: research/spec reconciliation
- Gemini anti-gravity: validation and test pressure

Parallelization guidance:

- 2 agents: typically 25-40% faster than single-agent flow
- 3 agents: typically 35-55% faster, with additional merge/reconciliation overhead

## Required Metadata for Sync

For each phase/task issue, keep these fields in description:

- `SOURCE_DOC: docs/ROADMAP.md`
- `PHASE_KEY: NN`
- `TASK_KEY: NN` (tasks only)
- `WINDOW_START: YYYY-MM-DD`
- `WINDOW_END: YYYY-MM-DD`
- `ETA_CLASS: <class>`

These fields make syncing deterministic and auditable.

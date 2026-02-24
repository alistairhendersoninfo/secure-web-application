# GUIDE: Using Multi-LLM GUI Test Orchestration

## Purpose

This guide explains how to use the GUI orchestration workflow in daily delivery work.

It is the practical companion to:

- `docs/ops/gui_llm_test_orchestration_spec.md`
- `.claude/skills/gui-dss-orchestration/SKILL.md`

## What to Use

- Task Card template: `docs/ops/templates/gui_task_card_template.yaml`
- DSS template: `docs/ops/templates/gui_dss_report_template.yaml`
- Skill: `.claude/skills/gui-dss-orchestration/SKILL.md`

## Standard Day-to-Day Flow

1. Create a Task Card
   - Copy `gui_task_card_template.yaml`.
   - Fill scope, required task, deterministic procedure, expected outcome, and acceptance checks.
2. Assign the first tool owner
   - Set `OWNER_TOOL`.
   - Set `STATUS: IN_PROGRESS`.
3. Execute the run
   - Owner follows only the documented procedure.
   - Capture evidence required by the card.
4. Publish DSS
   - Copy `gui_dss_report_template.yaml`.
   - Fill `TASK_REQUIRED`, `EXPECTED_OUTCOME`, `ACTUAL_OUTCOME`, `DEVIATIONS`, and evidence paths.
5. Handoff or close
   - Set `NEXT_ACTION` with owner/action/due.
   - Set `NEXT_TOOL` in Task Card.
   - If all acceptance checks pass, close with `STATUS: CLOSED`.

## Ownership Model

Use one active owner at a time:

- Antigravity: realistic UI path execution and interaction evidence capture.
- Claude Codebook: technical reconciliation and implementation-alignment checks.
- Claude Google plugin: framework/browser behavior cross-check and edge-case validation.

## Status Rules (Enforced)

- `FAILED` or `BLOCKED` must include a concrete `NEXT_ACTION`.
- Any expected-vs-actual mismatch must populate `DEVIATIONS`.
- `CLOSED` is allowed only when acceptance checks are resolved.
- Keep `TASK_ID` stable across all handoffs and revisions.

## Folder Convention

For each GUI task, store artifacts under one folder:

```text
artifacts/gui/<task-id>/
  task-card.yaml
  dss/
    2026-02-24T101500Z-antigravity.yaml
    2026-02-24T111000Z-claude-codebook.yaml
  screenshots/
  logs/
  traces/
```

This keeps evidence and history auditable.

## Prompting Pattern for LLM-Driven Runs

Use this prompt structure when invoking an LLM tool:

```text
Use gui-dss-orchestration.
Task card path: <path>
DSS output path: <path>
Act as OWNER_TOOL=<tool>.
Follow PROCEDURE exactly.
Return a completed DSS including TASK_REQUIRED, EXPECTED_OUTCOME, ACTUAL_OUTCOME, DEVIATIONS, and NEXT_ACTION.
```

## Copy/Paste Operational Checklist

```text
GUI Orchestration Checklist
- [ ] Task Card created from template
- [ ] TASK_REQUIRED filled
- [ ] EXPECTED_OUTCOME filled and testable
- [ ] Acceptance checks binary
- [ ] Owner assigned and status IN_PROGRESS
- [ ] Evidence captured to task artifact folder
- [ ] DSS published with EXPECTED vs ACTUAL
- [ ] DEVIATIONS filled if mismatch exists
- [ ] NEXT_ACTION set for handoff or fix
- [ ] Task closed only with resolved checks
```

## Example Run Update

```text
[DSS] TASK_ID=GUI-login-flow-01 TOOL=antigravity STATUS=FAILED
Required: Validate timeout warning modal before forced sign-out.
Method: Logged in as test user, idled to threshold, resumed interaction.
Expected: Warning modal appears before sign-out.
Actual: Immediate sign-out; no warning modal.
Deviation: Missing warning modal behavior.
Evidence: artifacts/gui/GUI-login-flow-01/screenshots/timeout.png, artifacts/gui/GUI-login-flow-01/logs/console.txt
Next: claude-codebook to inspect modal trigger path and propose code fix by 2026-02-25T12:00:00Z.
```

## Common Mistakes to Avoid

- Writing subjective acceptance checks (must be pass/fail).
- Closing tasks without a final DSS.
- Changing `TASK_ID` during handoffs.
- Omitting evidence references in DSS.
- Omitting `NEXT_ACTION` on blocked/failed runs.

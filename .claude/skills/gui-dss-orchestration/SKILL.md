---
name: gui-dss-orchestration
description: Create and execute GUI test Task Cards and DSS reports for Antigravity, Claude Codebook, and Claude Google plugin. Use when the user asks for GUI test coordination, cross-LLM handoffs, expected-vs-actual reporting, or standardized test run updates.
argument-hint: "--task-id <GUI-id> --owner-tool <tool> [--status <status>]"
disable-model-invocation: false
allowed-tools: Bash, Read
---

# GUI DSS Orchestration

Use this workflow to run GUI tasks with a consistent handoff and reporting method.

## Source of Truth

- `docs/ops/gui_llm_test_orchestration_spec.md`
- `docs/ops/templates/gui_task_card_template.yaml`
- `docs/ops/templates/gui_dss_report_template.yaml`

When generating output, always preserve these fields:

- `TASK_ID`
- `TASK_REQUIRED`
- `EXPECTED_OUTCOME`
- `ACTUAL_OUTCOME`

## Trigger Conditions

Apply this skill when the request mentions:

- GUI testing across multiple LLM tools
- Antigravity, Claude Codebook, Claude Google plugin handoffs
- expected vs actual result reporting
- DSS, task cards, or standard operating workflow

## Workflow

1. Identify target scope:
   - screen/module
   - environment
   - build id or commit
2. Create or update a Task Card from `gui_task_card_template.yaml`.
3. Execute or document run steps in deterministic order.
4. Create a DSS report from `gui_dss_report_template.yaml`.
5. Set `NEXT_ACTION` with explicit owner/action/due.
6. If all acceptance checks pass, set status to `CLOSED`; otherwise keep `FAILED` or `BLOCKED`.

## Output Contract

### Task Card rules

- Keep `TASK_ID` stable across revisions.
- Keep acceptance checks binary and testable.
- Do not leave `TASK_REQUIRED` or `EXPECTED_OUTCOME` blank.

### DSS rules

- Always include `TASK_REQUIRED`, `EXPECTED_OUTCOME`, `ACTUAL_OUTCOME`.
- If mismatch exists, add at least one `DEVIATIONS` entry.
- For `FAILED` or `BLOCKED`, `NEXT_ACTION` must be concrete.
- `CLOSED` requires acceptance checks resolved.

## Compact Chat Update Format

Use this plain-text format for progress updates:

```text
[DSS] TASK_ID=<id> TOOL=<tool> STATUS=<status>
Required: <task required>
Method: <how executed>
Expected: <expected outcome>
Actual: <actual outcome>
Deviation: <none|short mismatch statement>
Evidence: <artifact refs>
Next: <owner + action + due>
```

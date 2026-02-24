# SPEC: Multi-LLM GUI Test Orchestration and DSS Reporting

## Goal

Define one standard method for coordinating GUI testing across:

- Antigravity
- Claude Codebook
- Claude Google plugin

This method standardizes:

- what task is required,
- how each tool should execute the task,
- expected and actual outcomes, and
- how each tool reports back via DSS.

## Terms

- DSS: `Delivery Status Snapshot` (the required report payload after each step).
- Task Card: the canonical handoff object that tells the next tool what to do.
- Run: a single execution attempt by one tool against one Task Card.

## Templates (Canonical)

- Task Card template: `docs/ops/templates/gui_task_card_template.yaml`
- DSS report template: `docs/ops/templates/gui_dss_report_template.yaml`

## Operating Model

Use a baton-pass workflow with one active owner at a time:

1. Plan and split GUI work into numbered Task Cards.
2. Assign one tool as active owner.
3. Active owner executes and publishes a DSS.
4. Next owner picks up the same Task Card revision (or a new revision if changed).
5. Final owner publishes closure DSS with pass/fail and evidence index.

Status values:

- `QUEUED`
- `IN_PROGRESS`
- `BLOCKED`
- `FAILED`
- `PASSED`
- `CLOSED`

## Standard Task Card (Required Fields)

Every GUI task must be created in this structure:

```yaml
TASK_ID: GUI-<feature>-<nn>
TASK_TITLE: <short action-oriented title>
SCOPE:
  area: <screen/module>
  environment: <local/staging/prod-like>
  build: <commit sha or build id>
OWNER_TOOL: <antigravity|claude-codebook|claude-google-plugin>
NEXT_TOOL: <antigravity|claude-codebook|claude-google-plugin|none>
PRECONDITIONS:
  - <state that must already exist>
PROCEDURE:
  - <step 1>
  - <step 2>
EXPECTED_OUTCOME:
  - <observable expected behavior>
EVIDENCE_REQUIREMENTS:
  - screenshot
  - console-log-snippet
  - network-observation
ACCEPTANCE_CHECKS:
  - <binary pass/fail check>
  - <binary pass/fail check>
```

Rules:

- `TASK_ID` must be stable across all handoffs.
- `PROCEDURE` must be deterministic and reproducible.
- `EXPECTED_OUTCOME` must be observable and testable.
- `ACCEPTANCE_CHECKS` must be binary (pass/fail), not subjective.

## DSS Report Contract (Required After Every Run)

Each run must publish one DSS payload:

```yaml
DSS_VERSION: 1
TASK_ID: GUI-<feature>-<nn>
RUN_ID: <uuid-or-timestamp>
TOOL: <antigravity|claude-codebook|claude-google-plugin>
STATUS: <IN_PROGRESS|BLOCKED|FAILED|PASSED|CLOSED>
START_TIME: <ISO-8601>
END_TIME: <ISO-8601>
TASK_REQUIRED: <what was requested>
METHOD_USED:
  - <how task was executed>
EXPECTED_OUTCOME:
  - <copied from Task Card>
ACTUAL_OUTCOME:
  - <what really happened>
DEVIATIONS:
  - <expected vs actual mismatch, if any>
EVIDENCE_INDEX:
  screenshots:
    - <path-or-url>
  logs:
    - <path-or-url>
  traces:
    - <path-or-url>
NEXT_ACTION:
  owner: <tool-or-human>
  action: <specific next step>
  due: <ISO-8601 or none>
```

Rules:

- `TASK_REQUIRED`, `EXPECTED_OUTCOME`, and `ACTUAL_OUTCOME` are mandatory.
- If there is any mismatch, `DEVIATIONS` must be non-empty.
- `FAILED` or `BLOCKED` must always include a concrete `NEXT_ACTION`.
- `CLOSED` requires all `ACCEPTANCE_CHECKS` resolved.

## Tool Role Guidance

Use these default responsibilities unless explicitly overridden:

- Antigravity
  - Primary UI traversal and behavior validation under realistic user paths.
  - Capture interaction evidence (screenshots, sequence of actions).
- Claude Codebook
  - Convert findings into precise technical deltas and reproducible checks.
  - Validate whether implementation matches expected behavior contract.
- Claude Google plugin
  - Cross-check known framework/browser behavior and edge-case expectations.
  - Validate external references and compatibility assumptions.

## Standard Workflow

1. Author Task Card
   - Define required task, method, expected outcome, and binary checks.
2. Claim Ownership
   - Set `OWNER_TOOL` and mark `IN_PROGRESS`.
3. Execute
   - Run only documented procedure steps; note any additional diagnostic steps.
4. Publish DSS
   - Submit complete DSS with evidence links/paths.
5. Handoff
   - Assign `NEXT_TOOL` with explicit next action.
6. Resolve
   - Final owner marks `CLOSED` or escalates with `FAILED` and action plan.

## Reporting Format for Chat/Issue Updates

When posting updates, use this compact report:

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

## Example

```text
[DSS] TASK_ID=GUI-login-flow-01 TOOL=antigravity STATUS=FAILED
Required: Validate session timeout warning modal appears after idle threshold.
Method: Logged in as test user, idled for configured timeout, resumed interaction.
Expected: Warning modal appears before forced sign-out.
Actual: Session is terminated directly without warning modal.
Deviation: Missing pre-expiry warning behavior.
Evidence: screenshots/sess-timeout-01.png, logs/browser-console-22.txt
Next: claude-codebook to inspect modal trigger path and propose fix by 2026-02-25T12:00:00Z.
```

## Governance

- Store Task Cards and DSS reports with project artifacts (PR description, issue thread, or tracked run logs).
- Do not close GUI tasks without a closure DSS.
- Any change to expected behavior requires a new Task Card revision with a changelog note.

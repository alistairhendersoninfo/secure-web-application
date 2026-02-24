# Linear Task Template

Use this template when creating roadmap tasks in Linear.

## Template

```md
## Context

<task goal and scope>

## References

- SOURCE_DOC: docs/ROADMAP.md
- Related specs/plans: <doc paths>

## Metadata

SOURCE_DOC: docs/ROADMAP.md
PHASE_KEY: <NN>
TASK_KEY: <NN>
DIFFICULTY_TIER: <XS|S|M|L|XL>
DELIVERY_MODE: <HUMAN_ONLY|LLM_ONLY|LLM_PLUS_PERSON>
QA_LEVEL: <none|light|standard|deep>
GUI_TESTING: <enabled|disabled>
COORDINATION_MODE: <single_agent|multi_agent|multi_agent_with_human_gate>
ESTIMATED_MINUTES: <number>
WINDOW_START: <YYYY-MM-DD>
WINDOW_END: <YYYY-MM-DD>
ETA_CLASS: <LLM_FAST|LLM_PLUS_TEST|LLM_PLUS_HUMAN_REVIEW|LLM_PLUS_HUMAN_QA>

## Deliverables

- <deliverable 1>
- <deliverable 2>

## Task Timing Table

| Classification | Time Required (min) | Total Time (min) |
| --- | ---: | ---: |
| Base (difficulty) | <base_human_minutes> | <base_human_minutes> |
| Delivery mode applied | <base_human_minutes * delivery_multiplier> | <running_total> |
| QA overhead | <addon_minutes> | <running_total> |
| GUI overhead | <addon_minutes> | <running_total> |
| Coordination overhead | <addon_minutes> | <running_total> |
| Final estimate | - | <ESTIMATED_MINUTES> |
```

## Required Placement

- Keep `Task Timing Table` at the end of each task description.
- Keep metadata block present for deterministic sync updates.

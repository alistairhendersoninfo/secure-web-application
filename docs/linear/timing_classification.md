# Linear Task Classification and Timing

This document defines how to classify tasks and calculate planned duration for Linear.

## Inputs

- Difficulty tier: `XS`, `S`, `M`, `L`, `XL`
- Delivery mode: `HUMAN_ONLY`, `LLM_ONLY`, `LLM_PLUS_PERSON`
- QA level: `none`, `light`, `standard`, `deep`
- GUI testing: `enabled`, `disabled`
- Coordination mode: `single_agent`, `multi_agent`, `multi_agent_with_human_gate`

## Canonical Calculator

Use the canonical model file:

- `docs/ops/llm_delivery_timing_ordering_model.json`

Apply formula:

- `estimate_minutes = round(base_human_minutes * delivery_multiplier * (1 + qa_addon + gui_addon + coordination_addon))`

## Date and Duration Transfer Policy

After calculating `estimate_minutes`, write all of the following to the task:

- `ESTIMATED_MINUTES` metadata field in the issue body
- Linear `startDate`
- Linear `dueDate`
- Linear `duration` (minutes or the equivalent duration field used by your sync flow)

Use these conversion rules for planning:

- If `estimate_minutes` <= 480: start and end on same day.
- If `estimate_minutes` > 480: split across consecutive working days.
- Round end times to the nearest 15 minutes.
- If task depends on another task, set `startDate` to the next working slot after the dependency end.

## Required Task Timing Table

Every task issue description must end with this table shape:

| Classification | Time Required (min) | Total Time (min) |
| --- | ---: | ---: |
| Base (difficulty) | `<base_human_minutes>` | `<base_human_minutes>` |
| Delivery mode applied | `<base_human_minutes * delivery_multiplier>` | `<running_total>` |
| QA overhead | `<addon_minutes>` | `<running_total>` |
| GUI overhead | `<addon_minutes>` | `<running_total>` |
| Coordination overhead | `<addon_minutes>` | `<running_total>` |
| Final estimate | `-` | `<ESTIMATED_MINUTES>` |

# Linear Field Mapping

This document maps markdown task metadata to Linear task fields.

## Metadata in Issue Description

Keep these metadata lines in each task description:

- `SOURCE_DOC: docs/ROADMAP.md`
- `PHASE_KEY: NN`
- `TASK_KEY: NN`
- `DIFFICULTY_TIER: XS|S|M|L|XL`
- `DELIVERY_MODE: HUMAN_ONLY|LLM_ONLY|LLM_PLUS_PERSON`
- `QA_LEVEL: none|light|standard|deep`
- `GUI_TESTING: enabled|disabled`
- `COORDINATION_MODE: single_agent|multi_agent|multi_agent_with_human_gate`
- `ESTIMATED_MINUTES: <number>`
- `WINDOW_START: YYYY-MM-DD`
- `WINDOW_END: YYYY-MM-DD`
- `ETA_CLASS: <class>`

## Linear Fields

- `title`: from naming conventions in `docs/linear/naming_conventions.md`
- `project`: `SWAP Roadmap` (or project selected for the sync run)
- `parent`: phase issue (`NN.Phase`)
- `milestone`: matching `NN.Phase` milestone
- `startDate`: from timing calculation
- `dueDate`: from timing calculation
- `labels`: include phase label and domain labels when defined

## Sync Requirement

When an estimate changes, update both:

- issue metadata lines (`ESTIMATED_MINUTES`, `WINDOW_START`, `WINDOW_END`)
- Linear task date fields (`startDate`, `dueDate`, duration equivalent)

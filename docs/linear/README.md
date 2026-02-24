# Linear Planning Docs

Use this section as the single documentation area for Linear planning and sync conventions.

## Documents

- Naming rules: `docs/linear/naming_conventions.md`
- Task classification and timing policy: `docs/linear/timing_classification.md`
- Linear field mapping (how markdown fields map to Linear fields): `docs/linear/field_mapping.md`
- Task issue template (includes required timing table): `docs/linear/task_template.md`

## Canonical Model Data

- Machine-readable effort model: `docs/ops/llm_delivery_timing_ordering_model.json`

## Usage Rule

When creating or updating Linear issues:

1. Follow naming from `naming_conventions.md`.
2. Compute estimates with `timing_classification.md` + the JSON model.
3. Populate issue metadata and Linear fields from `field_mapping.md`.
4. Include the timing table from `task_template.md` at the end of each task issue description.

---
name: sync-github-linear
description: Sync GitHub issues and docs markdown to Linear project state using MCP. Use when user asks to keep GitHub and Linear in sync, mirror open/closed status, or refresh roadmap/docs tasks.
argument-hint: "--owner <owner> --repo <repo> [--project \"SWAP Roadmap\"]"
disable-model-invocation: false
allowed-tools: Read, Glob, CallMcpTool
---

# Sync GitHub and Linear (MCP)

Use this workflow to keep GitHub issues and Linear project items synchronized.

## Scope
- `docs/ROADMAP.md` -> Linear phase/task hierarchy (primary source of order)
- `docs/**/*.md` -> Linear docs index (refresh when file list changes)
- Optional GitHub issue state mirroring when explicitly requested

## Canonical Mapping
- **GitHub issue identity marker** in Linear issue description:
  - `SYNC_SOURCE: github:<owner>/<repo>#<number>`
- **Docs file identity marker** in Linear docs index:
  - file path list + GitHub blob links

Always preserve these markers so updates are idempotent.

## Ordering and Naming (Mandatory)
- Phase title format: `NN.Phase — <title>`
- Task title format: `NN. <title>`
- Phase and milestone must share the same `NN.Phase` key.
- Task numbering must follow `docs/ROADMAP.md` order exactly.

## Status Mapping
- GitHub `OPEN` -> Linear state `Backlog` (or `In Progress` if already active)
- GitHub `CLOSED` -> Linear state `Done`

## Workflow

1. **Load source state**
   - `user-github/list_issues` for `OPEN` and `CLOSED` with pagination as needed.
   - `Glob` for `docs/**/*.md`.
   - `Read` `docs/ROADMAP.md`.

2. **Ensure target Linear project exists**
   - `plugin-linear-linear/list_projects` (query by name).
   - If missing, create with `save_project`.

3. **Ensure phase parents exist**
   - Parse phases from `docs/ROADMAP.md` (not from GitHub issue ordering).
   - If missing, `create_issue`.
   - If present, `update_issue` title/description/state.
   - Ensure a matching milestone exists for each phase (`NN.Phase`).

4. **Ensure roadmap tasks exist and are linked**
   - Parse numbered tasks in `docs/ROADMAP.md`.
   - Create/update each as a child issue (`parentId` = phase issue id).
   - Keep task descriptions linked to relevant spec/plan paths.
   - Ensure task titles stay `NN. <title>` with two-digit numbering.

5. **Refresh docs index document**
   - Build markdown list of all docs paths with blob links.
   - Create or update a Linear document titled `Docs Mirror Index (docs/)`.

6. **State reconciliation**
   - For each mapped GitHub issue:
     - OPEN -> `update_issue state=Backlog` (unless actively in progress)
     - CLOSED -> `update_issue state=Done`

7. **Apply scheduling metadata**
   - Set milestone target dates sequentially by phase number.
   - Add issue metadata fields in descriptions:
     - `SOURCE_DOC: docs/ROADMAP.md`
     - `PHASE_KEY`, `TASK_KEY`
     - `WINDOW_START`, `WINDOW_END`
     - `ETA_CLASS`

8. **Report**
   - Return counts:
     - created/updated/closed Linear issues
     - created/updated documents
     - unmatched/missing mapping entries

## Safety Rules
- Never delete Linear issues/documents automatically.
- Never change unrelated Linear projects.
- Keep sync markers in descriptions.
- Prefer updates over duplicates (idempotent behavior).
- Do not reorder by creation time; always enforce roadmap numbering.

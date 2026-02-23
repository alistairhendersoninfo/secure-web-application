---
name: create-pr
description: Create a new feature branch and draft PR aligned with SPEC-first process
argument-hint: [feature-key]
disable-model-invocation: false
allowed-tools: Bash, Read, AskUserQuestion
---

# Create PR Workflow (SPEC-first)

You are setting up a new feature branch and draft PR for this project. All PRs must include a SPEC body and at least one mermaid diagram.

## Step 1: Gather information
Use AskUserQuestion to collect:
- Area: one of [auth-store, ssr-routes, events-sse, logging-ui, plugins, ops, security]
- Title: one-line PR title (Conventional PR title recommended)
- SPEC link(s): which doc(s) in docs/ this implements
- Acceptance: brief acceptance criteria bullets

## Step 2: Determine branch name
- `feat/<area>-<feature-key>` for features
- `fix/<area>-<feature-key>` for fixes
- `chore/<area>-<feature-key>` for infra

## Step 3: Create branch and draft PR
Build a PR body that includes:
- Summary
- Linked SPEC(s)
- Architecture Overview (mermaid)
- Acceptance Criteria
- Test Plan (if applicable)

Commands:
```bash
git fetch origin main && git checkout -b <branch> origin/main
gh pr create --title "<conventional title>" --body-file - <<'BODY'
## Summary
<summary>

## Linked SPECs
<paths>

## Architecture Overview
```mermaid
flowchart LR
  A --> B
```

## Acceptance Criteria
- <criteria>

## Test Plan
- <tests>
BODY
```

## Step 4: Summary
Print branch name and PR URL.


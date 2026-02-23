---
name: sync-linear-doc
description: Create/update a Linear issue from a repo markdown doc to keep human-readable specs in sync
argument-hint: "--file <path> [--project <name>] [--phase <Phase heading>]"
disable-model-invocation: false
allowed-tools: Bash, Read
---

# Sync Linear Doc

This skill creates a Linear issue from a markdown doc in `docs/` so the spec is visible in Linear and stays in sync.

## Env
- `LINEAR_API_KEY`: Linear API key
- `LINEAR_TEAM_ID`: Team ID for issues
- `LINEAR_PROJECT_NAME` (optional): defaults to `SWAP Roadmap`

## Usage
```bash
python3 tools/linear/sync_doc.py --file docs/security/auth_store_spec.md --project "SWAP Roadmap" --phase "Phase 1 — Core Security Foundations"
```

This creates a Linear issue with the doc’s title and full body, adds it to the project if present, and applies a phase label if found.

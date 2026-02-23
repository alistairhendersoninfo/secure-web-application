# Linear Setup and Sync Guide

This repo includes scripts to create a Linear project, add Phase trackers and tasks from the Roadmap, and keep docs and Linear issues in sync.

Prerequisites
- Linear API key with write access
- Team ID where issues should be created

Get your Team ID
- Run the GraphQL query below in https://api.linear.app/graphql with the `Authorization: Bearer <LINEAR_API_KEY>` header:
```
query { viewer { teams(first: 50) { nodes { id name key } } } }
```
- Copy the `id` for your target team.

Env vars
- LINEAR_API_KEY: your API key
- LINEAR_TEAM_ID: team id to create issues under
- (optional) LINEAR_PROJECT_NAME: defaults to "SWAP Roadmap"

Bootstrap (create project, phases, tasks)
- Install deps: `python3 -m pip install requests pyyaml`
- Dry run: `LINEAR_API_KEY=… LINEAR_TEAM_ID=… DRY_RUN=1 python3 tools/linear/bootstrap_linear.py`
- Apply: `LINEAR_API_KEY=… LINEAR_TEAM_ID=… python3 tools/linear/bootstrap_linear.py`
  - Creates/uses a Project named $LINEAR_PROJECT_NAME
  - Creates Phase tracker issues (one per Phase in Roadmap)
  - Creates child tasks for each Deliverable under the Phase

Sync a doc with a Linear issue
- Usage: `LINEAR_API_KEY=… LINEAR_TEAM_ID=… python3 tools/linear/sync_doc.py --file docs/security/auth_store_spec.md --project "SWAP Roadmap" --phase "Phase 1 — Core Security Foundations"`
- The script creates (or updates) a Linear issue with the doc content; stores the Linear ID in the doc frontmatter for future updates.

Notes
- All script output includes IDs created/updated; copy them for future mapping if desired.

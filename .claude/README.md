# Claude Code Skills for SWAP

This repository includes Claude Code skills and helper scripts to streamline PR workflows.

Skills (callable in Claude Code)
- approve-pr — admin self‑review gate and optional merge
- merge-pr — pick strategy and merge a PR
- pr-status — list/open PRs and issues (summary/detail)
- review-tasks — turn review comments into actionable tasks
- create-pr — create a SPEC‑first draft PR from a new branch

Prerequisites
- GitHub CLI (`gh`) authenticated with repo permissions
- Sufficient repo permissions to merge (for approve/merge)

Scripts
- `.claude/scripts/approve-pr.sh`
- `.claude/scripts/merge-pr.sh`
- `.claude/scripts/pr-status.sh`
- `.claude/scripts/create-pr.sh`
- `.claude/scripts/review-tasks.sh`

Notes
- All PRs must include SPEC sections and at least one mermaid diagram (enforced by CI).
- Use the skills’ prompts to generate conventional PR titles and bodies.

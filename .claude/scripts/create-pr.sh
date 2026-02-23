#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <branch-name> <title> [body]" >&2
  exit 1
fi

BR="$1"; TITLE="$2"; BODY="${3:-}"
git fetch origin main --quiet || true
git checkout -b "$BR" origin/main
if [[ -z "$BODY" ]]; then
  echo "Enter PR body, end with Ctrl-D:" >&2
  BODY=$(cat)
fi
gh pr create --title "$TITLE" --body "$BODY" --draft

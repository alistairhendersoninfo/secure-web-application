#!/usr/bin/env bash
set -euo pipefail

usage() { echo "Usage: $0 <pr-number> [--squash|--merge|--rebase] [--admin]"; }

if [[ ${1:-} == "-h" || ${1:-} == "--help" || $# -lt 1 ]]; then usage; exit 1; fi

PR="$1"; shift || true
METHOD="--squash"
ADMIN=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --squash|--merge|--rebase) METHOD="$1" ;;
    --admin) ADMIN="--admin" ;;
  esac
  shift
done

echo "Merging PR #$PR ($METHOD $ADMIN)"
gh pr view "$PR" --json number,title,isDraft,reviewDecision,statusCheckRollup,mergeable,url || { echo "PR not found"; exit 1; }
set +e
gh pr merge "$PR" "$METHOD" --delete-branch $ADMIN
rc=$?
set -e
if [[ $rc -ne 0 ]]; then
  echo "Merge failed. Showing checks:"
  gh pr view "$PR" --json statusCheckRollup,reviewDecision,isDraft,mergeable | jq
  exit $rc
fi
echo "Merged PR #$PR"

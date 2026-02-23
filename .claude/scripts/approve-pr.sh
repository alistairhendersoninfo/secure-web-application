#!/usr/bin/env bash
set -euo pipefail

PR="${1:-}"
if [[ -z "$PR" ]]; then
  echo "No PR specified. Open PRs:" >&2
  gh pr list --state open
  exit 1
fi

echo "Checking admin permissions..."
ADMIN=$(gh api repos/${GITHUB_REPOSITORY} --jq '.permissions.admin')
if [[ "$ADMIN" != "true" ]]; then
  echo "You are not an admin on this repo. Request a regular review." >&2
  exit 1
fi

echo "Review summary for PR #$PR"
gh pr view "$PR" --json number,title,reviewDecision,statusCheckRollup,url | jq
echo "Reviews:"; gh api repos/${GITHUB_REPOSITORY}/pulls/$PR/reviews | jq '.[].state' | sort | uniq -c || true
echo "Inline comments (count):"; gh api repos/${GITHUB_REPOSITORY}/pulls/$PR/comments | jq 'length'

echo "If all issues are addressed, you can merge with:"
echo ".claude/scripts/merge-pr.sh $PR --squash --admin"

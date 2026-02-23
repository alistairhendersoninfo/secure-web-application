#!/usr/bin/env bash
set -euo pipefail

PR="${1:-}"
if [[ -z "$PR" ]]; then
  echo "Usage: $0 <pr-number>" >&2
  exit 1
fi

OUT_DIR=".claude/tasks"
mkdir -p "$OUT_DIR"
OUT_FILE="$OUT_DIR/PR-$PR.md"

echo "# Review Tasks for PR #$PR" > "$OUT_FILE"
echo >> "$OUT_FILE"
echo "## Inline Comments" >> "$OUT_FILE"
gh api repos/${GITHUB_REPOSITORY}/pulls/$PR/comments | jq -r '.[] | "- [ ] (\(.path):\(.line // .original_line)) \(.body | gsub("\n"; " ")) [link](\(.html_url))"' >> "$OUT_FILE" || true

echo >> "$OUT_FILE"
echo "## Review Summary" >> "$OUT_FILE"
gh api repos/${GITHUB_REPOSITORY}/pulls/$PR/reviews | jq -r '.[] | "- \(.state) by @\(.user.login) at \(.submitted_at)"' >> "$OUT_FILE" || true

echo "Tasks written to $OUT_FILE"

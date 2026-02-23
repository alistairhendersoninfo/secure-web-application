#!/usr/bin/env bash
set -euo pipefail

if [[ $# -eq 0 ]]; then
  echo "Open PRs:"; gh pr list --state open --json number,title,headRefName,reviewDecision,statusCheckRollup,additions,deletions,changedFiles | jq -r '.[] | "#\(.number) \(.title) [branch: \(.headRefName)] files=\(.changedFiles) +/-=+\(.additions)/-\(.deletions) reviews=\(.reviewDecision)"'
  echo; echo "Open issues:"; gh issue list --state open --json number,title,labels,author,createdAt | jq -r '.[] | "#\(.number) \(.title) [labels: \(.labels|map(.name)|join(","))]"'
  exit 0
fi

if [[ "$1" == "issue" ]]; then
  num="$2"; gh issue view "$num" --json number,title,body,author,createdAt,updatedAt,labels,comments,state,url | jq
  exit 0
fi

PR="$1"
gh pr view "$PR" --json number,title,body,headRefName,baseRefName,author,createdAt,updatedAt,labels,reviewDecision,reviewRequests,reviews,statusCheckRollup,additions,deletions,changedFiles,files,comments,mergeable,isDraft,url | jq

---
name: merge-pr
description: Check PR status, resolve issues, and merge a pull request
argument-hint: [pr-number]
disable-model-invocation: false
allowed-tools: Bash, Read, AskUserQuestion
---

# Merge PR Workflow

## Step 1: Identify the PR
If `$ARGUMENTS` has a number, use it; else list open PRs and ask which one:
```bash
gh pr list --state open
```

## Step 2: Show status
```bash
gh pr view <PR> --json number,title,isDraft,reviewDecision,statusCheckRollup,mergeable,headRefName,baseRefName,url
```
Summarize draft/ready, reviews, checks, and mergeable state.

## Step 3: Choose strategy
Use AskUserQuestion:
- Squash and merge (recommended)
- Merge commit
- Rebase and merge
- Cancel

## Step 4: Merge
Map choice to:
```bash
gh pr merge <PR> --squash --delete-branch || \
gh pr merge <PR> --merge --delete-branch || \
gh pr merge <PR> --rebase --delete-branch
```
If branch protection blocks, report the unmet checks.

## Step 5: Summary
Print PR number/title, method used, and branch cleanup status.


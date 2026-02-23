---
name: approve-pr
description: Admin self-review gate — verifies review issues are resolved then clears PR for merge
argument-hint: [pr-number]
disable-model-invocation: false
allowed-tools: Bash, Read, AskUserQuestion
---

# Approve PR Workflow (Admin Self-Review)

You are verifying a PR in this repo is ready to merge using the admin bypass only after all review issues are addressed.

## Step 1: Identify the PR
- If `$ARGUMENTS` has a PR number, use it; otherwise:
```bash
gh pr view --json number,title,state,headRefName,isDraft 2>&1 || gh pr list --state open
```

## Step 2: Verify admin access
```bash
gh api repos/${GITHUB_REPOSITORY} --jq '.permissions.admin'
```
If not admin, stop and instruct the user to request a normal approval.

## Step 3: Check for open review feedback
- Reviews:
```bash
gh api repos/${GITHUB_REPOSITORY}/pulls/<PR>/reviews
```
- Inline comments:
```bash
gh api repos/${GITHUB_REPOSITORY}/pulls/<PR>/comments
```
Summarize any unresolved items (by file/line/summary). If none remain, proceed.

## Step 4: Gate decision
Use AskUserQuestion to confirm merging now (squash recommended) or cancel.

## Step 5: Merge
```bash
gh pr merge <PR> --squash --delete-branch --admin
```
If merge fails due to checks/branch protection, report and stop.

## Step 6: Summary
Print PR number/title, merge method, and confirm branch deleted.


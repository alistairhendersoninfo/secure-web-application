---
name: review-tasks
description: Read PR review comments and create todo tasks for each issue
argument-hint: [pr-number]
disable-model-invocation: false
allowed-tools: Bash, Read, TaskCreate, TaskUpdate, TaskList, AskUserQuestion
---

# Review Tasks Workflow

Create actionable tasks from PR review feedback.

## Step 1: Identify PR
```bash
gh pr view --json number,title,headRefName 2>&1 || gh pr list --state open
```

## Step 2: Fetch feedback
```bash
gh api repos/${GITHUB_REPOSITORY}/pulls/<PR>/reviews
gh api repos/${GITHUB_REPOSITORY}/pulls/<PR>/comments
gh pr view <PR> --comments
```

## Step 3: Parse and categorize
For each comment: Severity (critical/high/medium/low), File, Line, Issue summary, Suggested fix (if any), Reviewer, Link.

## Step 4: Create tasks
For each item, use TaskCreate with:
- subject: `Fix: <summary>`
- description: include severity, file:line, details, suggested patch, link
- activeForm: `Fixing <short description>`

## Step 5: Summary
Print totals and task IDs. Suggest working in severity order.


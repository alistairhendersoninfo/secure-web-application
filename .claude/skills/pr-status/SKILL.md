---
name: pr-status
description: Show open PRs and issues — summary or detailed view
argument-hint: "[pr-number | issue-number]"
disable-model-invocation: false
allowed-tools: Bash, Read
---

# PR Status

Three modes:
- Summary: no args — list open PRs and issues
- PR Detail: numeric arg — show one PR in detail
- Issue Detail: `issue <number>` — show one issue in detail

## Summary Mode
```bash
gh pr list --state open --json number,title,headRefName,author,createdAt,labels,reviewDecision,statusCheckRollup,additions,deletions,changedFiles --limit 50
gh issue list --state open --json number,title,labels,author,createdAt,updatedAt --limit 50
```
Present both as tables with counts.

## PR Detail Mode
```bash
gh pr view <PR_NUMBER> --json number,title,body,headRefName,baseRefName,author,createdAt,updatedAt,labels,reviewDecision,reviewRequests,reviews,statusCheckRollup,additions,deletions,changedFiles,files,comments,mergeable,isDraft,url
```
Present header, status, changes, description, comments, reviews.

## Issue Detail Mode
```bash
gh issue view <ISSUE_NUMBER> --json number,title,body,author,createdAt,updatedAt,labels,comments,state,url
```
Present header, status block, description, comments.


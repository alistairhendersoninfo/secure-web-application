# PR Workflow

## Branch → PR → CI → Merge → Delete

### 1. Create Feature Branch

```bash
git checkout -b feat/<descriptive-name>
# or: docs/<name>, fix/<name>, ci/<name>, chore/<name>
```

### 2. Commit and Push

```bash
git add <specific-files>
git commit -m "type(scope): description"
git push -u origin feat/<descriptive-name>
```

Commit message prefixes: `feat`, `fix`, `docs`, `ci`, `chore`, `refactor`, `test`

### 3. Create PR

```bash
gh pr create --title "type(scope): description" --body "..."
```

PR title must pass the `semantic-pr-title.yml` CI check.

### 4. CI Checks

Required checks before merge:
- **ci.yml** — Rust build, clippy lints, tests
- **docs-quality.yml** — Markdown linting, link checking (lychee)
- **codespell** — Spell checking
- **mermaid-validate.yml** — Mermaid diagram syntax validation
- **semantic-pr-title.yml** — PR title format validation

### 5. Merge

Branch protection requires:
- All CI checks passing
- Linear history (squash merge)
- Admin approval

```bash
gh pr merge <number> --squash --delete-branch
```

### 6. Cleanup

The `--delete-branch` flag deletes the remote branch after merge. Local cleanup:

```bash
git checkout main
git pull
git branch -d feat/<descriptive-name>
```

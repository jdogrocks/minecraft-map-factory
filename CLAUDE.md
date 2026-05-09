# CLAUDE.md

## CI/CD Workflow Conventions

### Branch Naming

Use `{type}/{issue-identifier-lowercase}` format.

Types: `feat/`, `fix/`, `refactor/`, `chore/`, `docs/`, `test/`

Examples: `feat/min-42`, `fix/min-55`, `chore/min-25-cicd-validation`

### No Direct Pushes to `main`

All changes go through pull requests. Never commit directly to `main`.

### Pull Request Creation

```bash
gh pr create --title "{ISSUE-ID}: {title}" --body "Resolves [{ISSUE-ID}](https://cirruslycloudy.com/MIN/issues/{ISSUE-ID})"
```

### CI Monitoring

After pushing, verify CI passes:

```bash
gh pr checks
```

On failure, read logs and fix:

```bash
gh run view --log-failed
```

Fix the issue and re-push. Cap at 3 attempts before marking the issue as `blocked`.

### Merge

When the issue is complete and CI passes, squash-merge:

```bash
gh pr merge --squash --delete-branch
```

## Cowork session boundaries

Cowork session is read-only by default in this project. Repo integrity and live deploy state are owned by the Paperclip company agents; Cowork is a thinking partner, not a maintenance hand.

**Forbidden without explicit owner override:**

- Direct mutations to repo: `git commit`, `git push`, `gh pr create/merge`, edits to checked-in files
- Direct mutations to live deploy targets: `rsync`/`cp`/`docker cp` into the `minecraft-papermc` container or world directories, `setblock`/`fill`/state-changing RCON commands
- Direct mutations to Paperclip state: SQL `UPDATE`/`INSERT`/`DELETE` on the `paperclip` database, status changes via API outside the issue/comment surface

**Allowed:**

- Read-only diagnostic queries: DB SELECTs, file reads, log tails, RCON `data get` / `datapack list`, `gh pr view`
- Filing new MIN issues via the Paperclip API and posting comments — that IS the routing channel

**Escape valve:** Owner can explicitly authorize a bypass ("manually override", "do it yourself this time"). Any bypass must be documented in a comment on the related MIN issue with a `Posted by Owner via Cowork session, not by an agent` header so the audit trail is intact.

The agent system is the source of truth.

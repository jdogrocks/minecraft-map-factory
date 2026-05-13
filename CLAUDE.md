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

## Independent QA Gate for Deploy and Visual-Change Issues

For any issue whose definition-of-done includes visible behavior on the live `minecraft-papermc` container (any deploy, any visual change, any walkthrough-gated issue), the CEO **MUST** route an independent QA agent to verify in-world behavior before closing the issue. The QA agent must be one of: QA Lead, Test Engineer, or Code Quality Specialist — **NOT** the agent that wrote the code or ran the smoke script.

The QA agent posts an `assigned_to: human` comment describing what they checked and what they observed. CEO does **NOT** mark the issue done until that QA comment is present **AND** the owner has confirmed the walkthrough on the parent `in_review` issue.

If a builder self-report and the QA report disagree, route the conflict back to `in_progress` and ask the builder to address the QA finding.

Owner-side `Posted by Owner via Cowork session` comments that explicitly reject a walkthrough **MUST** be treated as binding signals; CEO cannot close the parent issue while an unaddressed owner-rejection comment exists newer than the most recent QA-pass comment.

**Applies to**: deploy issues, visual-change issues, any issue with a walkthrough requirement.
**Does not apply to**: pure code/CI/infra work with no in-world behavior component.

> **Background**: Rule added 2026-05-12 after two incidents (MIN-159 Phase D rev-1 on 2026-05-08, MIN-194 corridor regression on 2026-05-12) where CEO closed issues on builder self-report + smoke exit 0 while owner had posted a rejection comment with a documented bug.

## Separate-Agent Code Reviews

PR reviews on `main` **MUST** come from an agent role that is distinct from the PR's author. The reviewer cannot be the same agent that opened or committed to the PR.

Eligible reviewer roles: QA Lead, Test Engineer, Code Quality Specialist — **NOT** the builder/author.

This is the code-review analogue of the Independent QA Gate rule: the builder does not mark its own homework. A `COMMENTED` post from the author does not satisfy branch protection or this rule.

> **Background**: Rule added 2026-05-13 after the MIN-194 / MIN-198 cycles where the same identity authored commits and approved the review, with the QA Lead's review reduced to a non-approving `COMMENTED` post.

## `done` Requires Merged PR

For any issue whose definition of done includes a merged PR, the status `done` is **invalid** until the linked PR is in `MERGED` state on `main`. An open PR is not done, even if CI passes and the code is correct.

- Agents **MUST NOT** flip an issue to `done` immediately after opening or pushing a PR.
- The issue stays `in_review` until the PR is squash-merged.
- CEO enforces this on review: any `done` issue with an open PR must be returned to `in_review`.

**Does not apply to**: issues with no PR (pure Paperclip task work, docs, config-only changes that bypass the PR gate).

> **Background**: Rule added 2026-05-13. MIN-198 was flipped to `done` 13 seconds after `startedAt` while PR #95 was still OPEN. Same status-drift pattern recurred across MIN-194, MIN-198, and others.

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

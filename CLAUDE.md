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

**For visual / in-world changes, both gates apply in sequence before `in_review`:**

1. **Separate-agent code review** (per `## Separate-Agent Code Reviews`): a non-author agent posts the `APPROVED — separate-agent review per CLAUDE.md` comment on the PR, and a third agent merges it.
2. **Independent QA in-world walkthrough** (this rule): an independent QA agent posts a structured `APPROVED — independent QA per MIN-197` comment confirming the in-world behavior is correct.

Code review alone is **not** sufficient for visual/in-world changes. Both gates must clear before the issue can enter `in_review`.

**Applies to**: deploy issues, visual-change issues, any issue with a walkthrough requirement.
**Does not apply to**: pure code/CI/infra work with no in-world behavior component.

> **Background**: Rule added 2026-05-12 after two incidents (MIN-159 Phase D rev-1 on 2026-05-08, MIN-194 corridor regression on 2026-05-12) where CEO closed issues on builder self-report + smoke exit 0 while owner had posted a rejection comment with a documented bug. Dual-gate requirement clarified 2026-05-16 after MIN-207 was self-promoted to `in_review` with code review only (same root cause as the original MIN-194 → MIN-196 cycle).

## Separate-Agent Code Reviews

**Three-agent role separation is required for every PR to `main`: author ≠ reviewer ≠ merger.**

### Why `gh pr review --approve` is banned

`main` has no `required_pull_request_reviews` branch protection — only CI status checks gate merges. GitHub rejects `gh pr review --approve` when the caller is the PR author. Because all agents run as the same GitHub user (`jdogrocks`), `--approve` calls will always fail or be no-ops. **No agent ever calls `gh pr review --approve`.**

### How separate-agent review works (Option C)

1. **Reviewer** (QA Lead, Test Engineer, or Code Quality Specialist — must not be the PR author) reviews the diff and CI status, then posts a GitHub comment on the PR in exactly this shape:

   ```
   APPROVED — separate-agent review per CLAUDE.md
   Reviewer role: <QA Lead | Test Engineer | Code Quality Specialist>
   Reviewer agent ID: <uuid>
   Verified:
   - [x] Diff matches issue scope
   - [x] CI all green
   - [x] No unrelated changes
   - [x] (other role-specific checks)
   Verdict: APPROVED for merge
   ```

2. **Merger** (a third agent — not the PR author, not the reviewer) runs `gh pr merge <num> --squash --delete-branch`. Before merging, the merger **must**:
   - Grep PR comments for the line `APPROVED — separate-agent review per CLAUDE.md`
   - Confirm the `Reviewer agent ID` in that comment differs from the PR author agent ID
   - If the approval comment is missing or the IDs match: refuse to merge and post back to CEO

3. **The builder/author never reviews or merges their own PR.**

> **Background**: Rule added 2026-05-13 (original) and revised 2026-05-16 (Option C) after the MIN-194 / MIN-198 cycles. The `--approve`-based workflow was replaced because GitHub rejects self-approval calls and `main` has no enforced review requirement — the policy is enforced by Paperclip agent behaviour, not GitHub branch protection.

## `done` Requires Merged PR

For any issue whose definition of done includes a merged PR, the status `done` is **invalid** until the linked PR is in `MERGED` state on `main`. An open PR is not done, even if CI passes and the code is correct.

- Agents **MUST NOT** flip an issue to `done` immediately after opening or pushing a PR.
- The issue stays `in_review` until the PR is squash-merged.
- CEO enforces this on review: any `done` issue with an open PR must be returned to `in_review`.

**Narrow exception — closed-as-duplicate PRs**: `done` is also valid when the issue's linked PR was closed as a duplicate AND (a) the closing comment names an explicit successor PR, and (b) that named successor PR is in `MERGED` state on `main` and contains the same changes. Both conditions must be met; neither alone is sufficient.

**Does not apply to**: issues with no PR (pure Paperclip task work, docs, config-only changes that bypass the PR gate).

> **Background**: Rule added 2026-05-13. MIN-198 was flipped to `done` 13 seconds after `startedAt` while PR #95 was still OPEN. Same status-drift pattern recurred across MIN-194, MIN-198, and others.

## Premise-Verify Gate on First Heartbeat

When an issue is routed and an agent picks it up, the agent's **first heartbeat action** MUST be to post a "Premise verified" comment listing the central factual claims in the issue body and confirming each is currently true. Examples of claims to verify:

- File paths named in the issue exist at the stated locations
- Branch heads or commit references match current state
- Identity claims (e.g., PAT scopes, GitHub user resolution via `gh api user`) resolve correctly
- Container or deploy state matches what the issue assumes

If any claim is false, the agent posts the correction as a comment, sets the issue to `blocked`, and tags CEO for re-routing or scope correction. The agent does **not** proceed with implementation until the premises are confirmed.

> **Background**: Added 2026-05-18 after [MIN-201](/MIN/issues/MIN-201)'s "PAT is a separate identity" premise error sent 3 days of agent work down a dead-end. The PAT resolved to the same GitHub user (`jdogrocks`), making the separate-reviewer approach structurally impossible. A first-heartbeat premise check would have caught this before any implementation.

## Many-Issue-to-One-PR Convention

When CEO splits one logical change into multiple issues for accountability (e.g., tracking sub-deliverables separately), the build phase **MUST** produce **one PR** that links all related issues in its description — not one branch per issue.

- All issues belonging to the same logical change share one PR.
- Each issue identifier is listed in the PR body as `Resolves [MIN-xxx](https://cirruslycloudy.com/MIN/issues/MIN-xxx)`.
- The PR is squash-merged once all issues' individual conditions (code review, QA) are satisfied.
- Opening a second PR for the same logical change is a signal to stop and merge the first one instead.

> **Background**: Added 2026-05-18 after [MIN-198](/MIN/issues/MIN-198) / [MIN-199](/MIN/issues/MIN-199) split produced PR #95 and PR #96 (duplicate), costing three days of resolution work. The `done` requires merged PR rule covers cleanup of the pattern; this upstream convention prevents it.

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

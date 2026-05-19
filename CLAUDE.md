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

For any issue whose definition-of-done includes visible behavior on the live `minecraft-papermc` container (any deploy, any visual change, any walkthrough-gated issue), the CEO **MUST** route an independent QA agent to verify in-world behavior before closing the issue. For **visual-change and in-world walkthrough issues**, the QA agent must be one of: **QA Lead or Texture Artist** — **NOT** Test Engineer (who performs headless inspection only, see Rule B below) and NOT the agent that wrote the code or ran the smoke script. For non-visual deploy issues (pure CI/infra/smoke), Test Engineer or Code Quality Specialist remain eligible.

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

## Model Tier Policy for Visual-Judgment Roles

Roles that perform visual / spatial / aesthetic judgment on this project MUST be on `claude-opus-4-6` (or its successor). Sonnet and Haiku are not sufficient for visual judgment work and have produced false-positive approvals in the past.

**Currently in scope (pinned to opus as of 2026-05-19):**

- Minecraft Scene Designer
- Texture Artist
- QA Lead — for visual-change Gate 2 issues specifically

Non-visual roles continue on Sonnet (leads + most engineering) or Haiku (specialty ICs). When a non-visual role is asked to do visual judgment, either temporarily re-pin to opus OR re-route to a visual-judgment role — not both. CEO is exempt; routing decisions don't require visual judgment if the visual-QA routing rule (below) is followed.

**Background**: Added 2026-05-19 after [MIN-207](/MIN/issues/MIN-207) sprawl shipped through the dual-gate system but was rejected on owner walkthrough — the cramped/unenclosed result was not caught by Sonnet-tier visual review.

## Visual-Change QA Routing Exclusion

Visual-change in-world QA gates (per `## Independent QA Gate for Deploy and Visual-Change Issues`) MUST NOT be routed to Test Engineer, regardless of Test Engineer's model. Test Engineer's strength is parse-clean smoke testing and headless code inspection — neither constitutes visual judgment.

**Eligible Gate 2 reviewers for visual changes:**

- QA Lead
- Texture Artist
- Any new visual-design specialist hires

The author of the change is also ineligible to QA their own change per the dual-gate rule.

If a previously-routed Gate 2 issue's QA approval came from an ineligible role, CEO MUST re-route to an eligible reviewer before promoting the parent to `in_review`. Existing `done` or `in_review` issues approved by an ineligible visual-QA reviewer are subject to retroactive re-routing if owner walkthrough rejects the result.

**Background**: Added 2026-05-19 after [MIN-232](/MIN/issues/MIN-232) (Gate 2 for sprawl) was reassigned from QA Lead to Test Engineer (Haiku), which approved via *"Headless agent verification via code inspection"* and noted *"Final human verification in Minecraft client recommended"* — a tier-mismatch approval that rubber-stamped a visual gate the agent could not perform.

## Build Semantics — Replace, Not Add

Default semantics for any in-world visual change: **replace the affected envelope**, not add geometry alongside. If a change is conceptually "expansion of the mall footprint," the old envelope at the old dimensions is OBSOLETE and must be torn down as part of the same change. The new envelope is the only envelope.

**Additive passes are allowed ONLY when:**

- The issue body explicitly contains the word "additive" in the title or scope statement.
- The body includes a rationale for preserving the old envelope (e.g. "preserve SEARZ z=-261..-279 mechanics" carve-outs).
- The rationale survives owner review at issue-filing time (i.e., owner does not reject the additive scope).

Default to replace. Make the agent argue for additive, not the other way around.

**For the builder agent**: if an issue description describes "expansion" or "redesign" or "scale up" without the explicit "additive" + rationale, the build MUST tear down the existing envelope before constructing the new one. Demolition fills (`fill ... minecraft:air`) over the old structural blocks belong at the start of `all.mcfunction` before any rebuild step.

**Background**: Added 2026-05-19 after [MIN-207](/MIN/issues/MIN-207) `f1_sprawl.mcfunction` shipped as *"Additive pass — runs AFTER all existing build functions. Expands outward without destroying mechanics-critical existing interiors"*. The original 98×178 mall outer walls were left in place inside the new shell, functioning as interior dividers and producing a cramped/unenclosed walkthrough experience.

## Visual-Change QA Requires In-World Block Queries

Approval of a visual-change Gate 2 issue MUST include **all three** of the following in the structured `APPROVED — independent QA per MIN-197` comment:

1. **Player-viewpoint descriptions** at named coordinates and facing directions — what the player actually sees standing at e.g. `(0, 66, -150) facing north`. Minimum three viewpoints relevant to the change.
2. **RCON `data get block` assertions** at the structural coordinates the change should have placed or removed. Quote the actual block IDs returned (e.g. `minecraft:white_concrete`, `minecraft:air`).
3. **A "what's notable / what's missing" prose paragraph** distinguishing what the change accomplished vs what remains for owner acceptance.

The phrase *"Headless agent verification via code inspection"* is explicitly disallowed as a sole basis for visual-change Gate 2 approval. Code-only inspection fails this rule and CEO MUST re-route to a reviewer who will produce the three artifacts.

**Background**: Added 2026-05-19 after [MIN-232](/MIN/issues/MIN-232) Gate 2 approval cited *"Headless agent verification via code inspection, function execution, and deployment confirmation"* and approved the gate without any in-world block query, even noting *"Final human verification in Minecraft client recommended to confirm visual sprawl geometry before closing MIN-207"* — approval was given for a check the reviewer had not performed.

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

# Live-Server Smoke Check Runbook

End-to-end smoke testing for Minecraft datapacks against the live PaperMC
container at `hp-mini-pc` (192.168.7.211). Background: [MIN-156](/MIN/issues/MIN-156).

## When to run

Run a smoke check whenever a change touches any of the following:

- `pack_format` or `pack.mcmeta` — wrong values are silently accepted at
  load time but cause runtime failures or are rejected outright
- Function paths (`data/<namespace>/function/`) — the directory name is
  singular `function`, not `functions`; the server will silently skip
  misnamed directories
- Command syntax inside `.mcfunction` files — server version dictates valid
  commands; syntax errors are only caught at execution time
- `load.json` / `tick.json` tag function lists

A smoke check is not needed for changes that only touch map geometry (`.mca`
region files), textures, or non-datapack pipeline code.

## How to run

```bash
scripts/motfb-smoke.sh <path-to-built-pack> [function-to-exercise ...]
```

`<path-to-built-pack>` is the local directory or `.zip` of the datapack to
test. Optionally pass one or more fully-qualified Minecraft function IDs
(e.g. `motfb:init`) to exercise after the pack loads.

### Example

```bash
# Test a freshly-built MOTFB datapack and run its init function
scripts/motfb-smoke.sh output/motfb-datapack motfb:init

# Quick pack_format / load check only (no extra functions)
scripts/motfb-smoke.sh output/motfb-datapack
```

The script handles all server-side steps:

1. Copies the pack into the container's datapacks directory
2. Fixes file ownership
3. Issues `reload` + `datapack enable`
4. Reads `datapack list` and tails `latest.log`
5. Runs any extra functions you specified via rcon
6. **Cleans up**: removes the pack and reloads again

## What to check in the output

A passing run looks like:

```
[motfb-smoke] pack_format accepted: <N>
[motfb-smoke] datapack list: "file/motfb-test-NNN" (enabled)
[motfb-smoke] WARN/ERROR count: 0
[motfb-smoke] function motfb:init → <output or "no output">
[motfb-smoke] PASS — cleanup done
```

Flag any run that contains:

| Signal | Meaning |
|--------|---------|
| `pack_format` rejected or `Unknown pack_format` in log | Wrong format version; check MC 26.x pack_format value |
| `WARN` or `ERROR` in log grep | Bad command syntax, missing function file, tag resolution failure |
| `datapack list` does not show the test pack as `(enabled)` | Load failure; inspect full log around `[DataPackManager]` |
| Script exits non-zero | Infrastructure problem — check docker socket / RCON connectivity |

## Guardrails

The script enforces these limits automatically — you cannot override them
at the call site:

- **Temp name only**: packs are installed under an ephemeral name
  (`motfb-test-<random>`), never under the production pack name.
- **No production world mutations**: the script never touches
  `Times_Square__NYC` map data or any other world files.
- **Auto-cleanup**: the temp pack is removed and the server is reloaded
  whether the smoke check passes or fails. A crash in the script still
  triggers the cleanup trap.
- **No destructive commands**: the script never issues `/op`, `/stop`,
  `/save-off`, or any command that modifies world state beyond loading a
  datapack.

## Server details (for reference)

| Item | Value |
|------|-------|
| Host | `hp-mini-pc` — 192.168.7.211 |
| Container | `minecraft-papermc` (`itzg/minecraft-server`) |
| MC version | 26.1.2 (Paper build 60) |
| RCON | `docker exec minecraft-papermc rcon-cli "<cmd>"` |
| Datapacks dir | `/data/Times_Square__NYC/datapacks/` |
| Host volume | `/home/jason/minecraft-server/data/` ↔ `/data/` |
| `function-permission-level` | 2 |

## Troubleshooting

**Script cannot reach docker**: confirm the executing user is in the `docker`
group on `hp-mini-pc`, or that the agent's workspace has the docker socket
mounted.

**`datapack enable` returns "Unknown datapack"**: the pack directory was not
copied correctly, or the name passed to `enable` does not match what was
copied. Check the cleanup step — a previous failed run may have left a
stale pack directory.

**Function produces no output**: the function may use `tellraw` targeting
a specific player. In an automated smoke context there are no players; use
`say` or write to the server log instead when authoring test functions.

**pack_format value to use**: for MC 26.1.2 the correct integer is `61`
(data-pack format, not resource-pack format). Run
`docker exec minecraft-papermc rcon-cli "data get entity @p DataVersion"`
on a live player to confirm the data version if unsure. The smoke script
resolves this automatically from the running server's version report.

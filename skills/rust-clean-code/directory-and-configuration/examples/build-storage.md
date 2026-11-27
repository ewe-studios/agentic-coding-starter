# Build storage: where Cargo writes outputs and its registry

_Written in ASD-STE100 Simplified Technical English (STE). Code and identifiers stay as they
are._

## Why this file exists

A Rust `target` directory grows to tens of gigabytes. One tree for each worktree shares
nothing, so several worktrees fill a volume. A full volume stops every build, and the errors
look like code errors. This file gives the one setting that prevents it, and states the risks
in plain words.

## The setting

```sh
export CARGO_TARGET_DIR=/Volumes/Outside/cargo-target
export CARGO_HOME=/Volumes/Outside/cargo-home
```

| Variable | What it moves | Default |
|---|---|---|
| `CARGO_TARGET_DIR` | Every compiled artifact: `deps`, incremental data, build-script output | `./target` in each workspace |
| `CARGO_HOME` | The registry cache, `config.toml`, credentials, and the `cargo install` binaries | `~/.cargo` |

In this repository, `.envrc` exports both values, and `.claude/settings.json` carries the
same values for agent shells.

## Does a shared target directory break builds between worktrees?

No, if you know three facts.

1. **Cargo keys each artifact.** The key holds the package, its features, the profile, the
   target triple, the rust flags and the compiler version. Two worktrees share an artifact
   only when every one of them matches. When one differs, Cargo writes a separate
   fingerprint. No build reads another build's output.
2. **Cargo locks the directory.** Two builds do not run in the same directory at the same
   time; the second waits. The cost is time, not correctness.
3. **Some commands act on the whole directory.** `cargo clean` in one worktree removes the
   shared artifacts for every worktree. Use `cargo clean -p <crate>` when you mean one crate.

What a shared directory does **not** do: it does not mix allocators, profiles or feature
sets, because they are part of the key. It does not publish anything. It does not change a
binary that Cargo already built from the same inputs. That is the point: the second worktree
reuses it.

## Set it safely

1. If `CARGO_HOME` changes, copy the old directory first. It holds `config.toml`,
   credentials, and the binaries that `cargo install` put in `bin`.

   ```sh
   mkdir -p /Volumes/Outside/cargo-home
   rsync -a "$HOME/.cargo/" /Volumes/Outside/cargo-home/
   ```

2. Keep `$CARGO_HOME/bin` on `PATH`. A missing value re-downloads the registry and hides
   the installed tools.
3. Create the target directory once: `mkdir -p /Volumes/Outside/cargo-target`.

## Verify it

```sh
cargo metadata --format-version 1 | jq -r .target_directory
# /Volumes/Outside/cargo-target

echo "$CARGO_HOME"
# /Volumes/Outside/cargo-home
```

A build then writes only under those two paths. Check with `du -sh` on the workspace after a
build: `./target` stays absent.

## Keep it small

```sh
cargo sweep --stamp --time 7 /Volumes/Outside/cargo-target
```

`cargo-sweep` deletes artifacts that were not touched in seven days. With `--stamp`, each run
deletes only what the previous run did not see. Run it weekly, and never during a build. The
cost of a removed artifact is one rebuild.

## Editors, agents and Kiro

An editor or an agent runs `cargo` in its own shell. It sees the values only if its
environment has them.

| Tool | What to do |
|---|---|
| Shell | `direnv allow` after a change to `.envrc` |
| Claude Code agents | Nothing: `.claude/settings.json` sets the values |
| Kiro | Set both variables in its environment, or launch it from a shell that exports them; put its workspace root on the large volume |
| CI | Set `CARGO_TARGET_DIR` to a runner path; the repository `.envrc` does not apply |

Ask the user for the location when it is not set. Do not invent one: a path on a small disk
recreates the problem this file solves.

## The one-minute version

```sh
export CARGO_TARGET_DIR=/Volumes/Outside/cargo-target
export CARGO_HOME=/Volumes/Outside/cargo-home
mkdir -p "$CARGO_TARGET_DIR"
cargo metadata --format-version 1 | jq -r .target_directory   # must print it
```

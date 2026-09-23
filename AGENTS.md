# cppbro

Rust CLI for generating small C++ projects and wrapping their CMake/Ninja build,
run, and clean workflows. Read [README.md](README.md), the
[knowledge index](knowledge/index.md), and [current state](knowledge/current-state.md)
before continuing.

## Project boundaries

- Preserve the Rust modules, Cargo manifests and lockfile, tests, and Git history.
  Use project-relative paths so any checkout location works.
- This repository is public by the owner's explicit choice. Review every staged
  file before pushing; never include credentials, personal records, or private
  workstation configuration.
- Preserve deterministic, source-only templates and target-based CMake design.
  Add dependencies only for a concrete project requirement.
- Exercise generation, force, and clean behavior only in a verified test directory
  or an explicitly selected target project. Preserve symlink and path-traversal
  safety checks.
- Keep the project understandable when cloned alone; do not rely on a local chat
  or sibling project for essential context.

## Development and verification

```sh
cargo build --locked
cargo test --locked
```

Read relevant tests before changing behavior. Some integration tests return early
without CMake or Ninja; report that limitation. Generated-project builds also need
a C++ compiler. Recreate native toolchains and ignored `target/` output on each OS.
A macOS check does not establish Linux behavior.

## Machine handoff

Inspect status, branch, and remote before work. Fetch and use `git pull --ff-only`
only from a clean checkout on the intended branch. Preserve dirty or diverged work;
do not automatically reset, stash, force-push, or clean it. Use separate branches
and checkouts for concurrent writers.

Update current state with meaningful changes, focused checks, and next steps.
Review and commit intended files, push when authorized, and verify the remote
branch SHA. Git shares committed source and knowledge; ignored files, credentials,
native dependencies, and editor registrations require separate handling.

Keep local project files inside the checkout by default. Valuable private material
must have a separate private backup. If external storage is necessary, document a
nonsecret pointer, retrieval steps, ownership, and unavailable-storage behavior.

## Maintain knowledge

Use the existing OKF v0.2 `knowledge/` bundle. Concepts need a nonempty `type`,
truthful provenance, and current validation limits; only the root index declares
`okf_version`. Validate local links, frontmatter, and whitespace after edits.
Keep observations, proposals, implementation, and tests distinct.

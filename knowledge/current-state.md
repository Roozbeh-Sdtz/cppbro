---
type: Project State
title: cppbro current state
description: Rust project scope, development checks, and portable handoff.
status: stable
generated:
  by: codex/gpt-6
  at: "2026-09-23T16:17:53Z"
sources:
  - id: owner
    resource: ../AGENTS.md
    title: Public repository policy and project handoff scope
  - id: overview
    resource: ../README.md
    title: Project overview and development commands
  - id: manifest
    resource: ../Cargo.toml
    title: Rust package manifest
  - id: tests
    resource: ../tests/integration.rs
    title: Integration tests
---

# Scope

cppbro generates small C++ projects and wraps CMake/Ninja build, run, and clean
workflows. Cargo.toml defines version 0.1.0 using Rust edition 2021. The source
modules are main.rs, cli.rs, generator.rs, templates.rs, and workflow.rs; the
repository includes a Cargo lockfile and integration tests.[^manifest][^tests]

The README describes minimal, cli, and embedded templates and defaults to C++20.
Preserve its small source-only layouts and target-based CMake convention. Package
installation availability and roadmap claims were not revalidated for this
documentation update.[^overview]

# Current change and publication

The owner authorized publishing portable agent instructions, this knowledge
bundle, and a Git command guide. The existing public visibility, main branch,
remote, source code, dependencies, and history are preserved. These new files
contain project guidance only; private records and workstation details do not
belong in this repository.[^owner]

The handoff is documentation-only. Validate frontmatter, local links, and the
staged whitespace diff before committing. Source tests are documented below for
future implementation changes; no new Rust or Linux runtime result is claimed.
Inspect live Git status and remote SHA for publication state rather than treating
this file as a push receipt.

# Development checks and next work

```sh
cargo build --locked
cargo test --locked
```

Integration tests cover generation, invalid input, overwrite rules, clean behavior,
and symlink safety. CMake/Ninja workflow tests return early if those commands are
unavailable; a C++ compiler is also needed for real generated-project builds.
Report which tool-dependent paths actually ran.[^overview][^tests]

For future implementation, establish the native toolchain and run the relevant
checks on the selected OS. Rebuild ignored output separately on macOS and Linux.
Follow [AGENTS.md](../AGENTS.md) for clean-branch pulls, deliberate conflict
resolution, reviewed commits, and remote verification before switching machines.

[^owner]: Explicit owner decision to keep cppbro public and subsequent authorization to push the remaining reviewed changes.
[^overview]: Existing project README; package availability was not independently checked.
[^manifest]: Existing package metadata; this update does not change dependencies.
[^tests]: Existing integration-test source and conditional external-tool checks; no new execution is claimed.

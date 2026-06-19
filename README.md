# cppbro

`cppbro` is a friendly Rust command-line tool, C++ project generator, and
modern CMake starter for creating, building, running, and cleaning small C++
projects. It includes focused templates for minimal apps, command-line tools,
and embedded-style programs.

It generates a minimal source-only C++ application using target-based CMake,
Ninja build commands, `compile_commands.json` for clangd, a `.clang-format`
file, and defaults that work well for Neovim, terminal-first workflows, and
systems programming practice.

Common search terms this project intentionally fits: C++ project generator,
CMake project template, C++ starter project, modern CMake CLI, Ninja C++ build,
clangd setup, Neovim C++ setup, `compile_commands.json`, clang-format starter,
C++ CLI template, and embedded C++ starter.

## Why this tool exists

Starting a small C++ project should not require copying an old directory,
remembering CMake boilerplate, or deleting IDE-generated files. `cppbro`
creates the few files a modern C++ executable needs and leaves the rest to you.

It is intentionally minimal: no test framework, no external C++ dependencies,
no generated build directory, and no hidden framework.

## Generated project structure

```text
MyProject/
├── CMakeLists.txt
├── .clang-format
├── .gitignore
├── README.md
└── src/
    └── main.cpp
```

## Installation

### macOS

```bash
brew install Roozbeh-Sdtz/tap/cppbro
```

### Linux

```bash
cargo install cppbro
```

### Windows

```powershell
cargo install cppbro
```

`cppbro build` and `cppbro run` use CMake and Ninja. The Homebrew package
installs those dependencies automatically; Linux and Windows users should have
CMake, Ninja, and a C++ compiler installed for build/run commands.

For local development from this repository:

```bash
cargo install --path .
```

## Quick Start

Create a project:

```bash
cppbro MyProject
cd MyProject
```

Build it:

```bash
cppbro build
```

Run it:

```bash
cppbro run
```

Clean generated build artifacts:

```bash
cppbro clean
```

The manual equivalent on macOS or Linux is:

```bash
cmake -S . -B build -G Ninja -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
ln -sf build/compile_commands.json compile_commands.json
cmake --build build -j
./build/myproject
rm -rf build compile_commands.json
```

On Windows with PowerShell, Ninja, and a configured C++ compiler:

```powershell
cmake -S . -B build -G Ninja -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
Copy-Item build/compile_commands.json compile_commands.json
cmake --build build -j
.\build\myproject.exe
```

## CLI Usage

```bash
cppbro <project-name>
cppbro new <project-name>
cppbro <project-name> --target <target-name>
cppbro <project-name> --template minimal
cppbro <project-name> --template cli
cppbro <project-name> --template embedded
cppbro <project-name> --std 17
cppbro <project-name> --std 20
cppbro <project-name> --std 23
cppbro <project-name> --force
cppbro <project-name> --no-readme
cppbro <project-name> --no-clang-format
cppbro <project-name> --no-gitignore
cppbro build
cppbro build --build-dir build
cppbro run
cppbro run --target <target-name>
cppbro run -- --program-arg
cppbro clean
cppbro clean --build-dir build
```

Defaults:

- C++ standard: `20`
- Executable target: normalized project name in lowercase
- Project template: `minimal`
- Build directory: `build`
- Existing non-empty directories are rejected unless `--force` is used
- `clean` removes only the configured build directory and root `compile_commands.json`

## Templates

`cppbro` keeps templates small and source-only. They are starting points, not
frameworks.

| Template | Use case | Generated structure |
| --- | --- | --- |
| `minimal` | Small experiments, coding practice, examples, and learning CMake | `src/main.cpp` |
| `cli` | Command-line tools that need a place for argument handling and app logic | `src/main.cpp`, `src/app.cpp`, `include/<target>/app.hpp` |
| `embedded` | Embedded-style programs that avoid iostreams and separate platform code | `src/main.cpp`, `src/platform.cpp`, `include/<target>/platform.hpp` |

## Examples

```bash
cppbro "Baseball Game"
```

Creates `Baseball_Game/` with a default executable target named
`baseball_game`.

```bash
cppbro Tools --target tools_cli --std 23
```

Creates `Tools/` using a custom executable target and C++23.

```bash
cppbro "Image Tool" --template cli
```

Creates a small command-line application skeleton with `run(argc, argv)`.

```bash
cppbro "Sensor Firmware" --template embedded
```

Creates an embedded-style skeleton with a tiny `platform` namespace and no
standard I/O in `main.cpp`.

```bash
cd Tools
cppbro build
cppbro run
cppbro clean
```

Configures, builds, runs, and cleans the generated executable project.

## Generated CMake Philosophy

`cppbro` uses modern target-based CMake. Compile features and warning flags are
attached to the executable target instead of being set globally. This keeps the
project easy to extend when more targets are added later.

The generated project does not include build files. Build directories and
`compile_commands.json` are produced by CMake when you configure the project.
`cppbro clean` removes those generated artifacts without touching source files.

## Neovim and clangd Workflow

clangd understands C++ projects best when it can read
`compile_commands.json`. `cppbro build` configures CMake with
`-DCMAKE_EXPORT_COMPILE_COMMANDS=ON` and places `compile_commands.json` at the
project root.

Neovim users can then rely on clangd for completion, diagnostics,
go-to-definition, and compile flag awareness without extra project
configuration.

## Example Generated CMakeLists.txt

```cmake
cmake_minimum_required(VERSION 3.20)
project(Baseball_Game
    VERSION 0.1.0
    LANGUAGES CXX
)

add_executable(baseball_game
    src/main.cpp
)

target_compile_features(baseball_game PRIVATE
    cxx_std_20
)

target_compile_options(baseball_game PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
```

## Example Generated main.cpp

```cpp
#include <iostream>

int main() {
    std::cout << "Hello, C++\n";
    return 0;
}
```

## Design Principles

- Generate a professional starting point, not a framework.
- Keep CMake explicit, target-based, and readable.
- Prefer editor and toolchain compatibility over cleverness.
- Avoid external C++ dependencies.
- Make output deterministic.
- Keep build and run commands as thin wrappers around standard CMake/Ninja.
- Keep clean behavior conservative and scoped to generated build artifacts.

## Development

```bash
cargo build
cargo test
cargo run -- MyProject
cargo run -- build
cargo run -- run
cargo run -- clean
cargo install --path .
```

## Testing

The integration tests run the compiled `cppbro` binary in temporary
directories and verify generated files, normalization, standards, force
behavior, invalid input handling, and the CMake build/run workflow when CMake
and Ninja are installed. They also cover clean behavior and symlink safety.

```bash
cargo test
```

## Roadmap

- Homebrew tap formula.
- crates.io release.
- winget package manifest.
- Optional library target generation.
- Optional test target generation.
- Optional `.clangd` generation.
- Shell completions.

## License

MIT. See [LICENSE](LICENSE).

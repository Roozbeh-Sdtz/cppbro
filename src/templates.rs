/// Supported project skeleton templates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectTemplate {
    Minimal,
    Cli,
    Embedded,
}

impl ProjectTemplate {
    pub fn name(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::Cli => "cli",
            Self::Embedded => "embedded",
        }
    }

    fn sources(self) -> &'static [&'static str] {
        match self {
            Self::Minimal => &["src/main.cpp"],
            Self::Cli => &["src/main.cpp", "src/app.cpp"],
            Self::Embedded => &["src/main.cpp", "src/platform.cpp"],
        }
    }

    fn uses_include_dir(self) -> bool {
        matches!(self, Self::Cli | Self::Embedded)
    }
}

/// Render a target-based CMake configuration for the generated executable.
pub fn render_cmake(
    project_name: &str,
    target_name: &str,
    cpp_standard: u16,
    project_template: ProjectTemplate,
) -> String {
    let sources = project_template
        .sources()
        .iter()
        .map(|source| format!("    {source}"))
        .collect::<Vec<_>>()
        .join("\n");
    let include_directories = if project_template.uses_include_dir() {
        format!(
            r#"

target_include_directories({target_name} PRIVATE
    include
)
"#
        )
    } else {
        String::new()
    };

    format!(
        r#"cmake_minimum_required(VERSION 3.20)
project({project_name}
    VERSION 0.1.0
    LANGUAGES CXX
)

add_executable({target_name}
{sources}
){include_directories}

target_compile_features({target_name} PRIVATE
    cxx_std_{cpp_standard}
)

target_compile_options({target_name} PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
"#
    )
}

/// Render the generated minimal C++ entry point.
pub fn render_minimal_main_cpp() -> &'static str {
    r#"#include <iostream>

int main() {
    std::cout << "Hello, C++\n";
    return 0;
}
"#
}

/// Render the command-line template entry point.
pub fn render_cli_main_cpp(target_name: &str) -> String {
    format!(
        r#"#include "{target_name}/app.hpp"

int main(int argc, char* argv[]) {{
    return run(argc, argv);
}}
"#
    )
}

/// Render the command-line template app header.
pub fn render_cli_app_hpp() -> &'static str {
    r#"#pragma once

int run(int argc, char* argv[]);
"#
}

/// Render the command-line template app implementation.
pub fn render_cli_app_cpp(target_name: &str) -> String {
    format!(
        r#"#include "{target_name}/app.hpp"

#include <iostream>
#include <string_view>

namespace {{

void print_usage(std::string_view program_name) {{
    std::cout << "Usage: " << program_name << " [--help]\n";
}}

}} // namespace

int run(int argc, char* argv[]) {{
    const auto program_name = argc > 0 ? std::string_view{{argv[0]}} : std::string_view{{"{target_name}"}};

    if (argc > 1 && std::string_view{{argv[1]}} == "--help") {{
        print_usage(program_name);
        return 0;
    }}

    std::cout << "Hello from {target_name}\n";
    return 0;
}}
"#
    )
}

/// Render the embedded template entry point.
pub fn render_embedded_main_cpp(target_name: &str) -> String {
    format!(
        r#"#include "{target_name}/platform.hpp"

int main() {{
    platform::initialize();
    platform::set_status_led(platform::LedState::on);

    for (auto i = 0U; i < 1000U; ++i) {{
        static_cast<void>(platform::ticks());
    }}

    platform::set_status_led(platform::LedState::off);
    return 0;
}}
"#
    )
}

/// Render the embedded template platform header.
pub fn render_embedded_platform_hpp() -> &'static str {
    r#"#pragma once

#include <cstdint>

namespace platform {

enum class LedState : std::uint8_t {
    off,
    on,
};

void initialize();
void set_status_led(LedState state);
std::uint32_t ticks();

} // namespace platform
"#
}

/// Render the embedded template platform implementation.
pub fn render_embedded_platform_cpp(target_name: &str) -> String {
    format!(
        r#"#include "{target_name}/platform.hpp"

namespace {{

std::uint32_t simulated_ticks = 0;
platform::LedState status_led = platform::LedState::off;

}} // namespace

namespace platform {{

void initialize() {{
    simulated_ticks = 0;
    status_led = LedState::off;
}}

void set_status_led(LedState state) {{
    status_led = state;
}}

std::uint32_t ticks() {{
    return ++simulated_ticks;
}}

}} // namespace platform
"#
    )
}

/// Render the generated clang-format configuration.
pub fn render_clang_format() -> &'static str {
    r#"BasedOnStyle: LLVM
IndentWidth: 4
ColumnLimit: 100
AllowShortFunctionsOnASingleLine: Empty
"#
}

/// Render the generated C++ project gitignore.
pub fn render_gitignore() -> &'static str {
    r#"build/
compile_commands.json
.DS_Store
"#
}

/// Render the generated C++ project README.
pub fn render_project_readme(
    project_name: &str,
    target_name: &str,
    project_template: ProjectTemplate,
) -> String {
    format!(
        r#"# {project_name}

Template: `{template_name}`

## Build

```bash
cppbro build
```

## Run

```bash
cppbro run
```

## Clean

```bash
cppbro clean
```

Manual equivalent:

```bash
cmake -S . -B build -G Ninja -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
ln -sf build/compile_commands.json compile_commands.json
cmake --build build -j
./build/{target_name}
```
"#,
        template_name = project_template.name(),
    )
}

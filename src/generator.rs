use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::templates;
pub use crate::templates::ProjectTemplate;

const RESERVED_CMAKE_TARGETS: &[&str] = &[
    "all",
    "clean",
    "edit_cache",
    "help",
    "install",
    "package",
    "rebuild_cache",
    "test",
];

const RESERVED_WINDOWS_DEVICE_NAMES: &[&str] = &[
    "aux", "con", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8", "com9", "lpt1",
    "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9", "nul", "prn",
];

/// Options used by the project generator.
#[derive(Debug, Clone)]
pub struct ProjectOptions {
    pub project_name: String,
    pub target_name: Option<String>,
    pub cpp_standard: u16,
    pub project_template: ProjectTemplate,
    pub force: bool,
    pub include_readme: bool,
    pub include_clang_format: bool,
    pub include_gitignore: bool,
}

/// Summary of a created project.
#[derive(Debug, Clone)]
pub struct CreatedProject {
    pub project_dir: PathBuf,
    pub cmake_project_name: String,
    pub target_name: String,
}

struct GeneratedFile {
    relative_path: String,
    contents: String,
}

/// Create a C++ project in the current working directory.
pub fn create_project(options: &ProjectOptions) -> Result<CreatedProject> {
    validate_safe_project_input(&options.project_name)?;
    validate_cpp_standard(options.cpp_standard)?;

    let cmake_project_name = normalize_project_name(&options.project_name)?;
    let target_name = match &options.target_name {
        Some(target_name) => normalize_target_name(target_name)?,
        None => normalize_target_name(&cmake_project_name)?,
    };
    validate_cmake_target_name(&target_name)?;

    let project_dir = env::current_dir()
        .context("failed to determine the current directory")?
        .join(&cmake_project_name);

    let files = generated_files(
        &cmake_project_name,
        &target_name,
        options.cpp_standard,
        options.project_template,
        options.include_readme,
        options.include_clang_format,
        options.include_gitignore,
    );

    preflight_project_dir(&project_dir, &files, options.force)?;
    for file in files {
        let path = project_dir.join(&file.relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }

        fs::write(&path, file.contents)
            .with_context(|| format!("failed to write {}", path.display()))?;
    }

    Ok(CreatedProject {
        project_dir,
        cmake_project_name,
        target_name,
    })
}

/// Normalize a user-facing project name into a safe project directory name.
pub fn normalize_project_name(input: &str) -> Result<String> {
    let normalized = normalize_name(input, LetterCase::Preserve);
    ensure_normalized_name(&normalized, "project name")?;
    Ok(normalized)
}

/// Normalize a project or target name into a lowercase executable target name.
pub fn normalize_target_name(input: &str) -> Result<String> {
    if input.contains('/') || input.contains('\\') {
        bail!("target name must not contain path separators");
    }

    let normalized = normalize_name(input, LetterCase::Lowercase);
    ensure_normalized_name(&normalized, "target name")?;
    Ok(normalized)
}

/// Validate that the requested C++ standard is supported.
pub fn validate_cpp_standard(cpp_standard: u16) -> Result<()> {
    match cpp_standard {
        17 | 20 | 23 => Ok(()),
        _ => {
            bail!("unsupported C++ standard {cpp_standard}; supported standards are 17, 20, and 23")
        }
    }
}

/// Validate that the raw project input is not a path and can normalize safely.
pub fn validate_safe_project_input(input: &str) -> Result<()> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        bail!("project name cannot be empty");
    }

    if trimmed == "." || trimmed == ".." {
        bail!("project name must not be '.' or '..'");
    }

    if input.contains('/') || input.contains('\\') {
        bail!("project name must not contain path separators");
    }

    normalize_project_name(input).map(|_| ())
}

fn generated_files(
    project_name: &str,
    target_name: &str,
    cpp_standard: u16,
    project_template: ProjectTemplate,
    include_readme: bool,
    include_clang_format: bool,
    include_gitignore: bool,
) -> Vec<GeneratedFile> {
    let mut files = vec![GeneratedFile {
        relative_path: "CMakeLists.txt".to_owned(),
        contents: templates::render_cmake(
            project_name,
            target_name,
            cpp_standard,
            project_template,
        ),
    }];

    match project_template {
        ProjectTemplate::Minimal => {
            files.push(GeneratedFile {
                relative_path: "src/main.cpp".to_owned(),
                contents: templates::render_minimal_main_cpp().to_owned(),
            });
        }
        ProjectTemplate::Cli => {
            files.extend([
                GeneratedFile {
                    relative_path: "src/main.cpp".to_owned(),
                    contents: templates::render_cli_main_cpp(target_name),
                },
                GeneratedFile {
                    relative_path: "src/app.cpp".to_owned(),
                    contents: templates::render_cli_app_cpp(target_name),
                },
                GeneratedFile {
                    relative_path: format!("include/{target_name}/app.hpp"),
                    contents: templates::render_cli_app_hpp().to_owned(),
                },
            ]);
        }
        ProjectTemplate::Embedded => {
            files.extend([
                GeneratedFile {
                    relative_path: "src/main.cpp".to_owned(),
                    contents: templates::render_embedded_main_cpp(target_name),
                },
                GeneratedFile {
                    relative_path: "src/platform.cpp".to_owned(),
                    contents: templates::render_embedded_platform_cpp(target_name),
                },
                GeneratedFile {
                    relative_path: format!("include/{target_name}/platform.hpp"),
                    contents: templates::render_embedded_platform_hpp().to_owned(),
                },
            ]);
        }
    }

    if include_clang_format {
        files.push(GeneratedFile {
            relative_path: ".clang-format".to_owned(),
            contents: templates::render_clang_format().to_owned(),
        });
    }

    if include_gitignore {
        files.push(GeneratedFile {
            relative_path: ".gitignore".to_owned(),
            contents: templates::render_gitignore().to_owned(),
        });
    }

    if include_readme {
        files.push(GeneratedFile {
            relative_path: "README.md".to_owned(),
            contents: templates::render_project_readme(project_name, target_name, project_template),
        });
    }

    files
}

fn preflight_project_dir(project_dir: &Path, files: &[GeneratedFile], force: bool) -> Result<()> {
    if let Some(metadata) = metadata_if_exists(project_dir)? {
        if metadata.file_type().is_symlink() {
            bail!(
                "{} is a symbolic link; refusing to write through it",
                project_dir.display()
            );
        }

        if !metadata.is_dir() {
            bail!("{} exists and is not a directory", project_dir.display());
        }
    }

    if project_dir.is_dir() && !force && !is_directory_empty(project_dir)? {
        bail!(
            "{} already exists and is not empty; use --force to overwrite cppbro-managed files",
            project_dir.display()
        );
    }

    for file in files {
        let path = project_dir.join(&file.relative_path);

        if let Some(parent) = path.parent() {
            if let Some(metadata) = metadata_if_exists(parent)? {
                if metadata.file_type().is_symlink() {
                    bail!(
                        "{} is a symbolic link; refusing to write through it",
                        parent.display()
                    );
                }

                if !metadata.is_dir() {
                    bail!("{} exists and is not a directory", parent.display());
                }
            }
        }

        if let Some(metadata) = metadata_if_exists(&path)? {
            if metadata.file_type().is_symlink() {
                bail!(
                    "{} is a symbolic link; refusing to overwrite it",
                    path.display()
                );
            }

            if metadata.is_dir() {
                bail!("{} exists and is a directory", path.display());
            }

            if !force {
                bail!(
                    "{} already exists; use --force to overwrite it",
                    path.display()
                );
            }
        }
    }

    Ok(())
}

fn metadata_if_exists(path: &Path) -> Result<Option<fs::Metadata>> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => Ok(Some(metadata)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("failed to inspect {}", path.display())),
    }
}

fn is_directory_empty(path: &Path) -> Result<bool> {
    let mut entries =
        fs::read_dir(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(entries.next().is_none())
}

fn validate_cmake_target_name(target_name: &str) -> Result<()> {
    if RESERVED_CMAKE_TARGETS.contains(&target_name) {
        bail!("target name '{target_name}' is reserved by CMake");
    }

    Ok(())
}

fn ensure_normalized_name(name: &str, label: &str) -> Result<()> {
    if name.is_empty() {
        bail!("{label} normalizes to an empty name");
    }

    if name == "." || name == ".." {
        bail!("{label} must not normalize to '.' or '..'");
    }

    if !name
        .chars()
        .any(|character| character.is_ascii_alphanumeric())
    {
        bail!("{label} must contain at least one letter or number");
    }

    if is_reserved_windows_device_name(name) {
        bail!("{label} '{name}' is reserved on Windows");
    }

    Ok(())
}

fn is_reserved_windows_device_name(name: &str) -> bool {
    let lowercase = name.to_ascii_lowercase();
    RESERVED_WINDOWS_DEVICE_NAMES.contains(&lowercase.as_str())
}

#[derive(Debug, Clone, Copy)]
enum LetterCase {
    Preserve,
    Lowercase,
}

fn normalize_name(input: &str, letter_case: LetterCase) -> String {
    let mut normalized = String::new();
    let mut previous_was_underscore = false;

    for character in input.trim().chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            let character = match letter_case {
                LetterCase::Preserve => character,
                LetterCase::Lowercase => character.to_ascii_lowercase(),
            };
            normalized.push(character);
            previous_was_underscore = false;
        } else if character.is_whitespace() || character == '-' {
            if !previous_was_underscore && !normalized.is_empty() {
                normalized.push('_');
                previous_was_underscore = true;
            }
        }
    }

    while normalized.ends_with('_') {
        normalized.pop();
    }

    normalized
}

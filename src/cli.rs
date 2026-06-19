use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::generator::ProjectTemplate;

/// Command-line options for cppbro.
#[derive(Debug, Parser)]
#[command(
    name = "cppbro",
    version,
    about = "Initialize, build, and run clean modern C++ projects",
    long_about = "Initialize a minimal C++ executable project with modern CMake, clang-format, and clangd-friendly build defaults. It can also build and run generated projects with short commands."
)]
pub struct Cli {
    /// Subcommand to run. Omit it to create a new project directly.
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Project creation arguments used when no subcommand is provided.
    #[command(flatten)]
    pub new_project: DirectNewProjectArgs,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a new C++ project.
    New(NewProjectArgs),

    /// Configure and build the current C++ project.
    Build(BuildArgs),

    /// Configure, build, and run the current C++ project.
    Run(RunArgs),

    /// Remove generated build artifacts.
    Clean(CleanArgs),
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TemplateArg {
    Minimal,
    Cli,
    Embedded,
}

impl From<TemplateArg> for ProjectTemplate {
    fn from(template: TemplateArg) -> Self {
        match template {
            TemplateArg::Minimal => Self::Minimal,
            TemplateArg::Cli => Self::Cli,
            TemplateArg::Embedded => Self::Embedded,
        }
    }
}

/// Arguments accepted by `cppbro <project-name>`.
#[derive(Debug, Args)]
pub struct DirectNewProjectArgs {
    /// Project name to create.
    #[arg(value_name = "PROJECT_NAME")]
    pub project_name: Option<String>,

    /// Executable target name. Defaults to the normalized project name.
    #[arg(long, value_name = "TARGET_NAME")]
    pub target: Option<String>,

    /// C++ standard to use: 17, 20, or 23.
    #[arg(long = "std", value_name = "STANDARD", default_value_t = 20)]
    pub cpp_standard: u16,

    /// Project skeleton template to generate.
    #[arg(long, value_enum, default_value_t = TemplateArg::Minimal)]
    pub template: TemplateArg,

    /// Allow generation in an existing directory and overwrite cppbro-managed files.
    #[arg(long)]
    pub force: bool,

    /// Do not generate a project README.md.
    #[arg(long)]
    pub no_readme: bool,

    /// Do not generate .clang-format.
    #[arg(long)]
    pub no_clang_format: bool,

    /// Do not generate .gitignore.
    #[arg(long)]
    pub no_gitignore: bool,
}

/// Arguments accepted by `cppbro new <project-name>`.
#[derive(Debug, Args)]
pub struct NewProjectArgs {
    /// Project name to create.
    #[arg(value_name = "PROJECT_NAME")]
    pub project_name: String,

    /// Executable target name. Defaults to the normalized project name.
    #[arg(long, value_name = "TARGET_NAME")]
    pub target: Option<String>,

    /// C++ standard to use: 17, 20, or 23.
    #[arg(long = "std", value_name = "STANDARD", default_value_t = 20)]
    pub cpp_standard: u16,

    /// Project skeleton template to generate.
    #[arg(long, value_enum, default_value_t = TemplateArg::Minimal)]
    pub template: TemplateArg,

    /// Allow generation in an existing directory and overwrite cppbro-managed files.
    #[arg(long)]
    pub force: bool,

    /// Do not generate a project README.md.
    #[arg(long)]
    pub no_readme: bool,

    /// Do not generate .clang-format.
    #[arg(long)]
    pub no_clang_format: bool,

    /// Do not generate .gitignore.
    #[arg(long)]
    pub no_gitignore: bool,
}

/// Arguments accepted by `cppbro build`.
#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Build directory to use.
    #[arg(long, value_name = "DIR", default_value = "build")]
    pub build_dir: PathBuf,
}

/// Arguments accepted by `cppbro run`.
#[derive(Debug, Args)]
pub struct RunArgs {
    /// Build directory to use.
    #[arg(long, value_name = "DIR", default_value = "build")]
    pub build_dir: PathBuf,

    /// Executable target to run. Defaults to the first add_executable target in CMakeLists.txt.
    #[arg(long, value_name = "TARGET_NAME")]
    pub target: Option<String>,

    /// Arguments passed to the C++ executable.
    #[arg(last = true, value_name = "ARGS")]
    pub program_args: Vec<String>,
}

/// Arguments accepted by `cppbro clean`.
#[derive(Debug, Args)]
pub struct CleanArgs {
    /// Build directory to remove.
    #[arg(long, value_name = "DIR", default_value = "build")]
    pub build_dir: PathBuf,
}

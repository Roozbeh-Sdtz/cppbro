mod cli;
mod generator;
mod templates;
mod workflow;

use anyhow::{bail, Result};
use clap::Parser;

use crate::cli::{Cli, Command, DirectNewProjectArgs, NewProjectArgs};
use crate::generator::{create_project, ProjectOptions};
use crate::workflow::{
    build_project, clean_project, run_project, BuildOptions, CleanOptions, RunOptions,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::New(args)) => create_new_project(args.into())?,
        Some(Command::Build(args)) => build_project(&BuildOptions {
            build_dir: args.build_dir,
        })?,
        Some(Command::Run(args)) => run_project(&RunOptions {
            build_dir: args.build_dir,
            target_name: args.target,
            program_args: args.program_args,
        })?,
        Some(Command::Clean(args)) => clean_project(&CleanOptions {
            build_dir: args.build_dir,
        })?,
        None => create_new_project(cli.new_project.try_into()?)?,
    }

    Ok(())
}

fn create_new_project(options: ProjectOptions) -> Result<()> {
    let project = create_project(&options)?;

    println!("Created C++ project at {}", project.project_dir.display());
    println!("CMake project: {}", project.cmake_project_name);
    println!("Executable target: {}", project.target_name);

    Ok(())
}

impl From<NewProjectArgs> for ProjectOptions {
    fn from(args: NewProjectArgs) -> Self {
        Self {
            project_name: args.project_name,
            target_name: args.target,
            cpp_standard: args.cpp_standard,
            project_template: args.template.into(),
            force: args.force,
            include_readme: !args.no_readme,
            include_clang_format: !args.no_clang_format,
            include_gitignore: !args.no_gitignore,
        }
    }
}

impl TryFrom<DirectNewProjectArgs> for ProjectOptions {
    type Error = anyhow::Error;

    fn try_from(args: DirectNewProjectArgs) -> Result<Self> {
        let Some(project_name) = args.project_name else {
            bail!("provide a project name, or use a subcommand such as 'build', 'run', or 'clean'");
        };

        Ok(Self {
            project_name,
            target_name: args.target,
            cpp_standard: args.cpp_standard,
            project_template: args.template.into(),
            force: args.force,
            include_readme: !args.no_readme,
            include_clang_format: !args.no_clang_format,
            include_gitignore: !args.no_gitignore,
        })
    }
}

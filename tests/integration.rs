use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use tempfile::TempDir;

fn run_cppbro(cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cppbro"))
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("failed to run cppbro")
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "expected success\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_failure(output: &Output) {
    assert!(
        !output.status.success(),
        "expected failure\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn command_available(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[test]
fn basic_project_generation_creates_expected_files() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["MyProject"]);
    assert_success(&output);

    let project = temp.path().join("MyProject");
    assert!(project.join("CMakeLists.txt").is_file());
    assert!(project.join(".clang-format").is_file());
    assert!(project.join(".gitignore").is_file());
    assert!(project.join("README.md").is_file());
    assert!(project.join("src/main.cpp").is_file());

    let main_cpp = fs::read_to_string(project.join("src/main.cpp")).unwrap();
    assert!(main_cpp.contains("Hello, C++"));

    let cmake = fs::read_to_string(project.join("CMakeLists.txt")).unwrap();
    assert!(cmake.contains("project(MyProject"));
    assert!(cmake.contains("add_executable(myproject"));
    assert!(cmake.contains("cxx_std_20"));
}

#[test]
fn new_subcommand_creates_project() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["new", "SubcommandProject", "--std", "23"]);
    assert_success(&output);

    let cmake = fs::read_to_string(temp.path().join("SubcommandProject/CMakeLists.txt")).unwrap();
    assert!(cmake.contains("project(SubcommandProject"));
    assert!(cmake.contains("cxx_std_23"));
}

#[test]
fn cli_template_generates_command_line_structure() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["Image Tool", "--template", "cli"]);
    assert_success(&output);

    let project = temp.path().join("Image_Tool");
    assert!(project.join("src/main.cpp").is_file());
    assert!(project.join("src/app.cpp").is_file());
    assert!(project.join("include/image_tool/app.hpp").is_file());

    let cmake = fs::read_to_string(project.join("CMakeLists.txt")).unwrap();
    assert!(cmake.contains("src/app.cpp"));
    assert!(cmake.contains("target_include_directories(image_tool PRIVATE"));

    let main_cpp = fs::read_to_string(project.join("src/main.cpp")).unwrap();
    assert!(main_cpp.contains("#include \"image_tool/app.hpp\""));

    let readme = fs::read_to_string(project.join("README.md")).unwrap();
    assert!(readme.contains("Template: `cli`"));
}

#[test]
fn embedded_template_generates_embedded_style_structure() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["Sensor Firmware", "--template", "embedded"]);
    assert_success(&output);

    let project = temp.path().join("Sensor_Firmware");
    assert!(project.join("src/main.cpp").is_file());
    assert!(project.join("src/platform.cpp").is_file());
    assert!(project
        .join("include/sensor_firmware/platform.hpp")
        .is_file());

    let cmake = fs::read_to_string(project.join("CMakeLists.txt")).unwrap();
    assert!(cmake.contains("src/platform.cpp"));
    assert!(cmake.contains("target_include_directories(sensor_firmware PRIVATE"));

    let main_cpp = fs::read_to_string(project.join("src/main.cpp")).unwrap();
    assert!(main_cpp.contains("#include \"sensor_firmware/platform.hpp\""));
    assert!(!main_cpp.contains("<iostream>"));

    let readme = fs::read_to_string(project.join("README.md")).unwrap();
    assert!(readme.contains("Template: `embedded`"));
}

#[test]
fn custom_target_name_controls_template_include_path() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(
        temp.path(),
        &["App", "--template", "cli", "--target", "app_cli"],
    );
    assert_success(&output);

    let project = temp.path().join("App");
    assert!(project.join("include/app_cli/app.hpp").is_file());

    let main_cpp = fs::read_to_string(project.join("src/main.cpp")).unwrap();
    assert!(main_cpp.contains("#include \"app_cli/app.hpp\""));
}

#[test]
fn invalid_template_is_rejected() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["BadTemplate", "--template", "desktop"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid value"));
}

#[test]
fn missing_project_name_is_rejected() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &[]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("provide a project name"));
}

#[test]
fn supported_cpp_standards_are_generated() {
    for standard in ["17", "20", "23"] {
        let temp = TempDir::new().unwrap();
        let project_name = format!("Std{standard}");
        let output = run_cppbro(temp.path(), &[&project_name, "--std", standard]);
        assert_success(&output);

        let cmake =
            fs::read_to_string(temp.path().join(project_name).join("CMakeLists.txt")).unwrap();
        assert!(cmake.contains(&format!("cxx_std_{standard}")));
    }
}

#[test]
fn invalid_cpp_standard_is_rejected() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["BadStandard", "--std", "14"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported C++ standard 14"));
}

#[test]
fn existing_non_empty_directory_is_rejected_without_force() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("Existing");
    fs::create_dir(&project).unwrap();
    fs::write(project.join("notes.txt"), "keep me").unwrap();

    let output = run_cppbro(temp.path(), &["Existing"]);
    assert_failure(&output);
    assert!(!project.join("CMakeLists.txt").exists());
    assert_eq!(
        fs::read_to_string(project.join("notes.txt")).unwrap(),
        "keep me"
    );
}

#[test]
fn force_allows_existing_directory_and_preserves_unrelated_files() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("Forced");
    fs::create_dir(&project).unwrap();
    fs::write(project.join("notes.txt"), "keep me").unwrap();

    let output = run_cppbro(temp.path(), &["Forced", "--force"]);
    assert_success(&output);
    assert!(project.join("CMakeLists.txt").is_file());
    assert_eq!(
        fs::read_to_string(project.join("notes.txt")).unwrap(),
        "keep me"
    );
}

#[test]
fn force_overwrites_managed_files_only() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("ForcedManaged");
    fs::create_dir(&project).unwrap();
    fs::create_dir(project.join("src")).unwrap();
    fs::write(project.join("CMakeLists.txt"), "old cmake").unwrap();
    fs::write(project.join("src/main.cpp"), "old main").unwrap();
    fs::write(project.join("notes.txt"), "keep me").unwrap();

    let output = run_cppbro(temp.path(), &["ForcedManaged", "--force"]);
    assert_success(&output);

    let cmake = fs::read_to_string(project.join("CMakeLists.txt")).unwrap();
    let main_cpp = fs::read_to_string(project.join("src/main.cpp")).unwrap();
    assert!(cmake.contains("project(ForcedManaged"));
    assert!(main_cpp.contains("Hello, C++"));
    assert_eq!(
        fs::read_to_string(project.join("notes.txt")).unwrap(),
        "keep me"
    );
}

#[test]
fn force_respects_disabled_outputs() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("KeepReadme");
    fs::create_dir(&project).unwrap();
    fs::write(project.join("README.md"), "existing readme").unwrap();

    let output = run_cppbro(temp.path(), &["KeepReadme", "--force", "--no-readme"]);
    assert_success(&output);

    assert_eq!(
        fs::read_to_string(project.join("README.md")).unwrap(),
        "existing readme"
    );
    assert!(project.join("CMakeLists.txt").is_file());
}

#[test]
fn no_readme_skips_project_readme() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["NoReadme", "--no-readme"]);
    assert_success(&output);

    assert!(!temp.path().join("NoReadme/README.md").exists());
}

#[test]
fn no_clang_format_skips_clang_format_file() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["NoFormat", "--no-clang-format"]);
    assert_success(&output);

    assert!(!temp.path().join("NoFormat/.clang-format").exists());
}

#[test]
fn no_gitignore_skips_gitignore_file() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["NoGitignore", "--no-gitignore"]);
    assert_success(&output);

    assert!(!temp.path().join("NoGitignore/.gitignore").exists());
}

#[test]
fn project_names_with_spaces_are_normalized() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["Baseball Game"]);
    assert_success(&output);

    let project = temp.path().join("Baseball_Game");
    assert!(project.is_dir());

    let cmake = fs::read_to_string(project.join("CMakeLists.txt")).unwrap();
    assert!(cmake.contains("project(Baseball_Game"));
    assert!(cmake.contains("add_executable(baseball_game"));
}

#[test]
fn project_names_with_hyphens_are_normalized() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["Arcade-Tools"]);
    assert_success(&output);

    let project = temp.path().join("Arcade_Tools");
    assert!(project.is_dir());

    let cmake = fs::read_to_string(project.join("CMakeLists.txt")).unwrap();
    assert!(cmake.contains("project(Arcade_Tools"));
    assert!(cmake.contains("add_executable(arcade_tools"));
}

#[test]
fn custom_target_name_is_used() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["Tools", "--target", "tools_cli"]);
    assert_success(&output);

    let cmake = fs::read_to_string(temp.path().join("Tools/CMakeLists.txt")).unwrap();
    assert!(cmake.contains("add_executable(tools_cli"));
    assert!(cmake.contains("target_compile_features(tools_cli PRIVATE"));
}

#[test]
fn target_name_path_separators_are_rejected() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["Tools", "--target", "../tools"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("target name must not contain path separators"));
}

#[test]
fn names_that_normalize_to_empty_are_rejected() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["--", "---"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("project name normalizes to an empty name"));
}

#[test]
fn reserved_cmake_target_names_are_rejected() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["Tools", "--target", "install"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("target name 'install' is reserved by CMake"));
}

#[test]
fn windows_reserved_project_names_are_rejected() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["CON"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("project name 'CON' is reserved on Windows"));
}

#[test]
fn windows_reserved_target_names_are_rejected() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["Device", "--target", "nul"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("target name 'nul' is reserved on Windows"));
}

#[test]
fn path_traversal_is_rejected() {
    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["../bad"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("project name must not contain path separators"));
    assert!(!temp.path().join("../bad").exists());
}

#[cfg(unix)]
#[test]
fn force_rejects_existing_project_directory_symlink() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let outside = temp.path().join("outside");
    fs::create_dir(&outside).unwrap();
    symlink(&outside, temp.path().join("LinkedProject")).unwrap();

    let output = run_cppbro(temp.path(), &["LinkedProject", "--force"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("is a symbolic link; refusing to write through it"));
    assert!(!outside.join("CMakeLists.txt").exists());
}

#[cfg(unix)]
#[test]
fn force_rejects_existing_managed_file_symlink() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let project = temp.path().join("LinkedFile");
    let outside = temp.path().join("outside.txt");
    fs::create_dir(&project).unwrap();
    fs::write(&outside, "outside").unwrap();
    symlink(&outside, project.join("CMakeLists.txt")).unwrap();

    let output = run_cppbro(temp.path(), &["LinkedFile", "--force"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("is a symbolic link; refusing to overwrite it"));
    assert_eq!(fs::read_to_string(outside).unwrap(), "outside");
}

#[cfg(unix)]
#[test]
fn force_rejects_existing_parent_directory_symlink() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let project = temp.path().join("LinkedSrc");
    let outside_src = temp.path().join("outside_src");
    fs::create_dir(&project).unwrap();
    fs::create_dir(&outside_src).unwrap();
    symlink(&outside_src, project.join("src")).unwrap();

    let output = run_cppbro(temp.path(), &["LinkedSrc", "--force"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("is a symbolic link; refusing to write through it"));
    assert!(!outside_src.join("main.cpp").exists());
}

#[test]
fn build_command_configures_and_builds_generated_project() {
    if !command_available("cmake") || !command_available("ninja") {
        return;
    }

    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["Buildable"]);
    assert_success(&output);

    let project = temp.path().join("Buildable");
    let output = run_cppbro(&project, &["build"]);
    assert_success(&output);

    assert!(project.join("compile_commands.json").exists());
    assert!(project.join("build/buildable").is_file());
}

#[test]
fn run_command_builds_and_runs_generated_project() {
    if !command_available("cmake") || !command_available("ninja") {
        return;
    }

    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["Runnable"]);
    assert_success(&output);

    let project = temp.path().join("Runnable");
    let output = run_cppbro(&project, &["run"]);
    assert_success(&output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello, C++"));
}

#[test]
fn cli_template_builds_and_runs() {
    if !command_available("cmake") || !command_available("ninja") {
        return;
    }

    let temp = TempDir::new().unwrap();
    let output = run_cppbro(temp.path(), &["Cli Buildable", "--template", "cli"]);
    assert_success(&output);

    let project = temp.path().join("Cli_Buildable");
    let output = run_cppbro(&project, &["run"]);
    assert_success(&output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello from cli_buildable"));
}

#[test]
fn embedded_template_builds_and_runs() {
    if !command_available("cmake") || !command_available("ninja") {
        return;
    }

    let temp = TempDir::new().unwrap();
    let output = run_cppbro(
        temp.path(),
        &["Embedded Buildable", "--template", "embedded"],
    );
    assert_success(&output);

    let project = temp.path().join("Embedded_Buildable");
    let output = run_cppbro(&project, &["run"]);
    assert_success(&output);
}

#[test]
fn clean_command_removes_default_artifacts_only() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("Cleanable");
    fs::create_dir(&project).unwrap();
    fs::create_dir(project.join("build")).unwrap();
    fs::write(
        project.join("CMakeLists.txt"),
        "add_executable(cleanable src/main.cpp)",
    )
    .unwrap();
    fs::write(project.join("build/cache.txt"), "generated").unwrap();
    fs::write(project.join("compile_commands.json"), "generated").unwrap();
    fs::write(project.join("notes.txt"), "keep me").unwrap();

    let output = run_cppbro(&project, &["clean"]);
    assert_success(&output);

    assert!(!project.join("build").exists());
    assert!(!project.join("compile_commands.json").exists());
    assert_eq!(
        fs::read_to_string(project.join("notes.txt")).unwrap(),
        "keep me"
    );
}

#[test]
fn clean_rejects_project_root() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("RootClean");
    fs::create_dir(&project).unwrap();
    fs::write(
        project.join("CMakeLists.txt"),
        "add_executable(rootclean src/main.cpp)",
    )
    .unwrap();

    let output = run_cppbro(&project, &["clean", "--build-dir", "."]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("refusing to clean the project root"));
    assert!(project.exists());
}

#[test]
fn clean_rejects_parent_directory_traversal() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("TraversalClean");
    fs::create_dir(&project).unwrap();
    fs::write(
        project.join("CMakeLists.txt"),
        "add_executable(traversalclean src/main.cpp)",
    )
    .unwrap();

    let output = run_cppbro(&project, &["clean", "--build-dir", "../outside"]);
    assert_failure(&output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("clean build directory must not contain '..'"));
}

#[cfg(unix)]
#[test]
fn clean_unlinks_build_directory_symlink_without_following_it() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let project = temp.path().join("SymlinkClean");
    let outside = temp.path().join("outside_build");
    fs::create_dir(&project).unwrap();
    fs::create_dir(&outside).unwrap();
    fs::write(
        project.join("CMakeLists.txt"),
        "add_executable(symlinkclean src/main.cpp)",
    )
    .unwrap();
    fs::write(outside.join("keep.txt"), "keep me").unwrap();
    symlink(&outside, project.join("build")).unwrap();

    let output = run_cppbro(&project, &["clean"]);
    assert_success(&output);

    assert!(!project.join("build").exists());
    assert_eq!(
        fs::read_to_string(outside.join("keep.txt")).unwrap(),
        "keep me"
    );
}

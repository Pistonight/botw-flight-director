use std::path::PathBuf;
use std::process::Command;

#[cfg(windows)]
const OBS_DEPS_DIR: &str = "obs/.deps/bin/64bit";
#[cfg(not(windows))]
compile_error!("Only windows is supported for now");

fn main() {
    // Ensure task installed
    let task = which::which("task").expect("task not found, please install it from https://taskfile.dev/");
    // Ensure OBS Deps are up to date
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root_dir = project_dir.parent().expect("Failed to get root directory");
    Command::new(task)
        .current_dir(root_dir)
        .args(&["obs:install"]).status().expect("Failed to install obs deps");
    // Tell Cargo about where obs.lib is
    let obs_deps_dir = root_dir.join(OBS_DEPS_DIR);
    println!("cargo:rustc-link-search=native={}", obs_deps_dir.display());
}


use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let vendored = env::var("CARGO_FEATURE_VENDORED").is_ok();

    if !vendored {
        let mut cfg = pkg_config::Config::new();
        if let Ok(lib) = cfg.probe("libvim") {
            for include in &lib.include_paths {
                println!("cargo:root={}", include.display());
            }
            return;
        }
    }
    println!("cargo:rustc-cfg=libvim_vendored");
    if !Path::new("libvim/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "libvim"])
            .status();
    }

    let mut child = Command::new("bash")
            .arg("-c")
            .arg("cd ./libvim/src && esy install && esy build")
            .spawn()
            .expect("Failed to libvim build");
    match child.wait() {
        Ok(output) => eprintln!("Status {}", output),
        Err(error) => panic!("Libvim build exit with error {}", error)
    }
}

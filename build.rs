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
    // println!("cargo:rustc-cfg=libvim_vendored");
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
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = Path::new(&dir).join("libvim/src/_esy/default/build/lib");
    let lib_include_path = Path::new(&dir).join("libvim/src/_esy/default/build/include");
    println!("cargo:rustc-link-lib=static={}", lib_path.join("libvim.a").display());
    cc::Build::new()
        .flag("-Wall")
        .flag("-Werror")
        .include(&lib_include_path)
        .include(lib_include_path.join("proto"))
        // TODO Check different OS
        // For now only darwin is supported
        .flag("-DHAVE_CONFIG_H")
        .flag("-DMACOS_X")
        .flag("-DMACOS_X_DARWIN")
        .cargo_metadata(true)
        .flag("-lvim")
        .flag("-lm")
        .flag("-lncurses")
        .flag("-liconv")
        .flag("framework AppKit").compile("libvim");

    //fs::copy("./libvim/src/_esy/default/build/libvim.a", "./lib/libvim.a").expect("Failed to copy library");
    //println!("cargo:rust-link-lib=static=libvim");
    //println!("cargo:rustc-flags=-L {}", Path::new(&dir).join("lib").display());

}

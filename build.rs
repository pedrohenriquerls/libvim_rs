use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let vendored = env::var("CARGO_FEATURE_VENDORED").is_ok();

    if !vendored {
        let cfg = pkg_config::Config::new();
        if let Ok(lib) = cfg.probe("vim") {
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
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = Path::new(&dir).join("libvim/src/_esy/default/build");
    println!("cargo:rustc-link-lib=intl");
    println!("cargo:rustc-link-lib=vim");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=ncurses");
    println!("cargo:rustc-link-lib=iconv");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-search={}", lib_path.display());
}

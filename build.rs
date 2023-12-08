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
    if !Path::new("libvim/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "libvim"])
            .status();
    } else {
        let _ = Command::new("git")
            .args(&["git submodule foreach --recursive git clean -xfd", "&&", "git submodule foreach --recursive git reset --hard"])
            .status();
    }

    println!("cargo:rustc-cfg=libvim_vendored");

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let source = Path::new(&dir);
    let lib_path = source.join("vim/build");

    if !lib_path.join("libvim.a").exists() {
        let mut child = Command::new("bash")
            .arg("-c")
            .arg(format!("cd ./libvim/src && cur__install={} ./build/build-posix.sh", lib_path.display()))
            .spawn()
            .expect("Failed to libvim build");
        match child.wait() {
            Ok(output) => eprintln!("Status {}", output),
            Err(error) => panic!("Libvim build exit with error {}", error)
        }

        let libvim_source = source.join("libvim/src");
        let mut child = Command::new("bash")
            .arg("-c")
            .arg(format!("cp {}/*.pro {}", libvim_source.join("proto").display(), libvim_source.display()))
            .spawn()
            .expect("Failed to libvim build");
        match child.wait() {
            Ok(output) => eprintln!("Status {}", output),
            Err(error) => panic!("Libvim build exit with error {}", error)
        }

        let clang_args = ["-DHAVE_CONFIG_H", "-DMACOS_X", "-DMACOS_X_DARWIN"];
        let bindings = bindgen::Builder::default()
            .header(libvim_source.join("libvim.h").to_str().expect("Return the heade file path"))
            .clang_args(clang_args)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(source.join("bindings.rs"))
            .expect("Couldn't write bindings!");

    }

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=vim");
        println!("cargo:rustc-link-lib=acl");
        println!("cargo:rustc-link-lib=ICE");
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=SM");
        println!("cargo:rustc-link-lib=framework=ncurses");
        println!("cargo:rustc-link-lib=framework=Xt");
    }

    if cfg!(target_os="macos") {
        println!("cargo:rustc-link-lib=intl");
        println!("cargo:rustc-link-lib=vim");
        println!("cargo:rustc-link-lib=m");
        println!("cargo:rustc-link-lib=ncurses");
        println!("cargo:rustc-link-lib=iconv");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-search={}", lib_path.display());
    }
}


use std::env;
use std::process::Command;

fn link_command(target: &str, path: String) {
    /*
    let args = match kind {
        "install" => vec!["install", "-buildmode=c-shared", "-linkshared", target],
        "build" => vec!["build", "-linkshared", target],
        _ => panic!("Unknown go build kind: {}", kind),
    };
    */

    let target_archive = format!("{}/lib{}.a", path, target);
    let ldflags = format!("-ldflags='-r {}/interfaces'", path);
    let output = Command::new("go")
        .args(&[
            "build",
            "-buildmode=c-shared",
            &ldflags,
            "-o", &target_archive,
            target
        ])
        .output()
        .expect(&format!("No output from go build: {}", target));

    if !output.status.success() {
        let s = String::from_utf8(output.stderr);
        if let Ok(err) = s {
            panic!("Go build failed: {}", err);
        }
    }
}

fn main() {
    // Partially build the library
    Command::new("cargo")
        .args(&["--lib", "--", "--crate-type=cdylib"])
        .output();

    if !partial.success() {
        let s = String::from_utf8(output.stderr);
        if let Ok(err) = s {
            panic!("Rust cdylib build failed: {}", err);
        }
    }

    let path = if let Ok(path) = env::var("CARGO_MANIFEST") {
        path
    }
    else {
        ".".to_string()
    };

    let path = format!("{}/golibs", path);
    //link_command("dhl", path.clone());

    println!("cargo:rustc-link-search={}", path);
}

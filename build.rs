use std::{path::PathBuf, process::Command};

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut shader_path = PathBuf::from(&manifest_dir);
    shader_path.push("src");
    shader_path.push("vshader.v.pica");
    println!("cargo:rerun-if-changed={}", shader_path.display());

    let mut out_path = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    out_path.push("src");
    out_path.push("assets");
    out_path.push("vshader.shbin");
    println!("cargo:rustc-env=VSHADER_BIN_PATH={}", out_path.display());

    std::fs::create_dir_all(out_path.parent().unwrap()).unwrap();

    let mut cmd = Command::new("picasso");
    cmd.arg(shader_path).arg("--out").arg(out_path);

    let status = cmd.spawn().unwrap().wait().unwrap();
    assert!(
        status.success(),
        "Command {cmd:#?} failed with code {status:?}"
    );
}

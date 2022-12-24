use bootloader_locator::locate_bootloader;
use std::{path::Path, process::Command};

pub fn main() {
    let bootloader_manifest = locate_bootloader("bootloader").unwrap();

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_dir = manifest_dir.parent().unwrap();

    let kernel_binary = Path::new("target/x86_64-unknown-none/debug/stellarium")
        .canonicalize()
        .unwrap();
    let kernel_manifest = kernel_dir.join("Cargo.toml");
    let target_dir = kernel_dir.join("target");
    let out_dir = kernel_binary.parent().unwrap();

    let mut build_command = Command::new(env!("CARGO"));

    build_command.arg("builder");
    build_command.arg("--kernel-manifest").arg(&kernel_manifest);
    build_command.arg("--kernel-binary").arg(&kernel_binary);
    build_command.arg("--target-dir").arg(&target_dir);
    build_command.arg("--out-dir").arg(&out_dir);

    let bootloader_dir = bootloader_manifest.parent().unwrap();
    build_command.current_dir(&bootloader_dir);

    let exit_status = build_command.status().unwrap();
    if !exit_status.success() {
        panic!("bootloader build failed");
    }
}

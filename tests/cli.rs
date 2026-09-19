use std::process::Command;

#[test]
fn binary_has_help() {
    let out = Command::new(env!("CARGO_BIN_EXE_snapmark")).arg("--help").output().unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("Local-first"));
}

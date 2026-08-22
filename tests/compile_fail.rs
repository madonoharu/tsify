use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

struct TempCrate(PathBuf);

impl TempCrate {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "tsify-rename-compile-fail-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(path.join("src")).expect("failed to create temporary test crate");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempCrate {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn invalid_container_rename_attributes_fail_to_compile() {
    let temp_crate = TempCrate::new();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dependency_path = manifest_dir.to_string_lossy().replace('\\', "\\\\");
    let manifest = format!(
        r#"[package]
name = "tsify-rename-compile-fail"
version = "0.0.0"
edition = "2021"

[dependencies]
tsify = {{ path = "{dependency_path}" }}

[workspace]
"#
    );
    fs::write(temp_crate.path().join("Cargo.toml"), manifest)
        .expect("failed to write temporary Cargo.toml");
    fs::write(
        temp_crate.path().join("src/lib.rs"),
        r#"use tsify::Tsify;

#[derive(Tsify)]
#[tsify(rename = "First", rename = "Second")]
struct DuplicateRename;

#[derive(Tsify)]
#[tsify(type_prefix = "Prefix", rename = "Renamed")]
struct RenameAfterPrefix;

#[derive(Tsify)]
#[tsify(rename = "Renamed", type_suffix = "Suffix")]
struct RenameBeforeSuffix;

#[derive(Tsify)]
#[tsify(rename = "not-valid")]
struct InvalidRename;
"#,
    )
    .expect("failed to write temporary lib.rs");

    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = Command::new(cargo)
        .args(["check", "--offline", "--quiet"])
        .current_dir(temp_crate.path())
        .env(
            "CARGO_TARGET_DIR",
            manifest_dir.join("target/tests/compile-fail"),
        )
        .output()
        .expect("failed to execute cargo check for compile-fail cases");
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success(), "fixture unexpectedly compiled");
    assert!(
        stderr.contains("duplicate attribute"),
        "missing duplicate rename diagnostic:\n{stderr}"
    );
    assert_eq!(
        stderr
            .matches("`rename` cannot be combined with `type_prefix` or `type_suffix`")
            .count(),
        2,
        "prefix/suffix conflicts should fail in either parse order:\n{stderr}"
    );
    assert!(
        stderr.contains("`rename` must be a valid TypeScript identifier"),
        "missing invalid identifier diagnostic:\n{stderr}"
    );
}

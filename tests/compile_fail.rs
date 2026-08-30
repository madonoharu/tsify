use indoc::{formatdoc, indoc};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

struct TempCrate(PathBuf);

struct CompileFailCase {
    module: &'static str,
    source: &'static str,
    line: usize,
    message: &'static str,
}

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

fn assert_diagnostic(stderr: &str, case: &CompileFailCase) {
    let error = format!("error: {}", case.message);
    let location = format!("--> src/{}.rs:{}:", case.module, case.line);
    let mut search_start = 0;

    while let Some(relative_start) = stderr[search_start..].find(&error) {
        let error_start = search_start + relative_start;
        let diagnostic = &stderr[error_start..];
        let diagnostic_end = diagnostic[1..]
            .find("\nerror:")
            .map_or(diagnostic.len(), |index| index + 1);
        let diagnostic = &diagnostic[..diagnostic_end];

        // rustc spells the path with the platform's separator, so the same
        // diagnostic points at `src/module.rs` on Unix and `src\module.rs` on
        // Windows. Compare one spelling.
        if diagnostic.replace('\\', "/").contains(&location) {
            return;
        }
        search_start = error_start + error.len();
    }

    panic!(
        "missing diagnostic for {} at line {}: {}\n{stderr}",
        case.module, case.line, case.message
    );
}

#[test]
fn invalid_rename_attributes_fail_to_compile() {
    let temp_crate = TempCrate::new();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dependency_path = manifest_dir.to_string_lossy().replace('\\', "\\\\");
    let manifest = formatdoc! {r#"
        [package]
        name = "tsify-rename-compile-fail"
        version = "0.0.0"
        edition = "2021"

        [dependencies]
        tsify = {{ path = "{dependency_path}" }}

        [workspace]
    "#};
    fs::write(temp_crate.path().join("Cargo.toml"), manifest)
        .expect("failed to write temporary Cargo.toml");
    let cases = [
        CompileFailCase {
            module: "duplicate_same_attribute",
            source: indoc! {r#"
                use tsify::Tsify;

                #[derive(Tsify)]
                #[tsify(rename = "First", rename = "Second")]
                struct DuplicateSameAttribute;
            "#},
            line: 4,
            message: "duplicate attribute",
        },
        CompileFailCase {
            module: "duplicate_separate_attributes",
            source: indoc! {r#"
                use tsify::Tsify;

                #[derive(Tsify)]
                #[tsify(rename = "First")]
                #[tsify(rename = "Second")]
                struct DuplicateSeparateAttributes;
            "#},
            line: 5,
            message: "duplicate attribute",
        },
        CompileFailCase {
            module: "empty",
            source: indoc! {r#"
                use tsify::Tsify;

                #[derive(Tsify)]
                #[tsify(rename = "")]
                struct Empty;
            "#},
            line: 4,
            message: "`rename` must be a valid TypeScript identifier",
        },
        CompileFailCase {
            module: "leading_digit",
            source: indoc! {r#"
                use tsify::Tsify;

                #[derive(Tsify)]
                #[tsify(rename = "1Name")]
                struct LeadingDigit;
            "#},
            line: 4,
            message: "`rename` must be a valid TypeScript identifier",
        },
        CompileFailCase {
            module: "invalid_character",
            source: indoc! {r#"
                use tsify::Tsify;

                #[derive(Tsify)]
                #[tsify(rename = "not-valid")]
                struct InvalidCharacter;
            "#},
            line: 4,
            message: "`rename` must be a valid TypeScript identifier",
        },
        CompileFailCase {
            module: "non_string",
            source: indoc! {r#"
                use tsify::Tsify;

                #[derive(Tsify)]
                #[tsify(rename = 123)]
                struct NonString;
            "#},
            line: 4,
            message: "expected string literal",
        },
        CompileFailCase {
            module: "value_missing",
            source: indoc! {r#"
                use tsify::Tsify;

                #[derive(Tsify)]
                #[tsify(rename)]
                struct ValueMissing;
            "#},
            line: 4,
            message: "expected `=`",
        },
        CompileFailCase {
            module: "rename_then_prefix",
            source: indoc! {r#"
                use tsify::Tsify;

                #[derive(Tsify)]
                #[tsify(rename = "Renamed", type_prefix = "Prefix")]
                struct RenameThenPrefix;
            "#},
            line: 4,
            message: "`rename` cannot be combined with `type_prefix` or `type_suffix`",
        },
        CompileFailCase {
            module: "prefix_then_rename",
            source: indoc! {r#"
                use tsify::Tsify;

                #[derive(Tsify)]
                #[tsify(type_prefix = "Prefix", rename = "Renamed")]
                struct PrefixThenRename;
            "#},
            line: 4,
            message: "`rename` cannot be combined with `type_prefix` or `type_suffix`",
        },
        CompileFailCase {
            module: "rename_then_suffix",
            source: indoc! {r#"
                use tsify::Tsify;

                #[derive(Tsify)]
                #[tsify(rename = "Renamed", type_suffix = "Suffix")]
                struct RenameThenSuffix;
            "#},
            line: 4,
            message: "`rename` cannot be combined with `type_prefix` or `type_suffix`",
        },
        CompileFailCase {
            module: "suffix_then_rename",
            source: indoc! {r#"
                use tsify::Tsify;

                #[derive(Tsify)]
                #[tsify(type_suffix = "Suffix", rename = "Renamed")]
                struct SuffixThenRename;
            "#},
            line: 4,
            message: "`rename` cannot be combined with `type_prefix` or `type_suffix`",
        },
    ];
    let modules = cases
        .iter()
        .map(|case| format!("mod {};", case.module))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(temp_crate.path().join("src/lib.rs"), modules)
        .expect("failed to write temporary lib.rs");
    for case in &cases {
        fs::write(
            temp_crate
                .path()
                .join("src")
                .join(format!("{}.rs", case.module)),
            case.source,
        )
        .unwrap_or_else(|error| panic!("failed to write {} fixture: {error}", case.module));
    }

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
    for case in &cases {
        assert_diagnostic(&stderr, case);
    }
}

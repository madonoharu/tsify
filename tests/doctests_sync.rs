//! Sync the crate's doctest with a folder, so their output can also be verified
//!
//!
#[ignore = "long test that builds wasm modules for all doctests"]
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::process::Command;

use serde::Deserialize;

const DOC_JSON: &str = "doctests.json";
const DOCTESTS_DIR: &str = "tests-doc";
const TOML: &str = "Cargo.toml";
const TOML_TEMPLATE: &str = "template.toml";
const ENTRY: &str = "entry_point.rs";
const REFERENCE: &str = "reference_output";

/// Doctests stuff from json
///
/// copied the schema from [here](https://doc.rust-lang.org/rustdoc/unstable-features.html#doctest)
/// with only relevant
#[derive(Debug, Deserialize)]
struct Doctests {
    format_version: u32,
    doctests: Vec<Doctest>,
}

#[derive(Debug, Deserialize)]
struct Doctest {
    file: String,
    /// will there be >4G line files?
    line: u32,
    doctest_attributes: Attributes,
    doctest_code: Code,
}

#[derive(Debug, Deserialize)]
struct Code {
    crate_level: String,
    code: String,
    wrapper: Wrapper,
}
impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        write!(
            f,
            "//! This file is auto-generated, please don't edit this file\n
            {}
            {}
            {}
            {}
            ",
            self.crate_level, self.wrapper.before, self.code, self.wrapper.after,
        )
    }
}

#[derive(Debug, Deserialize)]
struct Wrapper {
    before: String,
    after: String,
}

/// Attributes that indicate we don't need to include it
#[derive(Debug, Deserialize)]
struct Attributes {
    should_panic: bool,
    ignore: String,
    rust: bool,
    test_harness: bool,
    compile_fail: bool,
}

impl Attributes {
    fn should_build(&self) -> bool {
        !self.should_panic
            && self.ignore == "None"
            && self.rust
            && !self.test_harness
            && !self.compile_fail
    }
}

/// 1. Build json doctest output
/// 2. generate modules for every test
/// 3. build all tests
/// 4. compare outputs
#[test]
#[ignore = "Test that builds all doctests as wasm modules"]
fn doctests_are_synced() {
    // run rustdoc so we're testing against an up-to-date json
    let json_raw = Command::new("cargo")
        .args([
            "+nightly",
            "rustdoc",
            "-p",
            "tsify",
            "--lib",
            "--",
            "-Zunstable-options",
            "--output-format",
            "doctest",
        ])
        .output()
        .unwrap()
        .stdout;
    let json = &str::from_utf8(&json_raw).unwrap();

    let root = Path::new(DOCTESTS_DIR);

    // if the test is passing
    let mut passing = true;

    let Doctests {
        format_version,
        mut doctests,
    } = serde_json::from_str(json).unwrap();
    if format_version != 2 {
        eprintln!("format version is not 2");
        passing = false;
    }
    // otherwise it could be wrong?
    // assert_eq!(format_version, 2);

    // so the naming scheme is file-001 with the number increasing
    // so this will "fail" on a reordering within the file
    doctests.sort_by(|a, b| a.file.cmp(&b.file).then(a.line.cmp(&b.line)));
    let toml_template = fs::read_to_string(root.join(TOML_TEMPLATE)).expect(&format!(
        "Template Cargo.toml exists at {:?}",
        root.join(TOML)
    ));
    // number tests within a file
    for ftests in doctests.chunk_by(|a, b| a.file == b.file) {
        for (i, doctest) in ftests
            .iter()
            .filter(|d| d.doctest_attributes.should_build())
            .enumerate()
        {
            // create a folder
            // convert src/module/submodule.rs -> module-submodule-001
            let name = format!(
                "{}-{i:03}",
                doctest
                    .file
                    .strip_prefix("src/")
                    .unwrap()
                    .strip_suffix(".rs")
                    .unwrap()
                    .replace("\\/", &"-")
            );
            let test_dir = root.join(&name);
            fs::create_dir_all(test_dir.clone()).expect(&format!("{DOCTESTS_DIR:?} exists"));

            // check all Cargo.toml
            let toml = test_dir.join(TOML);
            let toml_contents = toml_template.replace("{name}", &name);
            fs::write(toml, &toml_contents).expect("can write toml");

            // check the actual code
            let entry_point = test_dir.join(ENTRY);
            let code = format!(
                "//! doctest at {}:{}{}\n",
                doctest.file,
                doctest.line,
                doctest.doctest_code.to_string().trim_end()
            );
            fs::write(entry_point, code.as_bytes()).expect("can write doctest");
        }
    }

    // Build all wasm modules
    assert!(Command::new(root.join("build_all.sh"))
        .status()
        .unwrap()
        .success());

    // verify output equality
    for ftests in doctests.chunk_by(|a, b| a.file == b.file) {
        for (i, doctest) in ftests
            .iter()
            .filter(|d| d.doctest_attributes.should_build())
            .enumerate()
        {
            let name = format!(
                "{}-{i:03}",
                doctest
                    .file
                    .strip_prefix("src/")
                    .unwrap()
                    .strip_suffix(".rs")
                    .unwrap()
                    .replace("\\/", &"-")
            );
            let test_dir = root.join(&name);
            // finally, I think we may check the reference output and for now also update it
            // That's because I'm lazy and just want to put all generated outputs in their directories
            // maybe this should really stop being a test now, but rather some executable that you can invoke with options
            // because we're really not using the testing harness much
            // not sure how to do it though
            let ref_dir = root.join(REFERENCE);
            // I think I'd rather just like the files to be in reference_output
            // not sure why it's such a nested structure in tests-e2e
            let out_name = name.replace("-", "_") + ".d.ts";
            let ref_file = ref_dir.join(&out_name);
            let out_file = test_dir.join("pkg").join(&out_name);
            if ref_file.is_file() {
                if fs::read_to_string(&ref_file).unwrap() != fs::read_to_string(&out_file).unwrap()
                {
                    eprintln!(
                        "{ref_file:?} does not match for {}:{}. Correct output is at {out_file:?}",
                        doctest.file, doctest.line
                    );
                    passing = false;
                }
            }
        }

        //
    }
    // generate examples/doctests/declare-001/...
    // compare with existing files
    // write differences
    // panic if anything changed
    assert!(passing)
}

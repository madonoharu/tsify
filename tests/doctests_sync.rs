//! Sync the crate's doctest with a folder, so their output can also be verified
//!
//!
use std::fmt::Debug;
use std::fs;
use std::path::Path;

use serde::Deserialize;

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

#[test]
fn doctests_are_synced() {
    let root = Path::new(DOCTESTS_DIR);
    let json = std::fs::read_to_string("target/doctests.json")
        .expect("run the doctest extraction step first");

    // if the test is passing
    let mut passing = true;
    let mut fixed = true;

    let Doctests {
        format_version,
        mut doctests,
    } = serde_json::from_str(&json).unwrap();
    if format_version != 2 {
        eprintln!("format version is not 2");
        passing = false;
    }
    // otherwise it could be wrong?
    // assert_eq!(format_version, 2);

    // so the naming scheme is file-001 with the number increasing
    // so this will "fail" on like a reordering
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
            // create a ??file??/??folder??
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
            // create directory if not exists
            if !test_dir.is_dir() {
                println!("creating directory {test_dir:?}");
                fs::create_dir(test_dir.clone()).expect(&format!("{DOCTESTS_DIR:?} exists"));
                passing = false;
            }
            // check all Cargo.toml
            let toml = test_dir.join(TOML);
            let toml_contents = toml_template.replace("{name}", &name);
            if !toml.is_file() {
                println!("writing new {toml:?}");
                fs::write(toml, &toml_contents).expect("can write toml");
                passing = false;
            } else if fs::read_to_string(&toml).unwrap() != toml_contents {
                println!("updating {toml:?}");
                fs::write(toml, &toml_contents).expect("can write toml");
                passing = false;
            }
            // check the actual code
            let entry_point = test_dir.join(ENTRY);
            let code = format!(
                "//! doctest at {}:{}{}\n",
                doctest.file,
                doctest.line,
                doctest.doctest_code.to_string().trim_end()
            );
            if !entry_point.is_file() {
                println!("creating new {entry_point:?}");
                fs::write(entry_point, code.as_bytes()).expect("can write doctest");
                passing = false;
            } else if fs::read(&entry_point).unwrap() != code.as_bytes() {
                println!("updating {entry_point:?}");
                fs::write(entry_point, code.as_bytes()).expect("can write doctest");
                passing = false;
            }
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
            match (ref_file.is_file(), out_file.is_file()) {
                (false, false) => {
                    eprintln!("reference {ref_file:?} does not exist and there is no output, please build all doctests");
                    passing = false;
                    fixed = false;
                }
                (false, true) => {
                    eprintln!("reference {ref_file:?} does not exist, copying from output");
                    fs::copy(out_file, ref_file).expect("can copy file");
                    passing = false;
                }
                (true, true) => {
                    if fs::read_to_string(&ref_file).unwrap()
                        != fs::read_to_string(&out_file).unwrap()
                    {
                        eprintln!("{ref_file:?} does not match");
                        passing = false;
                        fixed = false;
                    }
                }
                (true, false) => {
                    eprintln!("no generated output for {ref_file:?}, please build all doctests");
                    passing = false;
                    fixed = false;
                }
            }
        }

        //
    }
    // generate examples/doctests/declare-001/...
    // compare with existing files
    // write differences
    // panic if anything changed
    match (passing, fixed) {
        (false, true) => panic!("Doctests were not synced to output - they should be now"),
        (false, false) => panic!("tests-doc/build_all.sh"),
        _ => {}
    };
}

use crate::attrs::TypeGenerationConfig;

use super::TsType;

macro_rules! assert_ts {
        ($config:expr, $( $t:ty )|* , $expected:expr) => {
          $({
            let ty: syn::Type = syn::parse_quote!($t);
            let ts_type = TsType::from_syn_type(&$config, &ty);
            assert_eq!(ts_type.to_string(), $expected);
          })*
        };
      }

#[test]
fn test_basic_types() {
    let config = TypeGenerationConfig::default();
    if cfg!(feature = "js") {
        assert_ts!(config, (), "undefined");
        assert_ts!(config, u128 | i128, "bigint");
        assert_ts!(config, HashMap<String, i32> | BTreeMap<String, i32>, "Map<string, number>");
        assert_ts!(config, Option<i32>, "number | undefined");
        assert_ts!(config, Vec<Option<T>> | VecDeque<Option<T>> | LinkedList<Option<T>> | &'a [Option<T>], "(T | undefined)[]");
    } else {
        assert_ts!(config, (), "null");
        assert_ts!(config, u128 | i128, "number");
        assert_ts!(config, HashMap<String, i32> | BTreeMap<String, i32>, "Record<string, number>");
        assert_ts!(config, Option<i32>, "number | null");
        assert_ts!(config, Vec<Option<T>> | VecDeque<Option<T>> | LinkedList<Option<T>> | &'a [Option<T>], "(T | null)[]");
        assert_ts!(config, ByteBuf, "number[]");
    }

    assert_ts!(
        config,
        u8 | u16 | u32 | u64 | usize | i8 | i16 | i32 | i64 | isize | f32 | f64,
        "number"
    );
    assert_ts!(config, String | str | char | Path | PathBuf, "string");
    assert_ts!(config, bool, "boolean");
    assert_ts!(config, Box<i32> | Rc<i32> | Arc<i32> | Cell<i32> | RefCell<i32> | Cow<'a, i32>, "number");
    assert_ts!(config, Vec<i32> | VecDeque<i32> | LinkedList<i32> | &'a [i32], "number[]");
    assert_ts!(config, HashSet<i32> | BTreeSet<i32>, "number[]");

    assert_ts!(config, Result<i32, String>, "{\n    Ok: number;\n} | {\n    Err: string;\n}");
    assert_ts!(config, dyn Fn(String, f64) | dyn FnOnce(String, f64) | dyn FnMut(String, f64), "(arg0: string, arg1: number) => void");
    assert_ts!(config, dyn Fn(String) -> i32 | dyn FnOnce(String) -> i32 | dyn FnMut(String) -> i32, "(arg0: string) => number");

    assert_ts!(config, (i32), "number");
    assert_ts!(config, (i32, String, bool), "[number, string, boolean]");

    assert_ts!(config, [i32; 4], "[number, number, number, number]");
    assert_ts!(
        config,
        [i32; 16],
        format!("[{}]", ["number"; 16].join(", "))
    );
    assert_ts!(config, [i32; 17], "number[]");
    assert_ts!(config, [i32; 1 + 1], "number[]");

    assert_ts!(
        config,
        Duration,
        "{\n    secs: number;\n    nanos: number;\n}"
    );
    assert_ts!(
        config,
        SystemTime,
        "{\n    secs_since_epoch: number;\n    nanos_since_epoch: number;\n}"
    );

    assert_ts!(
        config,
        Range<i32>,
        "{\n    start: number;\n    end: number;\n}"
    );
    assert_ts!(
        config,
        Range<&'static str>,
        "{\n    start: string;\n    end: string;\n}"
    );
    assert_ts!(
        config,
        RangeInclusive<usize>,
        "{\n    start: number;\n    end: number;\n}"
    );
}

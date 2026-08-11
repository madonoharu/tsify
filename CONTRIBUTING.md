# Contributing to tsify

## Running the tests

`./test.sh` runs everything CI runs:

```sh
cargo test --all
cargo test --all -F js
wasm-pack test --node
wasm-pack test --node -F js
./tests-e2e/build_all.sh
./tests-e2e/reference_output/compare_output.sh
```

Required tools: stable Rust, [wasm-pack](https://rustwasm.github.io/wasm-pack/),
[cargo-expand](https://github.com/dtolnay/cargo-expand) (for the expand tests),
and Node.js (for the wasm/e2e tests).

## Expansion snapshots (`tests/expand/*.expanded.rs`)

The snapshot files record the full macro expansion of `#[derive(Tsify)]`,
**including** the code that wasm-bindgen's own macros emit. Because
`Cargo.lock` is intentionally not committed (see below), CI resolves
dependencies fresh on every run — so the snapshots always track the **latest
compatible wasm-bindgen release**, which is the configuration downstream users
actually get.

Two consequences:

1. A new wasm-bindgen release that changes its codegen will make `expandtest`
   fail with a snapshot diff even though nothing in this repo changed. This is
   expected: the nightly `check-wasmbindgen-changes` workflow regenerates the
   snapshots and opens an update PR automatically. **Prefer letting the cron
   do this.** Only regenerate by hand when your own change alters the macro
   output.

2. When you do regenerate by hand, make sure your local resolution matches
   CI's before overwriting:

   ```sh
   rm -f Cargo.lock && cargo generate-lockfile
   MACROTEST=overwrite cargo test -p tsify --test expandtest
   ```

   Skipping the first line is the classic trap: a stale local `Cargo.lock`
   (even a few weeks old) can resolve an older wasm-bindgen, producing
   snapshots that pass locally but fail in CI. `cargo metadata --locked`
   will *not* warn about this — the stale version still satisfies the
   manifest ranges.

## Why `Cargo.lock` is not committed

Keeping the lockfile out of version control means CI always resolves
dependencies fresh, so the test suite and the expansion snapshots verify
tsify against the dependency versions users actually get. The trade-off is
deliberate: a committed lockfile would make CI reproducible but silently
stale. If you think this should change, please open an issue rather than
including the change in an unrelated PR.

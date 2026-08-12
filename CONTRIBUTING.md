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
dependencies fresh on every run, so the snapshots always track the **latest
compatible wasm-bindgen release** — what downstream users actually get.

Two consequences:

1. A new wasm-bindgen release that changes its codegen makes `expandtest` fail
   with a snapshot diff even though nothing in this repo changed. That is
   expected: the nightly `check-wasmbindgen-changes` workflow regenerates the
   snapshots and opens an update PR. **Prefer letting the cron do this** —
   regenerate by hand only when your own change alters the macro output.

2. When you regenerate by hand, match CI's resolution before overwriting:

   ```sh
   rm -f Cargo.lock && cargo generate-lockfile
   MACROTEST=overwrite cargo test -p tsify --test expandtest
   ```

   Skipping the first line is the classic trap: a stale local `Cargo.lock`
   (even a few weeks old) can resolve an older wasm-bindgen and produce
   snapshots that pass locally but fail in CI. `cargo metadata --locked`
   will *not* warn — the stale version still satisfies the manifest ranges.

## Why `Cargo.lock` is not committed

Keeping the lockfile out of version control means CI always resolves
dependencies fresh, so the test suite and the snapshots verify tsify against
the versions downstream users actually get. The trade-off is deliberate: a
committed lockfile would make CI reproducible but silently stale. If you
think that should change, please open an issue rather than bundling it into
an unrelated PR.

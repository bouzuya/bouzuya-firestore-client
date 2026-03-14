# bouzuya-firestore-client

- Run `cargo +nightly fmt` and `cargo clippy -- -D warnings` after every file edit.
- Module files (`lib.rs` and files containing submodules) must only contain `mod xxx;` and `pub use self::xxx::...;`.
- `pub(crate)` methods must have tests in `#[cfg(test)] mod tests` within the same file.
- `pub` structs, functions, and trait implementations must have tests in `tests/{next_version}/{struct_name}.rs`.
- Do not use `unwrap` in test code. Use `?` with a `Result` return type instead.
- Sort methods within an `impl` block in ascending (alphabetical) order.
- Place associated functions (e.g. `new`) and methods (taking `&self` or `&mut self`) in separate `impl` blocks.
- Follow the TDD (Red-Green-Refactor) cycle strictly, one step at a time.
    - Do not write production code unless a failing test requires it.
    - Do not write more test code than is sufficient to fail.
    - Do not write more production code than is sufficient to pass the failing test.
    - Refactor only when all tests are green.
    - Red phase: Write a failing test, run `cargo test` to confirm it fails, then ask the human to review before proceeding.
    - Green phase: Write minimal production code to pass the test, run `cargo test` to confirm it passes, then ask the human to review before proceeding.
    - Refactor phase: Refactor the code, run `cargo test` to confirm all tests still pass, then ask the human to review before starting the next cycle.

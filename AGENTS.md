# bouzuya-firestore-client

- Run `cargo +nightly fmt` after every file edit.
- Module files (`lib.rs` and files containing submodules) must only contain `mod xxx;` and `pub use self::xxx::...;`.
- `pub(crate)` methods must have tests in `#[cfg(test)] mod tests` within the same file.
- Do not use `unwrap` in test code. Use `?` with a `Result` return type instead.
- Sort methods within an `impl` block in ascending (alphabetical) order.
- Place associated functions (e.g. `new`) and methods (taking `&self` or `&mut self`) in separate `impl` blocks.

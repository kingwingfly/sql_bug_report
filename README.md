# Steps
1. Start postgresql
2. Test a few times
```sh
cargo test
```
It ramdomly fails with the following error `PoolTimedOut`:
```sh
running 2 tests
INIT ModelManager
test tests::test2 ... ok
thread 'tests::test1' panicked at src/lib.rs:55:31:
called `Result::unwrap()` on an `Err` value: PoolTimedOut
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test tests::test1 ... FAILED
```

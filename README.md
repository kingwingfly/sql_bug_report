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
thread 'tests::test1' panicked at src/lib.rs:38:33:
called `Result::unwrap()` on an `Err` value: PoolTimedOut
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test tests::test1 ... FAILED
```


# Reason
This is because every tokio::test runs in a separate runtime, but share the same OnceCell initialized by one of them.

If the runtime which initialized the OnceCell ends and dropped, the left runtimes can nerver get the OnceCell.

Fix in this way:
```rust
#[tokio::test]
async fn test2() {
    let _mm = init_test().await;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
```
This can ensure the runtime which initialized the OnceCell is still alive when the left runtimes try to get the OnceCell.

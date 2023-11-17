#![allow(dead_code)]

use lazy_static::lazy_static;

use crate::init_test;

pub fn test_rt() -> &'static tokio::runtime::Runtime {
    lazy_static! {
        static ref RT: tokio::runtime::Runtime = {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(init_test());
            rt
        };
    }
    &RT
}

/// All the test about database should be run in this function.
/// Because we need to share the same database connection,
/// which cannot keep alive between runtimes.
/// In other words, if the database connection is closed
/// due to the end of the runtime which initialized it, the next runtimes will fail.
pub fn run_test<F: std::future::Future>(f: F) -> F::Output {
    test_rt().block_on(f)
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn fixed_test() {
        run_test(async {
            let mm = init_test().await;
            mm.pg().await;
        })
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn fixed_test() {
        run_test(async {
            let _mm = init_test().await;
        })
    }
}

use std::{error, future, marker::PhantomData};

use chain_host::HostError;
use derivative::Derivative;
use derive_new::new;
use tower::{retry, timeout::error::Elapsed};
use tracing::{debug, error};

#[derive(Derivative, new)]
#[derivative(Clone)]
pub struct Policy<EF: ErrorFilter> {
    remaining_attempts: usize,
    _phantom: PhantomData<EF>,
}

type Error = Box<dyn error::Error + Send + Sync>;

pub trait ErrorFilter {
    fn is_retriable(err: &Error) -> bool;
}

impl<Req: Clone, EF: ErrorFilter> retry::Policy<Req, (), Error> for Policy<EF> {
    type Future = future::Ready<()>;

    fn retry(&mut self, _req: &mut Req, result: &mut Result<(), Error>) -> Option<Self::Future> {
        if self.remaining_attempts == 0 {
            debug!("No more retry attempts");
            return None;
        }
        match result {
            Err(err) if EF::is_retriable(err) => {
                self.remaining_attempts -= 1;
                error!("Host error: {err} Remaining attempts: {}", self.remaining_attempts);
                Some(future::ready(()))
            }
            Err(_) | Ok(_) => None,
        }
    }

    fn clone_request(&mut self, req: &Req) -> Option<Req> {
        Some(req.clone())
    }
}

#[derive(Clone, Copy)]
pub struct HostErrorFilter;

impl ErrorFilter for HostErrorFilter {
    fn is_retriable(err: &Error) -> bool {
        if err.downcast_ref::<Elapsed>().is_some() {
            return true;
        }
        let err = err.downcast_ref::<HostError>().expect("unexpected error");
        match err {
            HostError::ChainDb(_)
            | HostError::BlockTrieError(_)
            | HostError::ProofSerializationError(_) => false,
            HostError::Prover(_) | HostError::BlockFetcher(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod error_filter {
        use chain_host::{BlockFetcherError, BlockTrieError, ChainDbError, ProverError};

        use super::*;

        fn wrap_err(err: HostError) -> Error {
            Box::new(err)
        }

        #[test]
        fn retriable() {
            for err in [
                wrap_err(HostError::Prover(ProverError::Prover("error".to_string()))),
                wrap_err(HostError::BlockFetcher(BlockFetcherError::Provider("error".to_string()))),
                Box::new(Elapsed::new()),
            ] {
                assert!(HostErrorFilter::is_retriable(&err));
            }
        }

        #[test]
        fn non_retriable() {
            for err in [
                wrap_err(HostError::ChainDb(ChainDbError::BlockNotFound)),
                wrap_err(HostError::BlockTrieError(BlockTrieError::GetBlockHashFailed(0))),
                wrap_err(HostError::ProofSerializationError(bincode::Error::new(
                    bincode::ErrorKind::SizeLimit,
                ))),
            ] {
                assert!(!HostErrorFilter::is_retriable(&err))
            }
        }
    }

    mod policy {
        use std::sync::atomic::{AtomicU32, Ordering};

        use future::Future;
        use tower::{Service, ServiceBuilder};

        use super::*;

        struct RetryAll;

        impl ErrorFilter for RetryAll {
            fn is_retriable(_err: &Error) -> bool {
                true
            }
        }

        struct RetryNone;

        impl ErrorFilter for RetryNone {
            fn is_retriable(_err: &Error) -> bool {
                false
            }
        }

        fn err_handler(_: ()) -> impl Future<Output = Result<(), Error>> {
            future::ready(Err("error".into()))
        }

        fn ok_handler(_: ()) -> impl Future<Output = Result<(), Error>> {
            future::ready(Ok(()))
        }

        async fn test_call<EF: ErrorFilter, Func, Fut>(
            max_retries: usize,
            expected_attempts: u32,
            service_fn: Func,
        ) -> Result<(), Error>
        where
            Func: Fn(()) -> Fut,
            Fut: Future<Output = Result<(), Error>>,
        {
            let attempts = AtomicU32::new(0);
            let mut service = ServiceBuilder::new()
                .retry(Policy::<EF>::new(max_retries))
                .service_fn(|req| {
                    attempts.fetch_add(1, Ordering::SeqCst);
                    service_fn(req)
                });
            let res = service.call(()).await;
            assert_eq!(attempts.load(Ordering::SeqCst), expected_attempts);
            res
        }

        #[tokio::test]
        async fn ok() {
            let res = test_call::<RetryAll, _, _>(1, 1, ok_handler).await;
            assert!(res.is_ok());
        }

        #[tokio::test]
        async fn no_more_attempts() {
            let res = test_call::<RetryAll, _, _>(0, 1, err_handler).await;
            assert!(res.is_err());
        }

        #[tokio::test]
        async fn retriable() {
            let res = test_call::<RetryAll, _, _>(1, 2, err_handler).await;
            assert!(res.is_err());
        }

        #[tokio::test]
        async fn non_retriable() {
            let res = test_call::<RetryNone, _, _>(1, 1, err_handler).await;
            assert!(res.is_err());
        }
    }
}

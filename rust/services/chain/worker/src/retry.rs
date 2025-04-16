use std::{error, future, marker::PhantomData, sync::Arc};

use chain_host::HostError;
use derivative::Derivative;
use tower::{
    retry::{
        self,
        budget::{Budget, TpsBudget},
    },
    timeout::error::Elapsed,
};
use tracing::error;

#[derive(Derivative)]
#[derivative(Clone)]
pub struct Policy<EF: ErrorFilter> {
    pub budget: Arc<TpsBudget>,
    _phantom: PhantomData<EF>,
}

impl<EF: ErrorFilter> Policy<EF> {
    pub fn new(budget: TpsBudget) -> Self {
        Self {
            budget: Arc::new(budget),
            _phantom: PhantomData,
        }
    }
}

type Error = Box<dyn error::Error + Send + Sync>;

pub trait ErrorFilter {
    fn is_retriable(err: &Error) -> bool;
}

impl<Req: Clone, EF: ErrorFilter> retry::Policy<Req, (), Error> for Policy<EF> {
    type Future = future::Ready<()>;

    fn retry(&mut self, _req: &mut Req, result: &mut Result<(), Error>) -> Option<Self::Future> {
        match result {
            Ok(_) => {
                self.budget.deposit();
                None
            }
            Err(err) if EF::is_retriable(err) => {
                let withdraw = self.budget.withdraw();
                if !withdraw {
                    error!("retry budget exhausted");
                    return None;
                }
                error!("retrying after error: {:?}", err);
                Some(future::ready(()))
            }
            Err(_) => None,
        }
    }

    fn clone_request(&mut self, req: &Req) -> Option<Req> {
        Some(req.clone())
    }
}

#[derive(Clone, Copy)]
pub struct HostErrorFilter;

impl ErrorFilter for HostErrorFilter {
    #[allow(clippy::expect_used)]
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
                wrap_err(HostError::BlockFetcher(BlockFetcherError::Provider(
                    ethers::providers::ProviderError::UnsupportedRPC,
                ))),
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
        use std::{
            sync::atomic::{AtomicU32, Ordering},
            time::Duration,
        };

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
            min_retries_per_second: u32,
            expected_attempts: u32,
            service_fn: Func,
        ) -> Result<(), Error>
        where
            Func: Fn(()) -> Fut,
            Fut: Future<Output = Result<(), Error>>,
        {
            let attempts = AtomicU32::new(0);
            let budget = TpsBudget::new(Duration::from_secs(1), min_retries_per_second, 0.0);
            let mut service = ServiceBuilder::new()
                .retry(Policy::<EF>::new(budget))
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

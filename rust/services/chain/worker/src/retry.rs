use std::{error, future};

use chain_host::{AppendPrepend, HostError};
use derive_new::new;
use tower::{retry, timeout::error::Elapsed};
use tracing::error;

#[derive(Clone, new)]
pub struct Policy {
    remaining_attempts: usize,
}

type Error = Box<dyn error::Error + Send + Sync>;

impl retry::Policy<AppendPrepend, (), Error> for Policy {
    type Future = future::Ready<()>;

    fn retry(
        &mut self,
        _req: &mut AppendPrepend,
        result: &mut Result<(), Error>,
    ) -> Option<Self::Future> {
        if self.remaining_attempts == 0 {
            return None;
        }
        let err = match result {
            Ok(_) => return None,
            Err(err) => err,
        };
        if err.downcast_ref::<Elapsed>().is_some() {
            self.remaining_attempts -= 1;
            error!("Host error: timeout");
            return Some(future::ready(()));
        }
        let err = err
            .downcast_ref::<HostError>()
            .expect("unexpected error from host");
        match err {
            HostError::ChainDb(_)
            | HostError::BlockTrieError(_)
            | HostError::ProofSerializationError(_) => None,
            HostError::Prover(_) | HostError::BlockFetcher(_) => {
                self.remaining_attempts -= 1;
                error!("Host error: {err} Remaining attempts: {}", self.remaining_attempts);
                Some(future::ready(()))
            }
        }
    }

    fn clone_request(&mut self, req: &AppendPrepend) -> Option<AppendPrepend> {
        Some(*req)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::atomic::{AtomicU32, Ordering},
        time::Duration,
    };

    use chain_host::{BlockFetcherError, BlockTrieError, ChainDbError, ProverError};
    use future::Future;
    use tower::{Service, ServiceBuilder};

    use super::*;

    fn prover_error() -> HostError {
        HostError::Prover(ProverError::Prover("error".to_string()))
    }

    fn block_fetcher_error() -> HostError {
        HostError::BlockFetcher(BlockFetcherError::Provider("error".to_string()))
    }

    const fn chain_db_error() -> HostError {
        HostError::ChainDb(ChainDbError::BlockNotFound)
    }

    const fn block_trie_error() -> HostError {
        HostError::BlockTrieError(BlockTrieError::GetBlockHashFailed(0))
    }

    fn serialization_error() -> HostError {
        HostError::ProofSerializationError(bincode::Error::new(bincode::ErrorKind::SizeLimit))
    }

    fn wrap_err(err: HostError) -> Error {
        Box::new(err)
    }

    fn err_result(err: HostError) -> impl Future<Output = Result<(), Error>> {
        future::ready(Err(wrap_err(err)))
    }

    fn unwrap_err(res: Result<(), Error>) -> HostError {
        *res.unwrap_err().downcast().unwrap()
    }

    async fn test_call<Func, Fut>(
        max_retries: usize,
        exp_attempts: u32,
        service_fn: Func,
    ) -> Result<(), Error>
    where
        Func: Fn(AppendPrepend) -> Fut,
        Fut: Future<Output = Result<(), Error>>,
    {
        let attempts = AtomicU32::new(0);
        let mut service = ServiceBuilder::new()
            .retry(Policy::new(max_retries))
            .timeout(Duration::from_secs(1))
            .service_fn(|req| {
                attempts.fetch_add(1, Ordering::Relaxed);
                service_fn(req)
            });
        let res = service.call(AppendPrepend).await;
        assert_eq!(attempts.load(Ordering::Relaxed), exp_attempts);
        res
    }

    #[tokio::test]
    async fn ok() {
        let res = test_call(1, 1, |_| future::ready(Ok::<_, Error>(()))).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn no_more_attempts() {
        let res = test_call(0, 1, |_| err_result(prover_error())).await;
        assert_eq!(unwrap_err(res), prover_error());
    }

    #[tokio::test]
    async fn prover_error_retry() {
        let res = test_call(1, 2, |_| err_result(prover_error())).await;
        assert_eq!(unwrap_err(res), prover_error());
    }

    #[tokio::test]
    async fn block_fetcher_error_retry() {
        let res = test_call(1, 2, |_| err_result(block_fetcher_error())).await;
        assert_eq!(unwrap_err(res), block_fetcher_error());
    }

    #[tokio::test(start_paused = true)]
    async fn timeout_retry() {
        let res = test_call(1, 2, |_| async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok(())
        })
        .await;
        assert!(res.unwrap_err().downcast::<Elapsed>().is_ok());
    }

    #[tokio::test]
    async fn chain_db_error_no_retry() {
        let res = test_call(1, 1, |_| err_result(chain_db_error())).await;
        assert_eq!(unwrap_err(res), chain_db_error());
    }

    #[tokio::test]
    async fn block_trie_error_no_retry() {
        let res = test_call(1, 1, |_| err_result(block_trie_error())).await;
        assert_eq!(unwrap_err(res), block_trie_error());
    }

    #[tokio::test]
    async fn serialization_error_no_retry() {
        let res = test_call(1, 1, |_| err_result(serialization_error())).await;
        assert_eq!(unwrap_err(res), serialization_error());
    }
}

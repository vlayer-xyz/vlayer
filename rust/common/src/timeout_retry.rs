use std::{future::Future, pin::Pin, time::Duration};

use derive_new::new;
use futures::{TryFuture, TryFutureExt};
use futures_retry::{ErrorHandler, FutureFactory, FutureRetry, RetryPolicy};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
/// Retry & timeout policy for asynchronous operations
pub struct Policy<Error> {
    #[builder(default, setter(strip_option))]
    /// Maximum time for all attempts (including delays)
    total_timeout: Option<Duration>,
    #[builder(default, setter(strip_option))]
    /// Maximum time for a single attempt
    attempt_timeout: Option<Duration>,
    #[builder(default, setter(strip_option))]
    /// Delay between retries (applies to errors only, not attempt timeouts)
    retry_delay: Option<Duration>,
    #[builder(default, setter(strip_option))]
    /// Maximum number of attempts
    max_attempts: Option<usize>,
    #[builder(default = Box::new(|_| true))]
    /// Filter which errors are retriable (by default all are)
    retry_only: Box<dyn Fn(&Error) -> bool>,
}

impl<Error> Policy<Error> {
    pub fn wrap<Factory>(self, factory: Factory) -> TimeoutFuture<Factory::FutureItem>
    where
        Error: 'static,
        Factory: FutureFactory + 'static,
        Factory::FutureItem: TryFuture<Error = Error> + 'static,
    {
        let factory = TimeoutFutureFactory::new(self.attempt_timeout, factory);
        let handler =
            Handler::new(self.retry_delay.unwrap_or_default(), self.max_attempts, self.retry_only);
        let retry_fut = FutureRetry::new(factory, handler)
            .map_err(|(e, _)| e)
            .map_ok(|(r, _)| r);
        maybe_timeout_future(self.total_timeout, retry_fut)
    }
}

#[derive(Debug)]
pub enum TimeoutOrError<Error> {
    Timeout,
    Error(Error),
}

impl<Error: Clone> Clone for TimeoutOrError<Error> {
    fn clone(&self) -> Self {
        match self {
            Self::Timeout => Self::Timeout,
            Self::Error(err) => Self::Error(err.clone()),
        }
    }
}

impl<Error: PartialEq> PartialEq for TimeoutOrError<Error> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Error(err1), Self::Error(err2)) => err1 == err2,
            (Self::Timeout, Self::Timeout) => true,
            _ => false,
        }
    }
}

impl<Error> From<Error> for TimeoutOrError<Error> {
    fn from(err: Error) -> Self {
        Self::Error(err)
    }
}

type TimeoutFuture<F> = Pin<
    Box<dyn Future<Output = Result<<F as TryFuture>::Ok, TimeoutOrError<<F as TryFuture>::Error>>>>,
>;

fn maybe_timeout_future<Fut, Return, Error>(
    timeout: Option<Duration>,
    future: Fut,
) -> Pin<Box<dyn Future<Output = Fut::Output>>>
where
    Fut: Future<Output = Result<Return, TimeoutOrError<Error>>> + 'static,
{
    if let Some(timeout) = timeout {
        Box::pin(async move {
            match tokio::time::timeout(timeout, future).await {
                Ok(res) => res,
                Err(_) => Err(TimeoutOrError::Timeout),
            }
        })
    } else {
        Box::pin(future)
    }
}

#[derive(new)]
struct TimeoutFutureFactory<Factory: FutureFactory> {
    timeout: Option<Duration>,
    inner: Factory,
}

impl<Factory> FutureFactory for TimeoutFutureFactory<Factory>
where
    Factory: FutureFactory,
    Factory::FutureItem: 'static,
{
    type FutureItem = TimeoutFuture<Factory::FutureItem>;

    fn new(&mut self) -> Self::FutureItem {
        let fut = self.inner.new().err_into();
        maybe_timeout_future(self.timeout, fut)
    }
}

#[derive(new)]
struct Handler<Error> {
    retry_delay: Duration,
    #[new(into)]
    max_attempts: Option<usize>,
    retry_only: Box<dyn Fn(&Error) -> bool>,
}

impl<Error> ErrorHandler<TimeoutOrError<Error>> for Handler<Error> {
    type OutError = TimeoutOrError<Error>;

    fn handle(
        &mut self,
        attempt: usize,
        err: TimeoutOrError<Error>,
    ) -> RetryPolicy<TimeoutOrError<Error>> {
        if let Some(max_attempts) = self.max_attempts {
            if attempt >= max_attempts {
                return RetryPolicy::ForwardError(err);
            }
        }
        match err {
            TimeoutOrError::Timeout => RetryPolicy::Repeat,
            TimeoutOrError::Error(err) if (self.retry_only)(&err) => {
                RetryPolicy::WaitRetry(self.retry_delay)
            }
            err => RetryPolicy::ForwardError(err),
        }
    }
}

#[cfg(test)]
mod tests;

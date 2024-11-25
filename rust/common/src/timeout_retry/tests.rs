use super::*;

mod policy_builder {
    use super::{Duration, Policy};

    #[test]
    fn default() {
        let policy = Policy::<()>::builder().build();
        assert_eq!(policy.total_timeout, None);
        assert_eq!(policy.attempt_timeout, None);
        assert_eq!(policy.retry_delay, None);
        assert_eq!(policy.max_attempts, None);
        assert!((policy.retry_only)(&()));
    }

    #[test]
    fn custom() {
        let policy = Policy::<&str>::builder()
            .total_timeout(Duration::from_secs(2))
            .attempt_timeout(Duration::from_secs(1))
            .retry_delay(Duration::from_millis(100))
            .max_attempts(3)
            .retry_only(Box::new(|err| err == &"retriable"))
            .build();
        assert_eq!(policy.total_timeout.unwrap(), Duration::from_secs(2));
        assert_eq!(policy.attempt_timeout.unwrap(), Duration::from_secs(1));
        assert_eq!(policy.retry_delay.unwrap(), Duration::from_millis(100));
        assert_eq!(policy.max_attempts.unwrap(), 3);
        assert!((policy.retry_only)(&"retriable"));
        assert!(!(policy.retry_only)(&"non_retriable"));
    }
}

mod handler {
    use super::{Duration, ErrorHandler, Handler, RetryPolicy};

    #[test]
    fn no_more_attempts() {
        let mut handler = Handler::new(Default::default(), 0, Box::new(|_| true));
        let retry_policy = handler.handle(0, "err".into());
        assert_eq!(retry_policy, RetryPolicy::ForwardError("err".into()));
    }

    #[test]
    fn non_retriable() {
        let mut handler = Handler::new(Default::default(), None, Box::new(|e| e == &"retriable"));
        let retry_policy = handler.handle(0, "non_retriable".into());
        assert_eq!(retry_policy, RetryPolicy::ForwardError("non_retriable".into()))
    }

    #[test]
    fn retry_delay() {
        let mut handler =
            Handler::new(Duration::from_secs(1), None, Box::new(|e| e == &"retriable"));
        let retry_policy = handler.handle(0, "retriable".into());
        assert_eq!(retry_policy, RetryPolicy::WaitRetry(Duration::from_secs(1)));
    }
}

mod maybe_timeout {
    use super::{maybe_timeout_future, Duration, TimeoutOrError};

    #[tokio::test(start_paused = true)]
    async fn timed_out() {
        let timeout = Some(Duration::from_secs(1));
        let future = async {
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok::<_, TimeoutOrError<()>>("result")
        };

        let result = maybe_timeout_future(timeout, future).await;
        assert_eq!(result.unwrap_err(), TimeoutOrError::Timeout);
    }

    #[tokio::test(start_paused = true)]
    async fn complete() {
        let future = async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            Ok::<_, TimeoutOrError<()>>("result")
        };

        let result = maybe_timeout_future(None, future).await;
        assert_eq!(result.unwrap(), "result");
    }
}

mod timeout_future_factory {
    use std::sync::{Arc, Mutex};

    use super::{Duration, FutureFactory, TimeoutFutureFactory, TimeoutOrError};

    #[tokio::test(start_paused = true)]
    async fn timeout_added() {
        let durations =
            Arc::new(Mutex::new(vec![Duration::from_millis(100), Duration::from_secs(2)]));
        let factory = || {
            let durations = durations.clone();
            async move {
                let duration = durations.lock().unwrap().remove(0);
                tokio::time::sleep(duration).await;
                Ok::<_, ()>("result")
            }
        };
        let mut timeout_factory = TimeoutFutureFactory::new(Some(Duration::from_secs(1)), factory);

        let future_100ms = timeout_factory.new();
        let result = future_100ms.await;
        assert_eq!(result.unwrap(), "result");

        let future_2s = timeout_factory.new();
        let result = future_2s.await;
        assert_eq!(result.unwrap_err(), TimeoutOrError::Timeout);
    }
}

mod policy {
    use std::{cell::RefCell, rc::Rc, time::Duration};

    use futures::future;

    use super::{FutureFactory, Policy, TimeoutOrError};

    fn count_attempts_factory<F: FutureFactory + Unpin>(
        mut factory: F,
    ) -> (Rc<RefCell<u32>>, impl FutureFactory<FutureItem = F::FutureItem>) {
        let attempts = Rc::new(RefCell::new(0));
        let attempts_ = attempts.clone();
        (attempts, move || {
            *attempts_.borrow_mut() += 1;
            factory.new()
        })
    }

    #[tokio::test]
    async fn success() {
        let policy = Policy::<()>::builder().build();
        let (attempts, factory) = count_attempts_factory(|| future::ready(Ok("result")));

        let result = policy.wrap(factory).await;
        assert_eq!(result.unwrap(), "result");
        assert_eq!(*attempts.borrow(), 1);
    }

    #[tokio::test]
    async fn non_retriable() {
        let policy = Policy::builder()
            .retry_only(Box::new(|err| err == &"retriable"))
            .build();
        let (attempts, factory) =
            count_attempts_factory(|| future::ready(Err::<(), _>("non_retriable")));

        let result = policy.wrap(factory).await;
        assert_eq!(result.unwrap_err(), TimeoutOrError::Error("non_retriable"));
        assert_eq!(*attempts.borrow(), 1);
    }

    #[tokio::test]
    async fn max_attempts_exceeded() {
        let policy = Policy::builder().max_attempts(2).build();
        let (attempts, factory) = count_attempts_factory(|| future::ready(Err::<(), _>("error")));

        let result = policy.wrap(factory).await;
        assert_eq!(result.unwrap_err(), TimeoutOrError::Error("error"));
        assert_eq!(*attempts.borrow(), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn attempt_timeout() {
        let policy = Policy::<()>::builder()
            .attempt_timeout(Duration::from_secs(1))
            .max_attempts(2)
            .build();
        let (attempts, factory) = count_attempts_factory(|| async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok(())
        });

        let result = policy.wrap(factory).await;
        assert_eq!(result.unwrap_err(), TimeoutOrError::Timeout);
        assert_eq!(*attempts.borrow(), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn test_total_timeout() {
        let policy = Policy::<()>::builder()
            .total_timeout(Duration::from_secs(1))
            .build();
        let (attempts, factory) = count_attempts_factory(|| async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok(())
        });

        let result = policy.wrap(factory).await;
        assert_eq!(result.unwrap_err(), TimeoutOrError::Timeout);
        assert_eq!(*attempts.borrow(), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn retry_delay() {
        let policy = Policy::builder()
            .retry_delay(Duration::from_millis(500))
            .max_attempts(2)
            .build();

        let start = tokio::time::Instant::now();
        let (attempts, factory) = count_attempts_factory(|| future::ready(Err::<(), _>("error")));

        let _ = policy.wrap(factory).await;
        assert!(start.elapsed() >= Duration::from_millis(500));
        assert_eq!(*attempts.borrow(), 2);
    }

    #[tokio::test]
    async fn gradual_success() {
        let policy = Policy::builder().build();
        let results = Rc::new(RefCell::new(vec![Err("error"), Err("error"), Ok("result")]));
        let (attempts, factory) =
            count_attempts_factory(move || future::ready(results.borrow_mut().remove(0)));

        let result = policy.wrap(factory).await;
        assert_eq!(result.unwrap(), "result");
        assert_eq!(*attempts.borrow(), 3);
    }
}

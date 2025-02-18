use test_runner::{cli::TestArgs, watch_test};
use tracing::info;

use crate::errors::{Error, Result};

pub async fn run_test(cmd: Box<TestArgs>) -> Result<()> {
    info!("Running vlayer tests");
    if cmd.is_watch() {
        Box::pin(watch_test(*cmd)).await?;
    } else {
        let test_result = Box::pin(cmd.run()).await?;
        let failed_tests_count = test_result.failed();
        if !test_result.allow_failure && failed_tests_count > 0 {
            return Err(Error::TestsFailed(failed_tests_count));
        }
    }
    Ok(())
}

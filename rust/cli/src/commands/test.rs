use test_runner::cli::TestArgs;
use tracing::info;

use crate::errors::CLIError;

pub async fn run_test(cmd: Box<TestArgs>) -> Result<(), CLIError> {
    info!("Running vlayer tests");
    let test_result = cmd.run().await?;
    let failed_tests_count = test_result.failed();
    if !test_result.allow_failure && failed_tests_count > 0 {
        return Err(CLIError::TestsFailed(failed_tests_count));
    }
    Ok(())
}

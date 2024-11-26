use tabled::Tabled;
use zkvm_benchmarks::BenchmarkResult;

use crate::cycle;

#[derive(Tabled)]
pub struct Row {
    name: String,
    actual_cycles: cycle::Count,
    snapshot_cycles: cycle::Count,
    absolute_diff: cycle::Diff,
    percentage_diff: cycle::PercentageDiff,
}

impl From<BenchmarkResult> for Row {
    fn from(result: BenchmarkResult) -> Self {
        let BenchmarkResult {
            name,
            actual_cycles,
            snapshot_cycles,
        } = result;

        Self {
            name,
            actual_cycles: actual_cycles.into(),
            snapshot_cycles: snapshot_cycles.into(),
            absolute_diff: cycle::Diff::new(actual_cycles, snapshot_cycles),
            percentage_diff: cycle::PercentageDiff::new(actual_cycles, snapshot_cycles),
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use tabled::Table;

    use super::*;

    #[test]
    fn prints_results_table() {
        let result = BenchmarkResult::new("test".to_string(), 1_010, 1_000);
        let rows: [Row; 1] = [result.into()];

        assert_snapshot!(Table::new(rows), @r###"
        +------+---------------+-----------------+---------------+-----------------+
        | name | actual_cycles | snapshot_cycles | absolute_diff | percentage_diff |
        +------+---------------+-----------------+---------------+-----------------+
        | test | 1_010         | 1_000           | 10            | 1.00 %          |
        +------+---------------+-----------------+---------------+-----------------+
        "###);
    }
}

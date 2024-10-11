#![cfg(test)]

use anyhow::Result;

use crate::Bencher;

#[test]
fn empty_benchmark() -> Result<()> {
    let bencher: Bencher = Default::default();

    let input = ();
    let result = bencher.run(input)?;

    assert!(result.total_cycles < 120);

    Ok(())
}

#![cfg(test)]

use anyhow::Result;

use super::Bencher;

#[test]
fn empty_benchmark() -> Result<()> {
    let bencher: Bencher = Default::default();

    let input = ();
    let result = bencher.run(input)?;

    assert_eq!(result, ());

    Ok(())
}

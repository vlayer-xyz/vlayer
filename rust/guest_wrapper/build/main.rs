#[cfg(not(clippy))]
mod builder;
#[cfg(not(clippy))]
mod data_layout;
#[cfg(not(clippy))]
mod utils;

pub fn main() -> anyhow::Result<()> {
    #[cfg(not(clippy))]
    builder::Builder::from_env().build()?;

    Ok(())
}

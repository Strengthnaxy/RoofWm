use crate::config::Config;
use anyhow::Result;

pub fn run(_cfg: Config) -> Result<()> {
    tracing::info!("compositor::run - implement smithay state and event loop here");
    Ok(())
}

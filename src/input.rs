use anyhow::Result;

pub fn init() -> Result<()> {
    tracing::info!("input::init - libinput backend goes here");
    Ok(())
}

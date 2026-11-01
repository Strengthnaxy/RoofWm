mod compositor;
mod config;
mod input;
mod layout;
mod output;

use anyhow::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "roofwm=info".into()),
        )
        .init();

    tracing::info!("roofwm starting");

    let cfg = config::load().unwrap_or_else(|e| {
        tracing::warn!("config not loaded ({e}), using defaults");
        config::Config::default()
    });
    tracing::info!(?cfg, "config");

    compositor::run(cfg)
}

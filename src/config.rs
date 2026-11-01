use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub layout: String,
    pub gaps: u32,
    pub border: u32,
    pub terminal: String,
    pub modifier: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            layout: "dwindle".into(),
            gaps: 8,
            border: 2,
            terminal: "foot".into(),
            modifier: "Super".into(),
        }
    }
}

pub fn load() -> anyhow::Result<Config> {
    let path = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("no config dir"))?
        .join("roof/roof.rs");
    let text = std::fs::read_to_string(&path)?;
    Ok(ron::from_str(&text)?)
}

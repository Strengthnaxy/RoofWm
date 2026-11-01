use anyhow::Result;

pub struct Output {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

impl Output {
    pub fn new(name: impl Into<String>, width: u32, height: u32) -> Self {
        Self { name: name.into(), width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn init() -> Result<Vec<Output>> {
    tracing::info!("output::init - enumerate drm outputs here");
    Ok(Vec::new())
}

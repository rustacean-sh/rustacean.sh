use anyhow::Result;

#[derive(Default)]
pub struct GitHub {}

impl GitHub {
    pub async fn stars(&self) -> Result<u32> {
        Ok(100)
    }
}

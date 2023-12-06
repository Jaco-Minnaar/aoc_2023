use anyhow::Result;

pub trait Solution {
    fn solve(&self, data: &str) -> Result<String>;
}

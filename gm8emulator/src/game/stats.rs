use serde::{Deserialize, Serialize};

pub const FILENAME: &str = "stats.info";

#[derive(Clone, Serialize, Deserialize)]
pub struct Stats {
    pub rerecords: u32,
    pub backups: u32,
}

impl Stats {
    pub fn from(rerecords: u32, backups: u32) -> Self {
        Self {
            rerecords,
            backups,
        }
    }
}
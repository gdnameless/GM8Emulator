use crate::game::string::RCStr;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Serialize, Deserialize)]
pub struct Sound {
    pub name: RCStr,
    pub source: Option<Arc<[u8]>>,
    pub kind: Kind,
    pub volume: f64,
    pub pan: f64,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Kind {
    Normal,
    BackgroundMusic,
    ThreeDimensional,
    Multimedia,
}

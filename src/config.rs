use serde::{Deserialize, Serialize};

use crate::style::StyleConfig;

#[derive(Serialize, Deserialize)]
pub struct Config {
    styles: Vec<StyleConfig>,
}

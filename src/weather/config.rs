use serde::{Serialize, Deserialize};

use super::{info::TempType, provider::ProviderType};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub date: Option<String>,
    pub address: Option<String>,
    pub temp_type: Option<TempType>,
    pub provider: Option<ProviderType>,
}
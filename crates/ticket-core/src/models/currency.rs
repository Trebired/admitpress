use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CurrencyOption {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};

/// Тема оформления интерфейса
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    /// Светлая тема
    Light,
    /// Тёмная тема
    Dark,
}

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};

/// Поддерживаемые языки интерфейса
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// Русский язык
    Ru,
    /// Английский язык
    En,
}

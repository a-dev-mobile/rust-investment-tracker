use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};

/// Единицы измерения
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[serde(rename_all = "lowercase")]
pub enum Unit {
    /// Миллиметры
    Mm,
    /// Дюймы
    Inch,
    /// Микроны
    Micron,
}

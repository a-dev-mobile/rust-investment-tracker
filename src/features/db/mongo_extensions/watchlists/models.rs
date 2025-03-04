use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

/// Модель для коллекции DbUserConfigWatchlists
#[derive(Debug, Serialize, Deserialize)]
pub struct DbUserConfigWatchlist {
    /// Уникальный идентификатор записи
    #[serde(rename = "_id")]
    pub id: ObjectId,

    /// Тикер инструмента
    pub ticker: String,

    /// Биржа
    pub exchange: String,

    /// Режим торгов
    pub trading_mode: String,

    /// ISIN код инструмента
    pub isin: String,

    pub figi: String,

    /// Флаг активности
    pub enabled: bool,

    /// Заметки/описание
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

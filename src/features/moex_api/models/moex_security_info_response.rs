use serde::{Serialize, Deserialize};



// Структуры для десериализации информации о ценной бумаге
#[derive(Debug, Serialize, Deserialize)]
pub struct MoexSecurityInfoResponse {
    pub description: SecurityDescription,
    pub boards: SecurityBoards,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityDescription {
    pub columns: Vec<String>,
    pub data: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityBoards {
    pub columns: Vec<String>,
    pub data: Vec<Vec<serde_json::Value>>,
}
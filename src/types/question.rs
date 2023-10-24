use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Serialize, Clone, Eq, PartialEq, Hash)]
pub struct QuestionId(pub String);

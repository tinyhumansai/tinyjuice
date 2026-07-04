use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenHumanCompressionContext {
    pub conversation_id: Option<String>,
    pub model: Option<String>,
}

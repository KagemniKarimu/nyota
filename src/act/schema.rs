use std::collections::HashMap;

pub struct ActionSchema {
    pub uid: i32,
    pub name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub action_type: ActionType,
    pub params: HashMap<String, String>,
}

pub enum ActionType {
    Tx,
    Query,
    ApiInteraction,
    Compound,
}

impl ActionSchema {
    pub fn new(
        uid: i32,
        name: String,
        short_desc: String,
        long_desc: String,
        action_type: ActionType,
        params: HashMap<String, String>,
    ) -> Self {
        Self {
            uid,
            name,
            short_desc,
            long_desc,
            action_type,
            params,
        }
    }
}

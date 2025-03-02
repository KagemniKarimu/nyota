pub trait ConversationStore {
    fn append_message(
        &mut self,
        key: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn get_message(&mut self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>>;
}

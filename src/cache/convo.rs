pub trait ConversationStore {
    fn append_message(
        &mut self,
        key: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn get_message(&mut self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // A simple dummy implementation of ConversationStore for testing.
    struct DummyStore;

    impl ConversationStore for DummyStore {
        fn append_message(
            &mut self,
            _key: &str,
            _message: &str,
        ) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn get_message(
            &mut self,
            _key: &str,
        ) -> Result<Option<String>, Box<dyn std::error::Error>> {
            Ok(Some("dummy".to_string()))
        }
    }

    #[test]
    fn test_dummy_store() {
        let mut store = DummyStore;
        store.append_message("dummy_key", "dummy_message").unwrap();
        let result = store.get_message("dummy_key").unwrap();
        assert_eq!(result, Some("dummy".to_string()));
    }
}

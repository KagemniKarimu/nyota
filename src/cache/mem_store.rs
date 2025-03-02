use crate::cache::convo::ConversationStore;
use lru::LruCache;
use std::num::NonZeroUsize;

pub struct InMemoryStore {
    cache: LruCache<String, String>,
}

impl InMemoryStore {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
        }
    }
}

impl ConversationStore for InMemoryStore {
    fn append_message(
        &mut self,
        key: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.cache.put(key.to_string(), message.to_string());
        Ok(())
    }

    fn get_message(&mut self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        Ok(self.cache.get(key).cloned())
    }
}

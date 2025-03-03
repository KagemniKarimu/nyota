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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::convo::ConversationStore;

    #[test]
    fn test_in_memory_store_append_and_get() {
        let mut store = InMemoryStore::new(10);
        let key = "test_key";
        let message = "Hello, world!";
        store.append_message(key, message).unwrap();
        let retrieved = store.get_message(key).unwrap();
        assert_eq!(retrieved, Some(message.to_string()));
    }
}

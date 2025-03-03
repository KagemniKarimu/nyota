use crate::cache::convo::ConversationStore;
use crate::cache::mem_store::InMemoryStore;
use crate::cache::redis_store::RedisStore;

pub struct HybridStore {
    local: InMemoryStore,
    redis: RedisStore,
}

impl HybridStore {
    pub fn new(local_capacity: usize, redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            local: InMemoryStore::new(local_capacity),
            redis: RedisStore::new(redis_url)?,
        })
    }
}

impl ConversationStore for HybridStore {
    fn append_message(
        &mut self,
        key: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Write to both caches
        self.local.append_message(key, message)?;
        self.redis.append_message(key, message)?;
        Ok(())
    }

    fn get_message(&mut self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        // First, check in-memory
        if let Some(msg) = self.local.get_message(key)? {
            return Ok(Some(msg));
        }
        // Fall back to Redis if missing in local cache
        if let Some(msg) = self.redis.get_message(key)? {
            // Optionally, update the in-memory cache
            self.local.append_message(key, &msg)?;
            return Ok(Some(msg));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::convo::ConversationStore;

    const TEST_REDIS_URL: &str = "redis://127.0.0.1/";

    #[test]
    fn test_hybrid_store_append_and_get() {
        let mut store = HybridStore::new(10, TEST_REDIS_URL).expect("Failed to create HybridStore");
        let key = "test_hybrid_key";
        let message = "Hybrid test message";
        store.append_message(key, message).unwrap();
        let retrieved = store.get_message(key).unwrap();
        assert_eq!(retrieved, Some(message.to_string()));
    }
}

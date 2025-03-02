use crate::cache::convo::ConversationStore;
use redis::{Client, Commands, Connection};

pub struct RedisStore {
    #[allow(dead_code)]
    client: Client,
    conn: Connection,
}

impl RedisStore {
    pub fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::open(redis_url)?;
        let conn = client.get_connection()?;
        Ok(Self { client, conn })
    }
}

impl ConversationStore for RedisStore {
    fn append_message(
        &mut self,
        key: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.set::<&str, &str, ()>(key, message)?;
        Ok(())
    }

    fn get_message(&mut self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let result: Option<String> = self.conn.get(key)?;
        Ok(result)
    }
}

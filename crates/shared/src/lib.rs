pub mod types {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AgentIdentity {
        pub id: Uuid,
        pub hostname: String,
        pub os: String,
        pub version: String,
    }
}

pub mod crypto {
    use anyhow::Result;

    pub fn hash_chain(prev: &[u8], entry: &[u8]) -> Result<Vec<u8>> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(prev);
        hasher.update(entry);
        Ok(hasher.finalize().to_vec())
    }
}

use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, new, Getters)]
#[getset(get = "pub")]
pub struct User {
    id: u64,
    name: String,
}

#[async_trait::async_trait]
pub trait UserRepository {
    async fn get_user();
}

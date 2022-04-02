use chrono::NaiveDateTime;
use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, new, Getters)]
#[getset(get = "pub")]
pub struct Todo {
    id: u64,
    content: String,
    create_time: NaiveDateTime,
}

#[async_trait::async_trait]
pub trait MemoRepository {
    async fn get_memo();
}

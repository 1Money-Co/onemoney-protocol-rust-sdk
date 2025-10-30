use crate::client::config::API_VERSION;
use crate::client::endpoints::governances::EPOCH_BY_ID;
use crate::responses::governances::EpochResponse;
use crate::{Client, Result};

impl Client {
    pub async fn get_epoch_by_id(&self, epoch: u64) -> Result<EpochResponse> {
        let path = format!("{}{}?id={}", API_VERSION, EPOCH_BY_ID, epoch);
        self.get(&path).await
    }
}

use anyhow::Result;
use serde::de::DeserializeOwned;

pub struct CliObj {
    pub api_key: String,
    reqwest_client: reqwest::Client,
    timestamp: u64,
    service_id: String,
    api_endpoint: &'static str,
}

pub trait TimestampHolder {
    fn get_timestamp(&self) -> u64;
}

impl CliObj {
    pub fn new(api_key: &str, service_id: &str, endpoint: &'static str) -> Result<CliObj> {
        let client = CliObj {
            api_key: api_key.to_string(),
            reqwest_client: reqwest::Client::builder().build()?,
            timestamp: 0,
            service_id: service_id.to_string(),
            api_endpoint: endpoint,
        };

        Ok(client)
    }

    pub fn reset_stats_consecutive(&mut self) {
        self.timestamp = 0;
    }

    pub async fn get_stats_consecutive<T: DeserializeOwned + TimestampHolder>(
        &mut self,
    ) -> Result<T> {
        let rt_stats = self.get_stats_from::<T>(self.timestamp).await?;

        self.timestamp = rt_stats.get_timestamp();

        Ok(rt_stats)
    }

    pub async fn get_stats_from<T: DeserializeOwned>(&mut self, start_timestamp: u64) -> Result<T> {
        let url = format!(
            "{}/{}/ts/{}",
            self.api_endpoint, self.service_id, start_timestamp
        );

        self.fetch_data(&url).await
    }

    pub async fn get_stats_120s<T: DeserializeOwned>(&self) -> Result<T> {
        let url = format!("{}/{}/ts/h", self.api_endpoint, self.service_id);

        self.fetch_data(&url).await
    }

    pub async fn get_stats_max<T: DeserializeOwned>(&self, max_entries: u64) -> Result<T> {
        let url = format!(
            "{}/{}/ts/h/limit/{}",
            self.api_endpoint, self.service_id, max_entries
        );

        self.fetch_data(&url).await
    }

    async fn fetch_data<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self
            .reqwest_client
            .get(url)
            .header("fastly-key", &self.api_key)
            .send()
            .await?;

        let rt_stats = response.error_for_status()?.json::<T>().await?;

        Ok(rt_stats)
    }
}

use crate::client::CliObj;
use crate::client::TimestampHolder;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response of real time data of origins of a service
#[derive(Debug, Serialize, Deserialize)]
pub struct OriginResponse {
    /// Offset of entry timestamps from the current time due to processing time.
    #[serde(alias = "AggregateDelay")]
    pub aggregate_delay: u64,
    
    /// A list of report entries, each representing one second of time.
    #[serde(alias = "Data")]
    pub data: Vec<OriginDataInSecond>,

    /// Timestamp value to use for subsequent requests.
    #[serde(alias = "Timestamp")]
    pub timestamp: u64,
}

impl TimestampHolder for OriginResponse {
    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

/// Hold data of all origins in one second of a service
#[derive(Debug, Serialize, Deserialize)]
pub struct OriginDataInSecond {
    /// The Unix timestamp at which this record's data was generated
    #[serde(default)]
    pub recorded: u64,

    /// Aggregates measurements across all Fastly POPs, of each origin
    /// Map of `origin_name` --> [`OriginStats`]
    #[serde(default)]
    pub aggregated: HashMap<String, OriginStats>,

    /// Measurements breakdown by POP
    /// Map of `pop_name` --> (map of `origin_name` --> [`OriginStats`])
    #[serde(default)]
    pub datacenter: HashMap<String, HashMap<String, OriginStats>>,
}

/// Statistics of origin
/// See explanation of members [here](https://developer.fastly.com/reference/api/metrics-stats/origin-inspector/real-time/#measurements-data-model)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OriginStats {
    #[serde(default)]
    pub resp_body_bytes: u64,

    #[serde(default)]
    pub resp_header_bytes: u64,

    #[serde(default)]
    pub responses: u64,

    #[serde(default)]
    status_1xx: u64,

    #[serde(default)]
    status_200: u64,

    #[serde(default)]
    status_204: u64,

    #[serde(default)]
    status_206: u64,

    #[serde(default)]
    status_2xx: u64,

    #[serde(default)]
    status_301: u64,

    #[serde(default)]
    status_302: u64,

    #[serde(default)]
    status_304: u64,

    #[serde(default)]
    status_3xx: u64,

    #[serde(default)]
    status_400: u64,

    #[serde(default)]
    status_401: u64,

    #[serde(default)]
    status_403: u64,

    #[serde(default)]
    status_404: u64,

    #[serde(default)]
    status_416: u64,

    #[serde(default)]
    status_429: u64,

    #[serde(default)]
    status_4xx: u64,

    #[serde(default)]
    status_500: u64,

    #[serde(default)]
    status_501: u64,

    #[serde(default)]
    status_502: u64,

    #[serde(default)]
    status_503: u64,

    #[serde(default)]
    status_504: u64,

    #[serde(default)]
    status_505: u64,

    #[serde(default)]
    status_5xx: u64,
}

/// Client to get origin real time data
pub struct OriginClient {
    cli: CliObj,
}

impl OriginClient {
    const ENDPOINT: &'static str = "https://rt.fastly.com/v1/origins";

    /// Create an OriginClient object
    pub fn new(api_key: &str, service_id: &str) -> Result<OriginClient> {
        Ok(OriginClient {
            cli: CliObj::new(api_key, service_id, OriginClient::ENDPOINT)?,
        })
    }

    /// Reset internal timestamp which used to track consecutive stats to 0
    /// After calling this function, calling get_stats_consecutive function will be the first call
    pub fn reset_stats_consecutive(&mut self) {
        self.cli.reset_stats_consecutive();
    }

    /// The first call of the function will get data of latest one second
    /// The consecutive call of the function will get consecutive data of last call to last second
    pub async fn get_stats_consecutive(&mut self) -> Result<OriginResponse> {
        self.cli.get_stats_consecutive().await
    }

    /// Get stats from start_timestamp to latest timestamp available for a service
    pub async fn get_stats_from(&mut self, start_timestamp: u64) -> Result<OriginResponse> {
        self.cli.get_stats_from(start_timestamp).await
    }

    /// Get data for the 120 seconds preceding the latest timestamp available for a service.
    pub async fn get_stats_120s(&self) -> Result<OriginResponse> {
        self.cli.get_stats_120s().await
    }

    /// Get data for the 120 seconds preceding the latest timestamp available for a service, up to a maximum of max_entries entries.
    pub async fn get_stats_max(&self, max_entries: u64) -> Result<OriginResponse> {
        self.cli.get_stats_max(max_entries).await
    }
}

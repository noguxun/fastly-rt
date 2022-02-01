use crate::client::CliObj;
use crate::client::TimestampHolder;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response of real time data of service
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceResponse {
    /// Offset of entry timestamps from the current time due to processing time.
    #[serde(alias = "AggregateDelay")]
    pub aggregate_delay: u64,

    /// A list of report entries, each representing one second of time.
    #[serde(alias = "Data")]
    pub data: Vec<ServiceDataInSecond>,

    /// Timestamp value to use for subsequent requests.
    #[serde(alias = "Timestamp")]
    pub timestamp: u64,
}

impl TimestampHolder for ServiceResponse {
    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

/// Data of service in one seconds
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceDataInSecond {
    /// The Unix timestamp at which this record's data was generated
    #[serde(default)]
    pub recorded: u64,

    /// Aggregates measurements across all Fastly POPs.
    #[serde(default)]
    pub aggregated: ServiceStats,

    /// Measurements breakdown by POP
    /// map of `pop_name` --> [`ServiceStats`]
    #[serde(default)]
    pub datacenter: HashMap<String, ServiceStats>,
}

/// Statistics of service
/// See explanation of members [here](https://developer.fastly.com/reference/api/metrics-stats/realtime/#measurements-data-model)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceStats {
    #[serde(default)]
    pub attack_blocked_req_body_bytes: u64,

    #[serde(default)]
    pub attack_blocked_req_header_bytes: u64,

    #[serde(default)]
    pub attack_logged_req_body_bytes: u64,

    #[serde(default)]
    pub attack_logged_req_header_bytes: u64,

    #[serde(default)]
    pub attack_passed_req_body_bytes: u64,

    #[serde(default)]
    pub attack_passed_req_header_bytes: u64,

    #[serde(default)]
    pub attack_req_body_bytes: u64,

    #[serde(default)]
    pub attack_req_header_bytes: u64,

    #[serde(default)]
    pub attack_resp_synth_bytes: u64,

    #[serde(default)]
    pub bereq_body_bytes: u64,

    #[serde(default)]
    pub bereq_header_bytes: u64,

    #[serde(default)]
    pub body_size: u64,

    #[serde(default)]
    pub compute_bereq_body_bytes: u64,

    #[serde(default)]
    pub compute_bereq_errors: u64,

    #[serde(default)]
    pub compute_bereq_header_bytes: u64,

    #[serde(default)]
    pub compute_bereqs: u64,

    #[serde(default)]
    pub compute_beresp_body_bytes: u64,

    #[serde(default)]
    pub compute_beresp_header_bytes: u64,

    #[serde(default)]
    pub compute_execution_time_ms: f64,

    #[serde(default)]
    pub compute_globals_limit_exceeded: u64,

    #[serde(default)]
    pub compute_guest_errors: u64,

    #[serde(default)]
    pub compute_heap_limit_exceeded: u64,

    #[serde(default)]
    pub compute_ram_used: u64,

    #[serde(default)]
    pub compute_req_body_bytes: u64,

    #[serde(default)]
    pub compute_req_header_bytes: u64,

    #[serde(default)]
    pub compute_request_time_ms: f64,

    #[serde(default)]
    pub compute_requests: u64,

    #[serde(default)]
    pub compute_resource_limit_exceeded: u64,

    #[serde(default)]
    pub compute_resp_body_bytes: u64,

    #[serde(default)]
    pub compute_resp_header_bytes: u64,

    #[serde(default)]
    pub compute_resp_status_1xx: u64,

    #[serde(default)]
    pub compute_resp_status_2xx: u64,

    #[serde(default)]
    pub compute_resp_status_3xx: u64,

    #[serde(default)]
    pub compute_resp_status_4xx: u64,

    #[serde(default)]
    pub compute_resp_status_5xx: u64,

    #[serde(default)]
    pub compute_runtime_errors: u64,

    #[serde(default)]
    pub compute_stack_limit_exceeded: u64,

    #[serde(default)]
    pub deliver_sub_count: u64,

    #[serde(default)]
    pub deliver_sub_time: f64,

    #[serde(default)]
    pub edge_hit_requests: u64,

    #[serde(default)]
    pub edge_hit_resp_body_bytes: u64,

    #[serde(default)]
    pub edge_hit_resp_header_bytes: u64,

    #[serde(default)]
    pub edge_miss_requests: u64,

    #[serde(default)]
    pub edge_miss_resp_body_bytes: u64,

    #[serde(default)]
    pub edge_miss_resp_header_bytes: u64,

    #[serde(default)]
    pub edge_requests: u64,

    #[serde(default)]
    pub edge_resp_body_bytes: u64,

    #[serde(default)]
    pub edge_resp_header_bytes: u64,

    #[serde(default)]
    pub error_sub_count: u64,

    #[serde(default)]
    pub error_sub_time: f64,

    #[serde(default)]
    pub errors: u64,

    #[serde(default)]
    pub fetch_sub_count: u64,

    #[serde(default)]
    pub fetch_sub_time: f64,

    #[serde(default)]
    pub hash_sub_count: u64,

    #[serde(default)]
    pub hash_sub_time: u64,

    #[serde(default)]
    pub header_size: u64,

    #[serde(default)]
    pub hit_resp_body_bytes: u64,

    #[serde(default)]
    pub hit_sub_count: u64,

    #[serde(default)]
    pub hit_sub_time: f64,

    #[serde(default)]
    pub hits: u64,

    #[serde(default)]
    pub hits_time: f64,

    #[serde(default)]
    pub http2: u64,

    #[serde(default)]
    pub http3: u64,

    #[serde(default)]
    pub imgopto: u64,

    #[serde(default)]
    pub imgopto_resp_body_bytes: u64,

    #[serde(default)]
    pub imgopto_resp_header_bytes: u64,

    #[serde(default)]
    pub imgopto_shield: u64,

    #[serde(default)]
    pub imgopto_shield_resp_body_bytes: u64,

    #[serde(default)]
    pub imgopto_shield_resp_header_bytes: u64,

    #[serde(default)]
    pub imgopto_transforms: u64,

    #[serde(default)]
    pub imgvideo: u64,

    #[serde(default)]
    pub imgvideo_frames: u64,

    #[serde(default)]
    pub imgvideo_resp_body_bytes: u64,

    #[serde(default)]
    pub imgvideo_resp_header_bytes: u64,

    #[serde(default)]
    pub imgvideo_shield: u64,

    #[serde(default)]
    pub imgvideo_shield_frames: u64,

    #[serde(default)]
    pub imgvideo_shield_resp_body_bytes: u64,

    #[serde(default)]
    pub imgvideo_shield_resp_header_bytes: u64,

    #[serde(default)]
    pub ipv6: u64,

    #[serde(default)]
    pub log: u64,

    #[serde(default)]
    pub log_bytes: u64,

    #[serde(default)]
    pub logging: u64,

    #[serde(default)]
    pub miss: u64,

    #[serde(default)]
    pub miss_histogram: HashMap<String, u64>,

    #[serde(default)]
    pub miss_resp_body_bytes: u64,

    #[serde(default)]
    pub miss_sub_count: u64,

    #[serde(default)]
    pub miss_sub_time: f64,

    #[serde(default)]
    pub miss_time: f64,

    #[serde(default)]
    pub object_size_100k: u64,

    #[serde(default)]
    pub object_size_100m: u64,

    #[serde(default)]
    pub object_size_10k: u64,

    #[serde(default)]
    pub object_size_10m: u64,

    #[serde(default)]
    pub object_size_1g: u64,

    #[serde(default)]
    pub object_size_1k: u64,

    #[serde(default)]
    pub object_size_1m: u64,

    #[serde(default)]
    pub object_size_other: u64,

    #[serde(default)]
    pub origin_cache_fetch_resp_body_bytes: u64,

    #[serde(default)]
    pub origin_cache_fetch_resp_header_bytes: u64,

    #[serde(default)]
    pub origin_cache_fetches: u64,

    #[serde(default)]
    pub origin_fetch_body_bytes: u64,

    #[serde(default)]
    pub origin_fetch_header_bytes: u64,

    #[serde(default)]
    pub origin_fetch_resp_body_bytes: u64,

    #[serde(default)]
    pub origin_fetch_resp_header_bytes: u64,

    #[serde(default)]
    pub origin_fetches: u64,

    #[serde(default)]
    pub origin_revalidations: u64,

    #[serde(default)]
    pub otfp: u64,

    #[serde(default)]
    pub otfp_deliver_time: f64,

    #[serde(default)]
    pub otfp_manifests: u64,

    #[serde(default)]
    pub otfp_resp_body_bytes: u64,

    #[serde(default)]
    pub otfp_resp_header_bytes: u64,

    #[serde(default)]
    pub otfp_shield: u64,

    #[serde(default)]
    pub otfp_shield_resp_body_bytes: u64,

    #[serde(default)]
    pub otfp_shield_resp_header_bytes: u64,

    #[serde(default)]
    pub otfp_shield_time: f64,

    #[serde(default)]
    pub pass: u64,

    #[serde(default)]
    pub pass_resp_body_bytes: u64,

    #[serde(default)]
    pub pass_sub_count: u64,

    #[serde(default)]
    pub pass_sub_time: f64,

    #[serde(default)]
    pub pass_time: f64,

    #[serde(default)]
    pub pci: u64,

    #[serde(default)]
    pub pipe_sub_count: u64,

    #[serde(default)]
    pub pipe_sub_time: f64,

    #[serde(default)]
    pub predeliver_sub_count: u64,

    #[serde(default)]
    pub predeliver_sub_time: f64,

    #[serde(default)]
    pub prehash_sub_count: u64,

    #[serde(default)]
    pub prehash_sub_time: f64,

    #[serde(default)]
    pub recv_sub_count: u64,

    #[serde(default)]
    pub recv_sub_time: f64,

    #[serde(default)]
    pub req_body_bytes: u64,

    #[serde(default)]
    pub req_header_bytes: u64,

    #[serde(default)]
    pub requests: u64,

    #[serde(default)]
    pub resp_body_bytes: u64,

    #[serde(default)]
    pub resp_header_bytes: u64,

    #[serde(default)]
    pub restarts: u64,

    #[serde(default)]
    pub segblock_origin_fetches: u64,

    #[serde(default)]
    pub segblock_shield_fetches: u64,

    #[serde(default)]
    pub shield: u64,

    #[serde(default)]
    pub shield_cache_fetches: u64,

    #[serde(default)]
    pub shield_fetch_body_bytes: u64,

    #[serde(default)]
    pub shield_fetch_header_bytes: u64,

    #[serde(default)]
    pub shield_fetch_resp_body_bytes: u64,

    #[serde(default)]
    pub shield_fetch_resp_header_bytes: u64,

    #[serde(default)]
    pub shield_fetches: u64,

    #[serde(default)]
    pub shield_resp_body_bytes: u64,

    #[serde(default)]
    pub shield_resp_header_bytes: u64,

    #[serde(default)]
    pub shield_revalidations: u64,

    #[serde(default)]
    pub status_1xx: u64,

    #[serde(default)]
    pub status_200: u64,

    #[serde(default)]
    pub status_204: u64,

    #[serde(default)]
    pub status_206: u64,

    #[serde(default)]
    pub status_2xx: u64,

    #[serde(default)]
    pub status_301: u64,

    #[serde(default)]
    pub status_302: u64,

    #[serde(default)]
    pub status_304: u64,

    #[serde(default)]
    pub status_3xx: u64,

    #[serde(default)]
    pub status_400: u64,

    #[serde(default)]
    pub status_401: u64,

    #[serde(default)]
    pub status_403: u64,

    #[serde(default)]
    pub status_404: u64,

    #[serde(default)]
    pub status_416: u64,

    #[serde(default)]
    pub status_429: u64,

    #[serde(default)]
    pub status_4xx: u64,

    #[serde(default)]
    pub status_500: u64,

    #[serde(default)]
    pub status_501: u64,

    #[serde(default)]
    pub status_502: u64,

    #[serde(default)]
    pub status_503: u64,

    #[serde(default)]
    pub status_504: u64,

    #[serde(default)]
    pub status_505: u64,

    #[serde(default)]
    pub status_5xx: u64,

    #[serde(default)]
    pub synth: u64,

    #[serde(default)]
    pub tls: u64,

    #[serde(default)]
    pub tls_v10: u64,

    #[serde(default)]
    pub tls_v11: u64,

    #[serde(default)]
    pub tls_v12: u64,

    #[serde(default)]
    pub tls_v13: u64,

    #[serde(default)]
    pub uncacheable: u64,

    #[serde(default)]
    pub video: u64,

    #[serde(default)]
    pub waf_blocked: u64,

    #[serde(default)]
    pub waf_logged: u64,

    #[serde(default)]
    pub waf_passed: u64,
}

/// Client to get service real time data
pub struct ServiceClient {
    cli: CliObj,
}

impl ServiceClient {
    const ENDPOINT: &'static str = "https://rt.fastly.com/v1/channel";

    /// Create a Service Client
    pub fn new(api_key: &str, service_id: &str) -> Result<ServiceClient> {
        Ok(ServiceClient {
            cli: CliObj::new(api_key, service_id, ServiceClient::ENDPOINT)?,
        })
    }

    /// Reset internal timestamp which used to track consecutive stats to 0
    /// After calling this function, calling get_stats_consecutive function will be the first call
    pub fn reset_stats_consecutive(&mut self) {
        self.cli.reset_stats_consecutive();
    }

    /// The first call of the function will get data of latest one second
    /// The consecutive call of the function will get consecutive data of last call to last second
    pub async fn get_stats_consecutive(&mut self) -> Result<ServiceResponse> {
        self.cli.get_stats_consecutive().await
    }

    /// Get stats from start_timestamp to latest
    pub async fn get_stats_from(&mut self, start_timestamp: u64) -> Result<ServiceResponse> {
        self.cli.get_stats_from(start_timestamp).await
    }

    /// Get data for the 120 seconds preceding the latest timestamp available for a service.
    pub async fn get_stats_120s(&self) -> Result<ServiceResponse> {
        self.cli.get_stats_120s().await
    }

    /// Get data for the 120 seconds preceding the latest timestamp available for a service, up to a maximum of max_entries entries.
    pub async fn get_stats_max(&self, max_entries: u64) -> Result<ServiceResponse> {
        self.cli.get_stats_max(max_entries).await
    }
}

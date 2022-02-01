use chrono::Utc;
use fastly_rt::service::ServiceClient;
use std::env;
use std::{thread, time};

fn load_api_key() -> String {
    env::var("KEY").expect("env KEY not set")
}

fn load_sid() -> String {
    env::var("SID").expect("env SID not set")
}

#[tokio::test]
async fn get_consecutive_stats() {
    let api_key = load_api_key();
    let sid = load_sid();
    let mut rt = ServiceClient::new(&api_key, &sid).unwrap();

    let mut rt_data = rt.get_stats_consecutive().await.unwrap();
    let timestamp = rt_data.timestamp;

    thread::sleep(time::Duration::from_secs(3));

    rt_data = rt.get_stats_consecutive().await.unwrap();
    println!("{}", timestamp);
    println!("{}", rt_data.timestamp);

    assert!(timestamp > 0);
    assert!(rt_data.timestamp > timestamp + 2);

    rt.reset_stats_consecutive();
    rt_data = rt.get_stats_consecutive().await.unwrap();
    assert!(rt_data.data.len() == 0 || rt_data.data.len() == 1)
}

#[tokio::test]
async fn get_stats_from() {
    let api_key = load_api_key();
    let sid = load_sid();
    let mut rt = ServiceClient::new(&api_key, &sid).unwrap();

    let timestamp = (Utc::now().timestamp() - 5) as u64;

    let rt_data = rt.get_stats_from(timestamp).await.unwrap();
    println!("{} {}", timestamp, rt_data.timestamp);

    assert!(timestamp < rt_data.timestamp);
}

#[tokio::test]
async fn get_stats_120s() {
    let api_key = load_api_key();
    let sid = load_sid();
    let rt = ServiceClient::new(&api_key, &sid).unwrap();

    let rt_data = rt.get_stats_120s().await.unwrap();
    let entry_count = rt_data.data.len();
    println!("{}", entry_count);

    assert!(entry_count == 0 || entry_count == 120);
}

#[tokio::test]
async fn get_stats_max() {
    let api_key = load_api_key();
    let sid = load_sid();
    let rt = ServiceClient::new(&api_key, &sid).unwrap();

    let mut rt_data = rt.get_stats_max(3).await.unwrap();
    let mut entry_count = rt_data.data.len();
    println!("{}", entry_count);

    assert!(entry_count == 0 || entry_count == 3);

    rt_data = rt.get_stats_max(5).await.unwrap();
    entry_count = rt_data.data.len();
    println!("{}", entry_count);

    assert!(entry_count == 0 || entry_count == 5);
}

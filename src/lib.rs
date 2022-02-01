// Warnings (other than unused variables) in doctests are promoted to errors.
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::invalid_codeblock_attributes)]

//! # Client for Fastly Real Time API 
//! The library contains wapper for Fastly's [Real-time analytics](https://developer.fastly.com/reference/api/metrics-stats/realtime/)
//! and [Real-time origin metrics](https://developer.fastly.com/reference/api/metrics-stats/origin-inspector/real-time/)
//! 
//! ## Real-time analytics
//! Real-time analytics provides statistics of a service.
//! Related structures are [`service::ServiceResponse`], [`service::ServiceDataInSecond`], [`service::ServiceStats`]
//! 
//! To get statistic concecutively 
//! ```ignore
//! use fastly_rt::service::ServiceClient;
//! use std::env;
//! 
//! let api_key = env::var("KEY").expect("env KEY not set");
//! let sid = env::var("SID").expect("env SID not set");
//! 
//! let mut rt = ServiceClient::new(&api_key, &sid).unwrap();
//! 
//! while (true) {
//!     let rt_data = rt.get_stats_consecutive().await.unwrap();
//!    
//!     for data in rt_data.data {
//!         println!("time {}, number of requests {}", data.recorded, data.aggregated.requests);
//!     }
//! }
//! ```
//! 
//! To get statistic of last 120 seconds
//! ```ignore
//! use fastly_rt::service::ServiceClient;
//! use std::env;
//! 
//! let api_key = env::var("KEY").expect("env KEY not set");
//! let sid = env::var("SID").expect("env SID not set");
//! 
//! let mut rt = ServiceClient::new(&api_key, &sid).unwrap();
//!
//! let rt_data = rt.get_stats_120s().await.unwrap();
//!    
//! for data in rt_data.data {
//!     println!("time {}, number of requests {}", data.recorded, data.aggregated.requests);
//! }
//! ```
//! 
//! To get statistic of last 10 seconds
//! ```ignore
//! use fastly_rt::service::ServiceClient;
//! use std::env;
//! 
//! let api_key = env::var("KEY").expect("env KEY not set");
//! let sid = env::var("SID").expect("env SID not set");
//! 
//! let mut rt = ServiceClient::new(&api_key, &sid).unwrap();
//!
//! let rt_data = rt.get_stats_max(10).await.unwrap();
//!    
//! for data in rt_data.data {
//!     println!("time {}, number of requests {}", data.recorded, data.aggregated.requests);
//! }
//! ```
//! ## Real-time origin metrics
//! Real-time origin metrics provides statistics of origins of a service.
//! Related structures are [`origin::OriginResponse`], [`origin::OriginDataInSecond`], [`origin::OriginStats`]
//! 
//! Examples are similar to that Real-time origin metrics

mod client;
pub mod origin;
pub mod service;

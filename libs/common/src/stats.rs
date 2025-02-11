use crate::connection_counter::get_connection_counter;
use crate::memory_stats_loop::{get_fd_count, get_memory_stats};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pid: u32,
    memory_usage: f64,
    fd_count: usize,
    connections: i64,
    hostname: String,
}

impl Stats {
    pub fn new() -> Self {
        let pid = process::id();
        let connections = get_connection_counter();
        let fd_count = get_fd_count();
        let memory_usage = get_memory_stats();
        let hostname: String = hostname::get()
            .unwrap_or_default()
            .into_string()
            .unwrap_or_default();

        Self {
            pid,
            connections,
            fd_count,
            memory_usage,
            hostname,
        }
    }
}

pub async fn stats() -> impl IntoResponse {
    let stats = Stats::new();
    Json(stats)
}

use std::time::Duration;

use crate::http::HttpServer;

/// Multi-producer, multi-consumer channel implementation
pub mod mpmc;

/// Tests for "Phase 1" of the assignment
#[cfg(test)]
pub mod phase1;

/// Tests for "Phase 2" of the assignment
#[cfg(test)]
pub mod phase2;

/// Tests for "Phase 3" of the assignment
#[cfg(test)]
pub mod phase3;

/// HTTP server implementation
pub mod http;

fn main() {
    let server = HttpServer::new(8000, 16);
    std::thread::sleep(Duration::MAX);
    drop(server);
}

//! The main module containing the `fn main()` entrypoint. Feel free to write quick test code here.

// We're disabling some noisy compiler lints here
#![allow(unused_imports)]
#![allow(clippy::new_without_default)]

/// Your implementation will go here (`graph.rs`).
pub mod graph;
use graph::*;

/// Tests are provided for you (`tests.rs`);
#[cfg(test)]
pub mod tests;

fn main() {
    let mut graph = Graph::new();

    // We need to use `.to_owned()` to convert `&str` into `String`
    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());
    let c = graph.add("C".to_owned());

    graph.connect(a, b);
    graph.connect(b, c);
    graph.connect(c, a);

    dbg!(&graph);

    println!("Use `cargo test` to run the test suite!");
}

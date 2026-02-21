use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use crate::graph::*;

/// Timeout to use for the stress test (increase this if your system is slow)
pub const STRESS_TEST_TIMEOUT: Duration = Duration::from_secs(3);

/// Prints out a node and its neighbors as a string
/// e.g. "A -> (B, C)" if "A" has out-neighbors "B" and "C"
pub fn neighbors_str(graph: &Graph, id: NodeId) -> String {
    let mut result = String::new();

    result += graph.value(id);
    result += " -> (";

    // Sort neighbors to prevent implementation-specific test behavior
    let mut neighbor_values: Vec<String> = graph
        .neighbors(id)
        .into_iter()
        .map(|id| graph.value(id).to_owned())
        .collect();
    neighbor_values.sort();

    let mut first = true;
    for value in neighbor_values {
        if !first {
            result += ", ";
        }
        result += &value;
        first = false;
    }
    result += ")";
    result
}

// Comprehensive test suite

// ^^^ Feel free to write your own tests here ^^^

#[test]
fn test_constructor() {
    let graph = Graph::new();
    assert_eq!(graph.ids().len(), 0);
}

#[test]
fn test_add_node() {
    let mut graph = Graph::new();
    let a = graph.add("A".to_owned());
    assert_eq!(graph.ids().len(), 1);
    assert_eq!(neighbors_str(&graph, a), "A -> ()");
}

#[test]
fn test_add_remove() {
    let mut graph = Graph::new();
    let a = graph.add("A".to_owned());
    graph.remove(a);
    assert_eq!(graph.ids().len(), 0);
}

#[test]
fn test_add_contains() {
    let mut graph = Graph::new();
    let a = graph.add("A".to_owned());
    assert!(graph.contains(a));
}

#[test]
fn test_remove_contains() {
    let mut graph = Graph::new();
    let a = graph.add("A".to_owned());
    graph.remove(a);
    assert!(!graph.contains(a));
}

#[test]
#[should_panic]
fn test_add_remove_illegal_access() {
    let mut graph = Graph::new();
    let a = graph.add("A".to_owned());
    graph.remove(a);
    let _ = graph.value(a);
}

#[test]
fn test_add_many_nodes() {
    let mut graph = Graph::new();

    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());
    let c = graph.add("C".to_owned());
    let d = graph.add("D".to_owned());
    let e = graph.add("E".to_owned());

    assert_eq!(graph.ids().len(), 5);

    assert_eq!(neighbors_str(&graph, a), "A -> ()");
    assert_eq!(neighbors_str(&graph, b), "B -> ()");
    assert_eq!(neighbors_str(&graph, c), "C -> ()");
    assert_eq!(neighbors_str(&graph, d), "D -> ()");
    assert_eq!(neighbors_str(&graph, e), "E -> ()");
}

#[test]
fn test_connect() {
    let mut graph = Graph::new();

    let from = graph.add("From".to_owned());
    let to = graph.add("To".to_owned());

    graph.connect(from, to);

    assert_eq!(neighbors_str(&graph, from), "From -> (To)");
    assert_eq!(neighbors_str(&graph, to), "To -> ()");
}

#[test]
fn test_connect_twice() {
    let mut graph = Graph::new();

    let from = graph.add("From".to_owned());
    let to = graph.add("To".to_owned());

    graph.connect(from, to);
    graph.connect(from, to);

    assert_eq!(neighbors_str(&graph, from), "From -> (To)");
    assert_eq!(neighbors_str(&graph, to), "To -> ()");
}

#[test]
fn test_connect_cycle() {
    let mut graph = Graph::new();

    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());
    let c = graph.add("C".to_owned());
    let d = graph.add("D".to_owned());
    let e = graph.add("E".to_owned());

    assert_eq!(graph.ids().len(), 5);

    graph.connect(a, b);
    graph.connect(b, c);
    graph.connect(c, d);
    graph.connect(d, e);
    graph.connect(e, a);

    assert_eq!(neighbors_str(&graph, a), "A -> (B)");
    assert_eq!(neighbors_str(&graph, b), "B -> (C)");
    assert_eq!(neighbors_str(&graph, c), "C -> (D)");
    assert_eq!(neighbors_str(&graph, d), "D -> (E)");
    assert_eq!(neighbors_str(&graph, e), "E -> (A)");
}

#[test]
fn test_connect_two_way_cycle() {
    let mut graph = Graph::new();

    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());
    let c = graph.add("C".to_owned());
    let d = graph.add("D".to_owned());
    let e = graph.add("E".to_owned());

    assert_eq!(graph.ids().len(), 5);

    graph.connect(a, b);
    graph.connect(b, c);
    graph.connect(c, d);
    graph.connect(d, e);
    graph.connect(e, a);

    graph.connect(a, e);
    graph.connect(b, a);
    graph.connect(c, b);
    graph.connect(d, c);
    graph.connect(e, d);

    assert_eq!(neighbors_str(&graph, a), "A -> (B, E)");
    assert_eq!(neighbors_str(&graph, b), "B -> (A, C)");
    assert_eq!(neighbors_str(&graph, c), "C -> (B, D)");
    assert_eq!(neighbors_str(&graph, d), "D -> (C, E)");
    assert_eq!(neighbors_str(&graph, e), "E -> (A, D)");
}

#[test]
fn test_remove_same_order() {
    let mut graph = Graph::new();

    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());
    let c = graph.add("C".to_owned());

    assert!(graph.contains(a));
    assert!(graph.contains(b));
    assert!(graph.contains(c));

    graph.remove(a);
    graph.remove(b);
    graph.remove(c);

    assert!(!graph.contains(a));
    assert!(!graph.contains(b));
    assert!(!graph.contains(c));

    assert_eq!(graph.ids().len(), 0);
}

#[test]
fn test_remove_reverse_order() {
    let mut graph = Graph::new();

    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());
    let c = graph.add("C".to_owned());

    assert!(graph.contains(a));
    assert!(graph.contains(b));
    assert!(graph.contains(c));

    graph.remove(c);
    graph.remove(b);
    graph.remove(a);

    assert!(!graph.contains(a));
    assert!(!graph.contains(b));
    assert!(!graph.contains(c));

    assert_eq!(graph.ids().len(), 0);
}

#[test]
fn test_two_way_connection() {
    let mut graph = Graph::new();

    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());

    graph.connect(a, b);
    graph.connect(b, a);

    assert_eq!(graph.ids().len(), 2);
    assert_eq!(neighbors_str(&graph, a), "A -> (B)");
    assert_eq!(neighbors_str(&graph, b), "B -> (A)");
}

#[test]
fn test_disconnect() {
    let mut graph = Graph::new();

    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());

    graph.connect(a, b);
    graph.connect(b, a);

    assert_eq!(neighbors_str(&graph, a), "A -> (B)");
    assert_eq!(neighbors_str(&graph, b), "B -> (A)");

    graph.disconnect(a, b);

    assert_eq!(neighbors_str(&graph, a), "A -> ()");
    assert_eq!(neighbors_str(&graph, b), "B -> (A)");

    graph.disconnect(b, a);

    assert_eq!(neighbors_str(&graph, a), "A -> ()");
    assert_eq!(neighbors_str(&graph, b), "B -> ()");
}

#[test]
fn test_disconnect_nonexistent_edge() {
    let mut graph = Graph::new();

    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());

    graph.disconnect(a, b);
    graph.disconnect(b, a);

    assert_eq!(neighbors_str(&graph, a), "A -> ()");
    assert_eq!(neighbors_str(&graph, b), "B -> ()");
}

#[test]
fn test_add_remove_highly_connected() {
    let mut graph = Graph::new();

    let mid = graph.add("Mid".to_owned());
    let in1 = graph.add("In".to_owned());
    let in2 = graph.add("In".to_owned());
    let out1 = graph.add("Out".to_owned());
    let out2 = graph.add("Out".to_owned());

    graph.connect(in1, mid);
    graph.connect(in2, mid);
    graph.connect(mid, out1);
    graph.connect(mid, out2);

    graph.connect(in1, in2);
    graph.connect(in2, out1);
    graph.connect(out1, out2);
    graph.connect(out2, in1);

    assert_eq!(graph.ids().len(), 5);

    assert_eq!(neighbors_str(&graph, mid), "Mid -> (Out, Out)");
    assert_eq!(neighbors_str(&graph, in1), "In -> (In, Mid)");
    assert_eq!(neighbors_str(&graph, in2), "In -> (Mid, Out)");
    assert_eq!(neighbors_str(&graph, out1), "Out -> (Out)");
    assert_eq!(neighbors_str(&graph, out2), "Out -> (In)");

    graph.remove(mid);
    assert_eq!(graph.ids().len(), 4);

    assert_eq!(neighbors_str(&graph, in1), "In -> (In)");
    assert_eq!(neighbors_str(&graph, in2), "In -> (Out)");
    assert_eq!(neighbors_str(&graph, out1), "Out -> (Out)");
    assert_eq!(neighbors_str(&graph, out2), "Out -> (In)");

    let mid = graph.add("Mid".to_owned());
    assert_eq!(graph.ids().len(), 5);

    assert_eq!(neighbors_str(&graph, in1), "In -> (In)");
    assert_eq!(neighbors_str(&graph, in2), "In -> (Out)");
    assert_eq!(neighbors_str(&graph, out1), "Out -> (Out)");
    assert_eq!(neighbors_str(&graph, out2), "Out -> (In)");

    graph.connect(in1, mid);
    graph.connect(in2, mid);
    graph.connect(mid, out1);
    graph.connect(mid, out2);

    assert_eq!(neighbors_str(&graph, mid), "Mid -> (Out, Out)");
    assert_eq!(neighbors_str(&graph, in1), "In -> (In, Mid)");
    assert_eq!(neighbors_str(&graph, in2), "In -> (Mid, Out)");
    assert_eq!(neighbors_str(&graph, out1), "Out -> (Out)");
    assert_eq!(neighbors_str(&graph, out2), "Out -> (In)");
}

#[test]
#[should_panic]
fn test_connect_removed_from() {
    let mut graph = Graph::new();
    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());
    graph.remove(a);
    graph.connect(a, b);
}

#[test]
#[should_panic]
fn test_connect_removed_to() {
    let mut graph = Graph::new();
    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());
    graph.remove(b);
    graph.connect(a, b);
}

#[test]
#[should_panic]
fn test_disconnect_removed_from() {
    let mut graph = Graph::new();
    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());
    graph.connect(a, b);
    graph.remove(a);
    graph.disconnect(a, b);
}

#[test]
#[should_panic]
fn test_disconnect_removed_to() {
    let mut graph = Graph::new();
    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());
    graph.connect(a, b);
    graph.remove(b);
    graph.disconnect(a, b);
}

#[test]
fn test_aba_problem() {
    let mut graph = Graph::new();

    let old = graph.add("Old".to_owned());
    graph.remove(old);
    let new = graph.add("New".to_owned());

    assert!(!graph.contains(old));
    assert!(graph.contains(new));

    assert_eq!(graph.ids().len(), 1);

    assert_eq!(neighbors_str(&graph, new), "New -> ()");
}

#[test]
#[should_panic]
fn test_aba_problem_illegal_access() {
    let mut graph = Graph::new();

    let old = graph.add("Old".to_owned());
    graph.remove(old);
    let _ = graph.add("New".to_owned());

    assert_eq!(neighbors_str(&graph, old), "Old -> ()");
}

#[test]
fn test_aba_problem_neighbors() {
    let mut graph = Graph::new();

    let sink = graph.add("Sink".to_owned());
    let old = graph.add("Source".to_owned());

    graph.connect(old, sink);
    assert_eq!(neighbors_str(&graph, old), "Source -> (Sink)");

    graph.remove(old);
    let new = graph.add("Source".to_owned());

    assert_eq!(neighbors_str(&graph, new), "Source -> ()");
}

#[test]
fn test_rename() {
    let mut graph = Graph::new();
    let id = graph.add("Old Name".to_owned());
    *graph.value_mut(id) = "New Name".to_owned();
    assert_eq!(graph.ids().len(), 1);
    assert_eq!(neighbors_str(&graph, id), "New Name -> ()");
}

#[test]
fn test_self_edge() {
    let mut graph = Graph::new();
    let id = graph.add("Self".to_owned());
    graph.connect(id, id);
    assert_eq!(graph.ids().len(), 1);
    assert_eq!(neighbors_str(&graph, id), "Self -> (Self)");
}

#[test]
fn test_self_edge_remove() {
    let mut graph = Graph::new();
    let id = graph.add("Self".to_owned());
    graph.connect(id, id);
    graph.remove(id);
    assert_eq!(graph.ids().len(), 0);
}

#[test]
#[should_panic]
fn test_self_multi_edge() {
    let mut graph = Graph::new();
    let id = graph.add("Self".to_owned());
    graph.connect(id, id);
    graph.connect(id, id);
    assert_eq!(graph.ids().len(), 1);
    assert_eq!(neighbors_str(&graph, id), "Self -> (Self, Self)");
}

#[test]
fn test_same_name() {
    let mut graph = Graph::new();

    let n1 = graph.add("Node".to_owned());
    let n2 = graph.add("Node".to_owned());
    let n3 = graph.add("Node".to_owned());

    graph.connect(n1, n2);
    graph.connect(n1, n3);
    graph.connect(n2, n3);

    assert_eq!(graph.ids().len(), 3);

    assert_eq!(neighbors_str(&graph, n1), "Node -> (Node, Node)");
    assert_eq!(neighbors_str(&graph, n2), "Node -> (Node)");
    assert_eq!(neighbors_str(&graph, n3), "Node -> ()");

    graph.remove(n1);
    assert_eq!(graph.ids().len(), 2);
    graph.remove(n2);
    assert_eq!(graph.ids().len(), 1);
    graph.remove(n3);
    assert_eq!(graph.ids().len(), 0);
}

#[test]
fn test_remove_neighbors() {
    let mut graph = Graph::new();

    let source = graph.add("Source".to_owned());
    let sink = graph.add("Sink".to_owned());

    graph.connect(source, sink);

    assert_eq!(graph.ids().len(), 2);

    assert_eq!(neighbors_str(&graph, source), "Source -> (Sink)");
    assert_eq!(neighbors_str(&graph, sink), "Sink -> ()");

    graph.remove(sink);

    assert_eq!(neighbors_str(&graph, source), "Source -> ()");
}

#[test]
fn test_hub_topology() {
    let mut graph = Graph::new();

    let center = graph.add("Center".to_owned());
    let mut nodes = Vec::new();

    for i in 0..1000 {
        let node = graph.add(format!("Node {}", i));
        graph.connect(center, node);
        graph.connect(node, center);
        nodes.push(node);
    }

    assert_eq!(graph.neighbors(center).len(), 1000);

    graph.remove(center);

    for node in nodes {
        assert_eq!(graph.neighbors(node).len(), 0);
    }

    assert!(!graph.contains(center));
    assert_eq!(graph.ids().len(), 1000);
}

#[test]
fn test_ids_excludes_removed_nodes() {
    let mut graph = Graph::new();

    let a = graph.add("A".to_owned());
    let b = graph.add("B".to_owned());
    let c = graph.add("C".to_owned());

    let ids = graph.ids();
    assert_eq!(ids.len(), 3);

    graph.remove(b);

    let ids = graph.ids();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&a));
    assert!(!ids.contains(&b));
    assert!(ids.contains(&c));
}

#[test]
fn test_empty_string_value() {
    let mut graph = Graph::new();
    let empty = graph.add("".to_owned());

    assert_eq!(graph.value(empty), "");
    assert_eq!(neighbors_str(&graph, empty), " -> ()");
}

#[test]
fn test_unicode_string_value() {
    let mut graph = Graph::new();
    let unicode = graph.add("🦀🔥💯".to_owned());

    assert_eq!(graph.value(unicode), "🦀🔥💯");
    assert_eq!(neighbors_str(&graph, unicode), "🦀🔥💯 -> ()");
}

#[test]
fn stress_test_add_remove() {
    let thread = std::thread::spawn(|| {
        let mut graph = Graph::new();
        // Repeat 10,000 times for a total of 10 GB
        for _ in 0..10_000 {
            // 1 MB string
            let huge_string = "x".repeat(1_000_000);
            let node = graph.add(huge_string);
            graph.remove(node);
        }
        assert_eq!(graph.ids().len(), 0);
    });
    let start = Instant::now();
    loop {
        if start.elapsed() > STRESS_TEST_TIMEOUT {
            panic!("timed out (are you leaking memory?)");
        }
        if thread.is_finished() {
            thread.join().expect("test panicked");
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

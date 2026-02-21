use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    new_id: u32,
    nodes: HashMap<NodeId, NodeData>,
}

#[derive(Debug)]
struct NodeData {
    value: String,
    neighbors: Vec<NodeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {
    id: u32,
}

// ^^^ Feel free to add your own structs as well ^^^

// This is called an "impl block", which implements functions associated with the `Graph` type.
// Within here you will see the `self` keyword, which refers to an implicit first argument.
// - e.g. calling `my_graph.get(id)` is equivalent to `Graph::get(&my_graph, id)`
#[allow(unused_variables)]
impl Graph {
    /// Constructs a new  graph. To call this function use `Graph::new()`.
    pub fn new() -> Graph {
        Graph {
            new_id: 0,
            nodes: HashMap::new(),
        }
    }

    /// Adds a node to the graph with the given value, returning a unique id
    pub fn add(self: &mut Graph, value: String) -> NodeId {
        let curr_node = NodeId { id: self.new_id };
        self.nodes.insert(
            curr_node,
            NodeData {
                value,
                neighbors: Vec::new(),
            },
        );
        self.new_id += 1;
        curr_node
    }

    /// Returns true if the graph contains the given id
    pub fn contains(self: &Graph, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    /// Removes a node from the graph, as well as any edges associated with it
    pub fn remove(self: &mut Graph, id: NodeId) {
        if !self.contains(id) {
            panic!("cannot remove nonexistent node");
        }

        // remove incoming edges
        for other_node in self.nodes.values_mut() {
            other_node.neighbors.retain(|&x| x != id);
        }

        // remove from the graph (and outgoing edges)
        self.nodes.remove(&id);
    }

    /// Immutably borrows the value stored in a node
    pub fn value(self: &Graph, id: NodeId) -> &String {
        &self.nodes.get(&id).unwrap().value
    }

    /// Mutably borrows the value stored in a node
    pub fn value_mut(self: &mut Graph, id: NodeId) -> &mut String {
        &mut self.nodes.get_mut(&id).unwrap().value
    }

    /// Adds a directed edge from the `from` node to the `to` node (if it doesn't exist)
    pub fn connect(self: &mut Graph, from: NodeId, to: NodeId) {
        if !self.contains(from) || !self.contains(to) {
            panic!("node does not exist");
        }

        let neighbors = &mut self.nodes.get_mut(&from).unwrap().neighbors;

        if neighbors.contains(&to) {
            if from == to {
                panic!("self multi edge not allowed");
            }
            return;
        }

        neighbors.push(to);
    }

    /// Removes a directed edge from the `from` node to the `to` node (if it exists)
    pub fn disconnect(self: &mut Graph, from: NodeId, to: NodeId) {
        if !self.contains(from) || !self.contains(to) {
            panic!("node does not exist");
        }

        self.nodes
            .get_mut(&from)
            .unwrap()
            .neighbors
            .retain(|&x| x != to);
    }

    /// Returns a list of (valid) node ids present in the graph
    pub fn ids(self: &Graph) -> Vec<NodeId> {
        self.nodes.keys().cloned().collect()
    }

    /// Returns the list of (out) neighbors of a node
    pub fn neighbors(self: &Graph, id: NodeId) -> Vec<NodeId> {
        self.nodes.get(&id).unwrap().neighbors.clone()
    }
}

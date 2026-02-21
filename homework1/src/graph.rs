use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    new_id: u32,
    values: HashMap<NodeId, String>,
    adjlist: HashMap<NodeId, Vec<NodeId>>,
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
        return Graph {
            new_id: 0,
            values: HashMap::new(),
            adjlist: HashMap::new(),
        };
    }

    /// Adds a node to the graph with the given value, returning a unique id
    pub fn add(self: &mut Graph, value: String) -> NodeId {
        let curr_node = NodeId { id: self.new_id };
        self.values.insert(curr_node, value);
        self.adjlist.insert(curr_node, Vec::new());
        self.new_id += 1;
        return curr_node;
    }

    /// Returns true if the graph contains the given id
    pub fn contains(self: &Graph, id: NodeId) -> bool {
        return self.values.contains_key(&id);
    }

    /// Removes a node from the graph, as well as any edges associated with it
    pub fn remove(self: &mut Graph, id: NodeId) {
        if !self.contains(id) {
            panic!("cannot remove nonexistent node");
        }

        // remove incoming edges
        for node in self.ids() {
            let neighbors = self.adjlist.get_mut(&node).unwrap();
            neighbors.retain(|&x| x != id);
        }

        // remove from the graph (and outgoing edges)
        self.values.remove(&id);
        self.adjlist.remove(&id);
    }

    /// Immutably borrows the value stored in a node
    pub fn value(self: &Graph, id: NodeId) -> &String {
        let value = self.values.get(&id);
        return value.unwrap(); // will panic if doesn't exist
    }

    /// Mutably borrows the value stored in a node
    pub fn value_mut(self: &mut Graph, id: NodeId) -> &mut String {
        let value = self.values.get_mut(&id);
        return value.unwrap();
    }

    /// Adds a directed edge from the `from` node to the `to` node (if it doesn't exist)
    pub fn connect(self: &mut Graph, from: NodeId, to: NodeId) {
        if !self.contains(from) || !self.contains(to) {
            panic!("node does not exist");
        }

        if !self.adjlist.get(&from).unwrap().contains(&to) {
            self.adjlist.get_mut(&from).unwrap().push(to);
        } else if from == to {
            panic!("self multi edge not allowed");
        }
    }

    /// Removes a directed edge from the `from` node to the `to` node (if it exists)
    pub fn disconnect(self: &mut Graph, from: NodeId, to: NodeId) {
        if !self.contains(from) || !self.contains(to) {
            panic!("node does not exist");
        }

        self.adjlist.get_mut(&from).unwrap().retain(|x| *x != to);
    }

    /// Returns a list of (valid) node ids present in the graph
    pub fn ids(self: &Graph) -> Vec<NodeId> {
        return self.values.keys().cloned().collect();
    }

    /// Returns the list of (out) neighbors of a node
    pub fn neighbors(self: &Graph, id: NodeId) -> Vec<NodeId> {
        let neighbors = self.adjlist.get(&id);
        return neighbors.unwrap().clone();
    }
}

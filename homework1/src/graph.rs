#[derive(Debug)]
pub struct Graph {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {}

// ^^^ Feel free to add your own structs as well ^^^

// This is called an "impl block", which implements functions associated with the `Graph` type.
// Within here you will see the `self` keyword, which refers to an implicit first argument.
// - e.g. calling `my_graph.get(id)` is equivalent to `Graph::get(&my_graph, id)`
#[allow(unused_variables)]
impl Graph {
    /// Constructs a new  graph. To call this function use `Graph::new()`.
    pub fn new() -> Graph {
        unimplemented!("new");
    }

    /// Adds a node to the graph with the given value, returning a unique id
    pub fn add(self: &mut Graph, value: String) -> NodeId {
        unimplemented!("add");
    }

    /// Returns true if the graph contains the given id
    pub fn contains(self: &Graph, id: NodeId) -> bool {
        unimplemented!("contains");
    }

    /// Removes a node from the graph, as well as any edges associated with it
    pub fn remove(self: &mut Graph, id: NodeId) {
        unimplemented!("remove");
    }

    /// Immutably borrows the value stored in a node
    pub fn value(self: &Graph, id: NodeId) -> &String {
        unimplemented!("value");
    }

    /// Mutably borrows the value stored in a node
    pub fn value_mut(self: &mut Graph, id: NodeId) -> &mut String {
        unimplemented!("value_mut");
    }

    /// Adds a directed edge from the `from` node to the `to` node (if it doesn't exist)
    pub fn connect(self: &mut Graph, from: NodeId, to: NodeId) {
        unimplemented!("connect");
    }

    /// Removes a directed edge from the `from` node to the `to` node (if it exists)
    pub fn disconnect(self: &mut Graph, from: NodeId, to: NodeId) {
        unimplemented!("disconnect");
    }

    /// Returns a list of (valid) node ids present in the graph
    pub fn ids(self: &Graph) -> Vec<NodeId> {
        unimplemented!("ids");
    }

    /// Returns the list of (out) neighbors of a node
    pub fn neighbors(self: &Graph, id: NodeId) -> Vec<NodeId> {
        unimplemented!("neighbors");
    }
}

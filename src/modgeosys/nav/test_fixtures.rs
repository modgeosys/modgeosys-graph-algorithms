#[cfg(test)]
pub mod tests
{
    use std::collections::HashSet;
    use crate::modgeosys::nav::types::{Edge, Graph, Node};

    pub fn valid_nodes() -> Vec<Node>
    {
        let nodes = vec![Node::new(0.0, 0.0), Node::new(0.0, 2.0), Node::new(1.0, 0.0), Node::new(2.0, 1.0), Node::new(2.0, 3.0)];
        nodes
    }

    pub fn valid_edges1() -> Vec<Edge>
    {
        let edges = vec![Edge::new(2.0, HashSet::from([0, 1])),
                         Edge::new(1.0, HashSet::from([0, 2])),
                         Edge::new(1.0, HashSet::from([2, 3])),
                         Edge::new(3.0, HashSet::from([1, 4])),
                         Edge::new(1.0, HashSet::from([3, 4]))];
        edges
    }

    pub fn valid_edges2() -> Vec<Edge>
    {
        let edges = vec![Edge::new(3.0, HashSet::from([0, 1])),
                         Edge::new(1.0, HashSet::from([0, 2])),
                         Edge::new(1.0, HashSet::from([2, 3])),
                         Edge::new(3.0, HashSet::from([1, 4])),
                         Edge::new(1.0, HashSet::from([3, 4]))];
        edges
    }

    pub fn valid_graph1() -> Graph
    {
        let graph = Graph::new(valid_nodes(), valid_edges1());
        graph
    }

    pub fn valid_graph2() -> Graph
    {
        let graph = Graph::new(valid_nodes(), valid_edges2());
        graph
    }
}

#[cfg(test)]
pub mod tests
{
    use std::collections::{HashSet, BTreeMap};
    use crate::distance::manhattan_distance;
    use crate::types::{Edge, EdgeDefinition, Graph, Node, specified_edge_weight};

    pub fn valid_nodes() -> Vec<Node>
    {
        let nodes = vec![Node::new(vec![0.0, 0.0], BTreeMap::new()),
                         Node::new(vec![0.0, 2.0], BTreeMap::new()),
                         Node::new(vec![1.0, 0.0], BTreeMap::new()),
                         Node::new(vec![2.0, 1.0], BTreeMap::new()),
                         Node::new(vec![2.0, 3.0], BTreeMap::new())];
        nodes
    }

    pub fn valid_edges1() -> Vec<Edge>
    {
        let edges = vec![Edge::new(HashSet::from([0, 1]), 2.0, BTreeMap::new()),
                         Edge::new(HashSet::from([0, 2]), 1.0, BTreeMap::new()),
                         Edge::new(HashSet::from([2, 3]), 1.0, BTreeMap::new()),
                         Edge::new(HashSet::from([1, 4]), 3.0, BTreeMap::new()),
                         Edge::new(HashSet::from([3, 4]), 1.0, BTreeMap::new())];
        edges
    }

    pub fn valid_edges2() -> Vec<Edge>
    {
        let edges = vec![Edge::new(HashSet::from([0, 1]), 3.0, BTreeMap::new()),
                         Edge::new(HashSet::from([0, 2]), 1.0, BTreeMap::new()),
                         Edge::new(HashSet::from([2, 3]), 1.0, BTreeMap::new()),
                         Edge::new(HashSet::from([1, 4]), 3.0, BTreeMap::new()),
                         Edge::new(HashSet::from([3, 4]), 1.0, BTreeMap::new())];
        edges
    }

    pub fn valid_graph1() -> Graph
    {
        let graph = Graph::new(valid_nodes(), valid_edges1(), BTreeMap::new(), specified_edge_weight, manhattan_distance);
        graph
    }

    pub fn valid_graph2() -> Graph
    {
        let graph = Graph::new(valid_nodes(), valid_edges2(), BTreeMap::new(), specified_edge_weight, manhattan_distance);
        graph
    }

    pub fn valid_graph_from_edge_definitions() -> Graph
    {
        let edge_definitions = vec![EdgeDefinition(vec![vec![0.0, 0.0], vec![0.0, 2.0]], 2.0, BTreeMap::new()),
                                    EdgeDefinition(vec![vec![0.0, 0.0], vec![1.0, 0.0]], 1.0, BTreeMap::new()),
                                    EdgeDefinition(vec![vec![1.0, 0.0], vec![2.0, 1.0]], 1.0, BTreeMap::new()),
                                    EdgeDefinition(vec![vec![0.0, 2.0], vec![2.0, 3.0]], 3.0, BTreeMap::new()),
                                    EdgeDefinition(vec![vec![2.0, 1.0], vec![2.0, 3.0]], 1.0, BTreeMap::new())];
        let graph = Graph::from_edge_definitions(edge_definitions, BTreeMap::new(), specified_edge_weight, manhattan_distance);

        graph
    }
}

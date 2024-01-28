use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

use ndarray::Array2;
use ordered_float::OrderedFloat;


#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Node
{
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
}

impl Node
{
    pub fn new(x: f64, y: f64) -> Self
    {
        Node
        {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Edge
{
    pub weight: OrderedFloat<f64>,
    pub node_indices: HashSet<usize>,
}

impl Edge
{
    pub fn new(weight: f64, node_indices: HashSet<usize>) -> Self
    {
        Edge
        {
            weight: OrderedFloat(weight),
            node_indices,
        }
    }

    pub fn index_of_other_node(&self, current_index: usize) -> usize
    {
        let node_indices: Vec<usize> = self.node_indices.iter().cloned().collect();
        if node_indices[0] == current_index
        {
            node_indices[1]
        }
        else
        {
            node_indices[0]
        }
    }
}

impl PartialEq for Edge
{
    fn eq(&self, other: &Self) -> bool
    {
        self.weight == other.weight && self.node_indices == other.node_indices
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for Edge
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}



#[derive(Debug, Clone)]
pub struct EdgeTransit
{
    pub edge: Edge,
    pub g: OrderedFloat<f64>,
    pub h: OrderedFloat<f64>,
}

impl EdgeTransit
{
    pub fn new(edge: Edge, g: f64, h: f64) -> Self
    {
        EdgeTransit
        {
            edge: edge,
            g: OrderedFloat(g),
            h: OrderedFloat(h),
        }
    }

    pub fn f(&self) -> OrderedFloat<f64>
    {
        self.g + self.h
    }
}

impl PartialEq for EdgeTransit
{
    fn eq(&self, other: &Self) -> bool
    {
        self.edge == other.edge && self.g == other.g && self.h == other.h
    }
}

impl Eq for EdgeTransit {}

impl PartialOrd for EdgeTransit
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        self.edge.partial_cmp(&other.edge)
            .and_then(|ordering| match ordering
            {
                Ordering::Equal => self.g.partial_cmp(&other.g)
                    .and_then(|ordering| match ordering
                    {
                        Ordering::Equal => self.h.partial_cmp(&other.h),
                        _ => Some(ordering),
                    }),
                _ => Some(ordering),
            })
    }
}

impl Ord for EdgeTransit
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}


#[derive(Debug, Clone)]
pub struct Graph
{
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph
{
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self
    {
        Graph { nodes, edges }
    }

    pub fn adjacency_map(&self) -> HashMap<Node, Vec<Edge>>
    {
        let mut adjacency_map: HashMap<Node, Vec<Edge>> = HashMap::new();

        for node in &self.nodes
        {
            adjacency_map.insert(node.clone(), vec![]);
        }

        for edge in &self.edges
        {
            for node_index in &edge.node_indices
            {
                adjacency_map.get_mut(&self.nodes[*node_index]).unwrap().push(edge.clone());
            }
        }

        for node in &self.nodes
        {
            adjacency_map.get_mut(node).unwrap().sort();
        }

        adjacency_map
    }

    pub fn adjacency_matrix(&self) -> Array2<f64>
    {
        let mut adjacency_matrix = Array2::from_elem((self.nodes.len(), self.nodes.len()), f64::INFINITY);

        for edge in &self.edges
        {
            let node_indices: Vec<usize> = edge.node_indices.iter().cloned().collect();
            adjacency_matrix[[node_indices[0], node_indices[1]]] = f64::from(edge.weight);
            adjacency_matrix[[node_indices[1], node_indices[0]]] = f64::from(edge.weight);
        }

        adjacency_matrix
    }
}

#[derive(Debug, Clone)]
pub struct NoNavigablePathError
{
    pub start_node: Node,
    pub goal_node: Node,
}

impl NoNavigablePathError
{
    pub fn new(start_node: Node, goal_node: Node) -> Self
    {
        NoNavigablePathError
        {
            start_node,
            goal_node,
        }
    }
}


#[cfg(test)]
mod tests
{
    use super::*;
    use crate::test_fixtures::tests::{valid_nodes, valid_edges1, valid_graph1};

    #[test]
    fn test_edge_creation_()
    {
        let edge = Edge::new(10.0, HashSet::from([1, 2]));
        assert_eq!(edge.weight, 10.0);
        assert_eq!(edge.node_indices, HashSet::from([1, 2]));
    }

    #[test]
    fn test_edge_index_of_other_node()
    {
        let edge = Edge::new(10.0, HashSet::from([1, 2]));
        assert_eq!(edge.index_of_other_node(1), 2);
        assert_eq!(edge.index_of_other_node(2), 1);
    }

    #[test]
    fn test_edge_equality()
    {
        let edge_1 = Edge::new(10.0, HashSet::from([1, 2]));
        let edge_2 = Edge::new(10.0, HashSet::from([1, 2]));
        assert_eq!(edge_1, edge_2);
    }

    #[test]
    fn test_edge_inequality()
    {
        let edge_1 = Edge::new(10.0, HashSet::from([1, 2]));
        let edge_2 = Edge::new(10.0, HashSet::from([1, 3]));
        assert_ne!(edge_1, edge_2);
    }

    #[test]
    fn test_edge_transit_creation()
    {
        let edge_transit = EdgeTransit::new(Edge::new(10.0, HashSet::from([1, 2])), 5.0, 5.0);
        assert_eq!(edge_transit.edge, Edge::new(10.0, HashSet::from([1, 2])));
        assert_eq!(edge_transit.g, OrderedFloat(5.0f64));
        assert_eq!(edge_transit.h, OrderedFloat(5.0f64));
    }

    #[test]
    fn test_edge_transit_f_calculation()
    {
        let edge_transit = EdgeTransit::new(Edge::new(10.0, HashSet::from([1, 2])), 5.0, 5.0);
        assert_eq!(edge_transit.f(), OrderedFloat(10.0f64));
    }

    #[test]
    fn test_edge_transit_equality()
    {
        let edge_transit1 = EdgeTransit::new(Edge::new(10.0, HashSet::from([1, 2])), 5.0, 5.0);
        let edge_transit2 = EdgeTransit::new(Edge::new(10.0, HashSet::from([1, 2])), 5.0, 5.0);
        assert_eq!(edge_transit1, edge_transit2);
    }

    #[test]
    fn test_graph_creation()
    {
        assert_eq!(valid_graph1().nodes, valid_nodes());
        assert_eq!(valid_graph1().edges, valid_edges1());
    }

    #[test]
    fn test_graph_adjacency_map()
    {
        let nodes = valid_nodes();
        let edges = valid_edges1();
        let graph = Graph::new(nodes, edges);

        let adjacency_map = graph.adjacency_map();

        assert_eq!(adjacency_map.len(), 5);
        assert_eq!(adjacency_map.keys().collect::<Vec<&Node>>().sort(), vec![&Node::new(0.0, 1.0), &Node::new(0.0, 2.0), &Node::new(2.0, 3.0), &Node::new(1.0, 4.0), &Node::new(3.0, 4.0)].sort());

        assert_eq!(adjacency_map[&graph.nodes[0]], vec![Edge::new(1.0, HashSet::from([0, 2])), Edge::new(2.0, HashSet::from([0, 1]))]);
        assert_eq!(adjacency_map[&graph.nodes[1]], vec![Edge::new(2.0, HashSet::from([0, 1])), Edge::new(3.0, HashSet::from([1, 4]))]);
        assert_eq!(adjacency_map[&graph.nodes[2]], vec![Edge::new(1.0, HashSet::from([0, 2])), Edge::new(1.0, HashSet::from([2, 3]))]);
        assert_eq!(adjacency_map[&graph.nodes[3]], vec![Edge::new(1.0, HashSet::from([2, 3])), Edge::new(1.0, HashSet::from([3, 4]))]);
        assert_eq!(adjacency_map[&graph.nodes[4]], vec![Edge::new(1.0, HashSet::from([3, 4])), Edge::new(3.0, HashSet::from([1, 4]))]);
    }

    #[test]
    fn test_graph_adjacency_matrix()
    {
        let nodes = valid_nodes();
        let edges = valid_edges1();
        let graph = Graph::new(nodes, edges);

        let adjacency_matrix = graph.adjacency_matrix();

        assert_eq!(adjacency_matrix.shape(), [5, 5]);

        assert_eq!(adjacency_matrix, ndarray::arr2(&[[f64::INFINITY,           2.0,           1.0, f64::INFINITY, f64::INFINITY],
                                                     [          2.0, f64::INFINITY, f64::INFINITY, f64::INFINITY,           3.0],
                                                     [          1.0, f64::INFINITY, f64::INFINITY,           1.0, f64::INFINITY],
                                                     [f64::INFINITY, f64::INFINITY,           1.0, f64::INFINITY,           1.0],
                                                     [f64::INFINITY,           3.0, f64::INFINITY,           1.0, f64::INFINITY]]));

    }
}
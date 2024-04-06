// Simple and complex data types for the graph module.

use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

use ndarray::{Array1, Array2, Ix1};
use ndarray::iter::Iter;
use ordered_float::OrderedFloat;



// A node in a graph.
#[derive(Debug, Clone, Eq, Hash)]
pub struct Node(pub Array1<OrderedFloat<f64>>);

impl Node
{
    pub fn new(coordinates: Vec<f64>) -> Self
    {
        Node(Array1::from_vec(coordinates).mapv(OrderedFloat))
    }
}

impl PartialOrd for Node
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        for (self_coord, other_coord) in self.0.iter().zip(other.0.iter())
        {
            match self_coord.cmp(other_coord)
            {
                Ordering::Equal => continue,
                non_equal => return Some(non_equal),
            }
        }
        Some(Ordering::Equal)
    }
}

impl PartialEq for Node
{
    fn eq(&self, other: &Self) -> bool
    {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
    }
}

impl Ord for Node
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl<'a> IntoIterator for &'a Node
{
    type Item = &'a OrderedFloat<f64>;
    type IntoIter = Iter<'a, OrderedFloat<f64>, Ix1>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.0.iter()
    }
}


#[derive(Debug, Clone)]
pub struct EdgeDefinition(pub f64, pub Vec<Vec<f64>>);


// An edge in a graph.
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

    // Given one node index, return the other node index.
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


// A graph.
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

    pub fn from_edge_definitions(edge_definitions: Vec<EdgeDefinition>) -> Self
    {
        let mut coordinates_of_all_nodes: Vec<Vec<f64>> = vec![];

        for edge_definition in &edge_definitions
        {
            for edge_node_coordinates in &edge_definition.1
            {
                if !coordinates_of_all_nodes.contains(edge_node_coordinates)
                {
                    coordinates_of_all_nodes.push(edge_node_coordinates.clone());
                }
            }
        }

        let mut node_map: HashMap<usize, Node> = HashMap::new();
        let mut edges: Vec<Edge> = vec![];

        for edge_definition in &edge_definitions
        {
            let mut indices: Vec<usize> = vec![];
            for edge_node_coordinates in &edge_definition.1
            {
                let index = coordinates_of_all_nodes.iter().position(|coordinates| coordinates == edge_node_coordinates).unwrap();
                indices.push(index);
                node_map.insert(index, Node::new(edge_node_coordinates.clone()));
            }
            let node_indices: HashSet<_> = indices.into_iter().collect();
            let edge = Edge::new(edge_definition.0, node_indices);
            edges.push(edge);
        }

        let mut node_vec: Vec<(usize, Node)> = node_map.into_iter().collect();
        node_vec.sort_by_key(|(key, _)| *key);
        let nodes: Vec<Node> = node_vec.into_iter().map(|(_, node)| node).collect();

        Graph::new(nodes, edges)
    }

    // Render an adjacency map.
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

    // Render an adjacency matrix.
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


// Returned when no path can be found to the goal node.
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
    use crate::test_fixtures::tests::{valid_nodes, valid_edges1, valid_graph1, valid_graph_from_edge_definitions};

    #[test]
    fn test_node_equality()
    {
        let node1 = Node::new(vec![0.0, 0.0]);
        let node2 = Node::new(vec![0.0, 0.0]);
        assert_eq!(node1, node2);
    }

    #[test]
    fn test_node_inequality()
    {
        let node1 = Node::new(vec![0.0, 0.0]);
        let node2 = Node::new(vec![0.0, 1.0]);
        assert_ne!(node1, node2);
    }

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
    fn test_graph_creation()
    {
        assert_eq!(valid_graph1().nodes, valid_nodes());
        assert_eq!(valid_graph1().edges, valid_edges1());
    }

    #[test]
    fn test_graph_from_edge_definitions()
    {
        let graph = valid_graph_from_edge_definitions();

        assert_eq!(graph.nodes, valid_nodes());
        assert_eq!(graph.edges, valid_edges1());
    }

    #[test]
    fn test_graph_adjacency_map()
    {
        let nodes = valid_nodes();
        let edges = valid_edges1();
        let graph = Graph::new(nodes, edges);

        let adjacency_map = graph.adjacency_map();

        assert_eq!(adjacency_map.len(), 5);
        assert_eq!(adjacency_map.keys().collect::<Vec<&Node>>().sort(), vec![&Node::new(vec![0.0, 1.0]), &Node::new(vec![0.0, 2.0]), &Node::new(vec![2.0, 3.0]), &Node::new(vec![1.0, 4.0]), &Node::new(vec![3.0, 4.0])].sort());

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
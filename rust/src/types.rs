// Simple and complex data types for the graph module.

use std::collections::{BTreeMap, HashMap};
use std::collections::HashSet;
use std::cmp::Ordering;

use ndarray::{Array1, Array2, Ix1};
use ndarray::iter::Iter;
use ordered_float::OrderedFloat;



// A property value.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PropertyValue
{
    String(String),
    Integer(i64),
    Float(OrderedFloat<f64>),
    Boolean(bool),
}


// A node in a graph.
#[derive(Debug, Clone, Eq, Hash)]
pub struct Node
{
    pub coordinates: Array1<OrderedFloat<f64>>,
    pub properties: BTreeMap<String, PropertyValue>,
}

impl Node
{
    pub fn new(coordinates: Vec<f64>, properties: BTreeMap<String, PropertyValue>) -> Self
    {
        Node
        {
            coordinates: Array1::from_vec(coordinates).mapv(OrderedFloat),
            properties,
        }
    }
}

impl PartialOrd for Node
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        for (self_coord, other_coord) in self.coordinates.iter().zip(other.coordinates.iter())
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
        self.coordinates.iter().zip(other.coordinates.iter()).all(|(a, b)| a == b)
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
        self.coordinates.iter()
    }
}


#[derive(Debug, Clone)]
pub struct EdgeDefinition(pub Vec<Vec<f64>>, pub f64, pub BTreeMap<String, PropertyValue>);


// An edge in a graph.
#[derive(Debug, Clone)]
pub struct Edge
{
    pub node_indices: HashSet<usize>,
    pub weight: OrderedFloat<f64>,
    pub properties: BTreeMap<String, PropertyValue>,
}

impl Edge
{
    pub fn new(node_indices: HashSet<usize>, weight: f64, properties: BTreeMap<String, PropertyValue>) -> Self
    {
        Edge
        {
            node_indices,
            weight: OrderedFloat(weight),
            properties,
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
        self.weight == other.weight && self.node_indices == other.node_indices && self.properties == other.properties
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
    pub properties: BTreeMap<String, PropertyValue>,
    pub distance_function: fn(&Node, &Node) -> OrderedFloat<f64>,
    pub edge_weight_function: fn(&Graph, &Edge) -> OrderedFloat<f64>,
}

impl Graph
{
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>, properties: BTreeMap<String, PropertyValue>, distance_function: fn(&Node, &Node) -> OrderedFloat<f64>, edge_weight_function: Option<fn(&Graph, &Edge) -> OrderedFloat<f64>>) -> Self
    {
        let edge_weight_function = edge_weight_function.unwrap_or(specified_edge_weight);
        let mut graph = Graph { nodes, edges, properties, edge_weight_function, distance_function };

        // Compute edge weights.
        let new_weights: Vec<OrderedFloat<f64>> = graph.edges.iter().map(|edge| (graph.edge_weight_function)(&graph, edge)).collect();

        // Assign new weights to edges.
        for (edge, &new_weight) in graph.edges.iter_mut().zip(new_weights.iter())
        {
            edge.weight = new_weight;
        }

        graph
    }

    pub fn from_edge_definitions(edge_definitions: Vec<EdgeDefinition>, properties: BTreeMap<String, PropertyValue>, distance_function: fn(&Node, &Node) -> OrderedFloat<f64>, edge_weight_function: Option<fn(&Graph, &Edge) -> OrderedFloat<f64>>) -> Self
    {
        let mut coordinates_of_all_nodes: Vec<Vec<f64>> = vec![];

        for edge_definition in &edge_definitions
        {
            for edge_node_coordinates in &edge_definition.0
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
            for edge_node_coordinates in &edge_definition.0
            {
                let index = coordinates_of_all_nodes.iter().position(|coordinates| coordinates == edge_node_coordinates).unwrap();
                indices.push(index);
                node_map.insert(index, Node::new(edge_node_coordinates.clone(), BTreeMap::new()));
            }
            let node_indices: HashSet<_> = indices.into_iter().collect();
            let edge = Edge::new(node_indices, edge_definition.1, edge_definition.2.clone());
            edges.push(edge);
        }

        let mut node_vec: Vec<(usize, Node)> = node_map.into_iter().collect();
        node_vec.sort_by_key(|(key, _)| *key);
        let nodes: Vec<Node> = node_vec.into_iter().map(|(_, node)| node).collect();

        Graph::new(nodes, edges, properties, distance_function, edge_weight_function)
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

pub fn specified_edge_weight(_graph: &Graph, edge: &Edge) -> OrderedFloat<f64>
{
    edge.weight
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
    // use crate::distance::manhattan_distance;
    use crate::test_fixtures::tests::{valid_nodes, valid_edges1, valid_edges1_with_computed_weights, valid_graph1, valid_graph_from_edge_definitions, valid_graph3, valid_edges3_with_computed_weights};

    #[test]
    fn test_node_equality()
    {
        let node1 = Node::new(vec![0.0, 0.0], BTreeMap::new());
        let node2 = Node::new(vec![0.0, 0.0], BTreeMap::new());
        assert_eq!(node1, node2);
    }

    #[test]
    fn test_node_inequality()
    {
        let node1 = Node::new(vec![0.0, 0.0], BTreeMap::new());
        let node2 = Node::new(vec![0.0, 1.0], BTreeMap::new());
        assert_ne!(node1, node2);
    }

    #[test]
    fn test_edge_creation_()
    {
        let edge = Edge::new(HashSet::from([1, 2]), 10.0, BTreeMap::new());
        assert_eq!(edge.node_indices, HashSet::from([1, 2]));
        assert_eq!(edge.weight, 10.0);
    }

    #[test]
    fn test_edge_index_of_other_node()
    {
        let edge = Edge::new(HashSet::from([1, 2]), 10.0, BTreeMap::new());
        assert_eq!(edge.index_of_other_node(1), 2);
        assert_eq!(edge.index_of_other_node(2), 1);
    }

    #[test]
    fn test_edge_equality()
    {
        let edge_1 = Edge::new(HashSet::from([1, 2]), 10.0, BTreeMap::new());
        let edge_2 = Edge::new(HashSet::from([1, 2]), 10.0, BTreeMap::new());
        assert_eq!(edge_1, edge_2);
    }

    #[test]
    fn test_edge_inequality()
    {
        let edge_1 = Edge::new(HashSet::from([1, 2]), 10.0, BTreeMap::new());
        let edge_2 = Edge::new(HashSet::from([1, 3]), 10.0, BTreeMap::new());
        assert_ne!(edge_1, edge_2);
    }

    #[test]
    fn test_graph_creation()
    {
        assert_eq!(valid_graph1().nodes, valid_nodes());
        assert_eq!(valid_graph1().edges, valid_edges1());
        assert_eq!(valid_graph1().properties, BTreeMap::new());
        // assert_eq!(valid_graph1().heuristic_distance_function, manhattan_distance);
        // assert_eq!(valid_graph1().edge_weight_function, specified_edge_weight);
    }

    #[test]
    fn test_graph_creation_with_edge_weight_function()
    {
        let graph = valid_graph3();

        assert_eq!(graph.nodes, valid_nodes());
        assert_eq!(graph.edges, valid_edges3_with_computed_weights());
        assert_eq!(graph.properties, BTreeMap::new());
        // assert_eq!(graph.heuristic_distance_function, manhattan_distance);
        // assert_eq!(graph.edge_weight_function, length_cost_per_unit);
    }

    #[test]
    fn test_graph_creation_from_edge_definitions()
    {
        let graph = valid_graph_from_edge_definitions();

        assert_eq!(graph.nodes, valid_nodes());
        assert_eq!(graph.edges, valid_edges1_with_computed_weights());
        assert_eq!(graph.properties, BTreeMap::new());
        // assert_eq!(graph.heuristic_distance_function, manhattan_distance);
        // assert_eq!(graph.edge_weight_function, specified_edge_weight);
    }

    #[test]
    fn test_graph_adjacency_map()
    {
        let graph = valid_graph1();

        let adjacency_map = graph.adjacency_map();

        assert_eq!(adjacency_map.len(), 5);
        assert_eq!(adjacency_map.keys().collect::<Vec<&Node>>().sort(), vec![&Node::new(vec![0.0, 1.0], BTreeMap::new()), &Node::new(vec![0.0, 2.0], BTreeMap::new()), &Node::new(vec![2.0, 3.0], BTreeMap::new()), &Node::new(vec![1.0, 4.0], BTreeMap::new()), &Node::new(vec![3.0, 4.0], BTreeMap::new())].sort());

        assert_eq!(adjacency_map[&graph.nodes[0]], vec![Edge::new(HashSet::from([0, 2]), 1.0, BTreeMap::new()), Edge::new(HashSet::from([0, 1]), 2.0, BTreeMap::new())]);
        assert_eq!(adjacency_map[&graph.nodes[1]], vec![Edge::new(HashSet::from([0, 1]), 2.0, BTreeMap::new()), Edge::new(HashSet::from([1, 4]), 3.0, BTreeMap::new())]);
        assert_eq!(adjacency_map[&graph.nodes[2]], vec![Edge::new(HashSet::from([0, 2]), 1.0, BTreeMap::new()), Edge::new(HashSet::from([2, 3]), 1.0, BTreeMap::new())]);
        assert_eq!(adjacency_map[&graph.nodes[3]], vec![Edge::new(HashSet::from([2, 3]), 1.0, BTreeMap::new()), Edge::new(HashSet::from([3, 4]), 1.0, BTreeMap::new())]);
        assert_eq!(adjacency_map[&graph.nodes[4]], vec![Edge::new(HashSet::from([3, 4]), 1.0, BTreeMap::new()), Edge::new(HashSet::from([1, 4]), 3.0, BTreeMap::new())]);
    }

    #[test]
    fn test_graph_adjacency_matrix()
    {
        let graph = valid_graph1();

        let adjacency_matrix = graph.adjacency_matrix();

        assert_eq!(adjacency_matrix.shape(), [5, 5]);

        assert_eq!(adjacency_matrix, ndarray::arr2(&[[f64::INFINITY,           2.0,           1.0, f64::INFINITY, f64::INFINITY],
                                                     [          2.0, f64::INFINITY, f64::INFINITY, f64::INFINITY,           3.0],
                                                     [          1.0, f64::INFINITY, f64::INFINITY,           1.0, f64::INFINITY],
                                                     [f64::INFINITY, f64::INFINITY,           1.0, f64::INFINITY,           1.0],
                                                     [f64::INFINITY,           3.0, f64::INFINITY,           1.0, f64::INFINITY]]));

    }
}
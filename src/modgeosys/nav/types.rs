use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

use ndarray::Array2;
use ordered_float::OrderedFloat;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
//
// impl PartialEq for Node
// {
//     fn eq(&self, other: &Self) -> bool
//     {
//         self.0 == other.0 && self.1 == other.1
//     }
// }
//
// impl Eq for Node {}
//
// impl Hash for Node
// {
//     fn hash<H: Hasher>(&self, state: &mut H)
//     {
//         self.0.to_bits().hash(state);
//         self.1.to_bits().hash(state);
//     }
// }

#[derive(Debug, Clone)]
pub struct Edge
{
    pub weight: OrderedFloat<f64>,
    pub node_indices: HashSet<usize>,
    pub g: Option<OrderedFloat<f64>>,
    pub h: Option<OrderedFloat<f64>>,
}

impl Edge
{
    pub fn new(weight: f64, node_indices: HashSet<usize>, g: Option<f64>, h: Option<f64>) -> Self
    {
        Edge
        {
            weight: OrderedFloat(weight),
            node_indices,
            g: g.map(OrderedFloat),
            h: h.map(OrderedFloat),
        }
    }

    pub fn f(&self) -> Option<OrderedFloat<f64>>
    {
        match (self.g, self.h)
        {
            (Some(g), Some(h)) => Some(g + h),
            _ => None,
        }
    }

    pub fn coordinates_of_other(&self, current_index: usize) -> usize
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
        self.weight == other.weight && self.g == other.g && self.h == other.h
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        let self_g = self.g.unwrap_or(OrderedFloat(0f64));
        let other_g = other.g.unwrap_or(OrderedFloat(0f64));

        let self_h = self.h.unwrap_or(OrderedFloat(0f64));
        let other_h = other.h.unwrap_or(OrderedFloat(0f64));

        match self.weight.partial_cmp(&other.weight)
        {
            Some(Ordering::Equal) => match self_g.partial_cmp(&other_g)
            {
                Some(Ordering::Equal) => self_h.partial_cmp(&other_h),
                other => other,
            },
            other => other,
        }
    }
}

impl Ord for Edge
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
//
// impl Hash for Edge
// {
//     fn hash<H: Hasher>(&self, state: &mut H)
//     {
//         self.weight.to_bits().hash(state);
//         match self.g
//         {
//             Some(g) => g.to_bits().hash(state),
//             None => f64::INFINITY.to_bits().hash(state),
//         }
//         match self.h
//         {
//             Some(h) => h.to_bits().hash(state),
//             None => f64::INFINITY.to_bits().hash(state),
//         }
//     }
// }

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

    #[test]
    fn test_edge_creation_with_valid_parameters()
    {
        let edge = Edge::new(10.0, HashSet::from([1, 2]), Some(5.0), Some(5.0));
        assert_eq!(edge.weight, 10.0);
        assert_eq!(edge.node_indices, HashSet::from([1, 2]));
        assert_eq!(edge.g, Some(OrderedFloat(5.0f64)));
        assert_eq!(edge.h, Some(OrderedFloat(5.0f64)));
    }

    #[test]
    fn test_edge_coordinates_of_other()
    {
        let edge = Edge::new(10.0, HashSet::from([1, 2]), None, None);
        assert_eq!(edge.coordinates_of_other(1), 2);
        assert_eq!(edge.coordinates_of_other(2), 1);
    }

    #[test]
    fn test_edge_f_calculation()
    {
        let edge = Edge::new(10.0, HashSet::from([1, 2]), Some(5.0), Some(5.0));
        assert_eq!(edge.f(), Some(OrderedFloat(10.0f64)));
    }

    #[test]
    fn test_edge_f_with_none_values()
    {
        let edge = Edge::new(10.0, HashSet::from([1, 2]), None, None);
        assert_eq!(edge.f(), None);
    }

    #[test]
    fn test_edge_equality()
    {
        let edge_1 = Edge::new(10.0, HashSet::from([1, 2]), Some(5.0), Some(5.0));
        let edge_2 = Edge::new(10.0, HashSet::from([1, 2]), Some(5.0), Some(5.0));
        assert_eq!(edge_1, edge_2);
    }

    #[test]
    fn test_graph_adjacency_map()
    {
        let nodes = vec![Node::new(0.0, 1.0), Node::new(0.0, 2.0), Node::new(2.0, 3.0), Node::new(1.0, 4.0), Node::new(3.0, 4.0)];
        let edges = vec![Edge::new(2.0, HashSet::from([0, 1]), None, None),
                         Edge::new(1.0, HashSet::from([0, 2]), None, None),
                         Edge::new(1.0, HashSet::from([2, 3]), None, None),
                         Edge::new(3.0, HashSet::from([1, 4]), None, None),
                         Edge::new(1.0, HashSet::from([3, 4]), None, None)];
        let graph = Graph::new(nodes, edges);

        let adjacency_map = graph.adjacency_map();

        assert_eq!(adjacency_map.len(), 5);

        assert_eq!(adjacency_map[&Node::new(0.0, 0.0)], vec![Edge::new(1.0, HashSet::from([0, 2]), None, None), Edge::new(2.0, HashSet::from([0, 1]), None, None)]);
        assert_eq!(adjacency_map[&Node::new(0.0, 2.0)], vec![Edge::new(2.0, HashSet::from([0, 1]), None, None), Edge::new(3.0, HashSet::from([1, 4]), None, None)]);
        assert_eq!(adjacency_map[&Node::new(1.0, 0.0)], vec![Edge::new(1.0, HashSet::from([0, 2]), None, None), Edge::new(1.0, HashSet::from([2, 3]), None, None)]);
        assert_eq!(adjacency_map[&Node::new(2.0, 1.0)], vec![Edge::new(1.0, HashSet::from([2, 3]), None, None), Edge::new(1.0, HashSet::from([3, 4]), None, None)]);
        assert_eq!(adjacency_map[&Node::new(2.0, 3.0)], vec![Edge::new(1.0, HashSet::from([3, 4]), None, None), Edge::new(3.0, HashSet::from([1, 4]), None, None)]);
    }

    #[test]
    fn test_graph_adjacency_matrix()
    {
        let nodes = vec![Node::new(0.0, 1.0), Node::new(0.0, 2.0), Node::new(2.0, 3.0), Node::new(1.0, 4.0), Node::new(3.0, 4.0)];
        let edges = vec![Edge::new(2.0, HashSet::from([0, 1]), None, None),
                         Edge::new(1.0, HashSet::from([0, 2]), None, None),
                         Edge::new(1.0, HashSet::from([2, 3]), None, None),
                         Edge::new(3.0, HashSet::from([1, 4]), None, None),
                         Edge::new(1.0, HashSet::from([3, 4]), None, None)];
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
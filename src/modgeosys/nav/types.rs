use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

use ndarray::Array2;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node(f64, f64);

#[derive(Debug, Clone, Hash)]
pub struct Edge
{
    pub weight: f64,
    pub node_indices: HashSet<usize>,
    pub g: Option<f64>,
    pub h: Option<f64>,
}

impl Edge
{
    pub fn new(weight: f64, node_indices: HashSet<usize>, g: Option<f64>, h: Option<f64>) -> Self
    {
        Edge
        {
            weight,
            node_indices,
            g,
            h,
        }
    }

    pub fn f(&self) -> Option<f64>
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
        Some(self.cmp(other))
    }
}

impl Ord for Edge
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        match self.weight.partial_cmp(&other.weight)
        {
            Some(Ordering::Equal) => match self.g.partial_cmp(&other.g)
            {
                Some(Ordering::Equal) => self.h.partial_cmp(&other.h).unwrap_or(Ordering::Equal),
                other => other.unwrap_or(Ordering::Equal),
            },
            other => other.unwrap_or(Ordering::Equal),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph
{
    nodes: Vec<Node>,
    edges: Vec<Edge>,
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
                // TODO: Fully implement an insertion sort.
                // NOTE: Rust doesn't have a direct equivalent for Python's bisect.insort function and Python's dynamic typing.
                //       Therefore, we just append the edge to the node's list without sorting it.
                adjacency_map.get_mut(&self.nodes[*node_index]).unwrap().push(edge.clone());
            }
        }

        adjacency_map
    }

    pub fn adjacency_matrix(&self) -> Array2<f64>
    {
        let mut adjacency_matrix = Array2::from_elem((self.nodes.len(), self.nodes.len()), f64::INFINITY);

        for edge in &self.edges
        {
            let node_indices: Vec<usize> = edge.node_indices.iter().cloned().collect();
            adjacency_matrix[[node_indices[0], node_indices[1]]] = edge.weight;
            adjacency_matrix[[node_indices[1], node_indices[0]]] = edge.weight;
        }

        adjacency_matrix
    }
}

#[derive(Debug, Clone)]
pub struct NoNavigablePathError
{
    start_node: Node,
    goal_node: Node,
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
        let edge = Edge::new(10.0, [1, 2].iter().cloned().collect(), Some(5.0), Some(5.0));
        assert_eq!(edge.weight, 10.0);
        assert_eq!(edge.node_indices, [1, 2].iter().cloned().collect());
        assert_eq!(edge.g, Some(5.0));
        assert_eq!(edge.h, Some(5.0));
    }

    #[test]
    fn test_edge_coordinates_of_other()
    {
        let edge = Edge::new(10.0, [1, 2].iter().cloned().collect(), None, None);
        assert_eq!(edge.coordinates_of_other(1), 2);
        assert_eq!(edge.coordinates_of_other(2), 1);
    }

    #[test]
    fn test_edge_f_calculation()
    {
        let edge = Edge::new(10.0, [1, 2].iter().cloned().collect(), Some(5.0), Some(5.0));
        assert_eq!(edge.f(), Some(10.0));
    }

    #[test]
    fn test_edge_f_with_none_values()
    {
        let edge = Edge::new(10.0, [1, 2].iter().cloned().collect(), None, None);
        assert_eq!(edge.f(), None);
    }

    #[test]
    fn test_edge_equality()
    {
        let edge_1 = Edge::new(10.0, [1, 2].iter().cloned().collect(), Some(5.0), Some(5.0));
        let edge_2 = Edge::new(10.0, [1, 2].iter().cloned().collect(), Some(5.0), Some(5.0));
        assert_eq!(edge_1, edge_2);
    }

    #[test]
    fn test_graph_adjacency_map()
    {
        let nodes = vec![Node(0.0, 1.0), Node(0.0, 2.0), Node(2.0, 3.0), Node(1.0, 4.0), Node(3.0, 4.0)];
        let edges = vec![Edge::new(2.0, [0, 1].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [0, 2].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [2, 3].iter().cloned().collect(), None, None),
                         Edge::new(3.0, [1, 4].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [3, 4].iter().cloned().collect(), None, None)];
        let graph = Graph::new(nodes, edges);

        let adjacency_map = graph.adjacency_map();

        assert_eq(len(adjacency_map), 5);

        assert_eq!(adjacency_map[&Node(0.0, 0.0)], vec![Edge::new(1.0, [0, 2].iter().cloned().collect(), None, None), Edge::new(2.0, [0, 1].iter().cloned().collect(), None, None)]);
        assert_eq!(adjacency_map[&Node(0.0, 2.0)], vec![Edge::new(2.0, [0, 1].iter().cloned().collect(), None, None), Edge::new(3.0, [1, 4].iter().cloned().collect(), None, None)]);
        assert_eq!(adjacency_map[&Node(1.0, 0.0)], vec![Edge::new(1.0, [0, 2].iter().cloned().collect(), None, None), Edge::new(1.0, [2, 3].iter().cloned().collect(), None, None)]);
        assert_eq!(adjacency_map[&Node(2.0, 1.0)], vec![Edge::new(1.0, [2, 3].iter().cloned().collect(), None, None), Edge::new(1.0, [3, 4].iter().cloned().collect(), None, None)]);
        assert_eq!(adjacency_map[&Node(2.0, 3.0)], vec![Edge::new(1.0, [3, 4].iter().cloned().collect(), None, None), Edge::new(3.0, [1, 4].iter().cloned().collect(), None, None)]);
    }

    #[test]
    fn test_graph_adjacency_matrix()
    {
        let nodes = vec![Node(0.0, 1.0), Node(0.0, 2.0), Node(2.0, 3.0), Node(1.0, 4.0), Node(3.0, 4.0)];
        let edges = vec![Edge::new(2.0, [0, 1].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [0, 2].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [2, 3].iter().cloned().collect(), None, None),
                         Edge::new(3.0, [1, 4].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [3, 4].iter().cloned().collect(), None, None)];
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
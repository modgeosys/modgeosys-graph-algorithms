// A module containing an implementation of the A* algorithm for finding the shortest path between two nodes in a graph.

use std::cmp::Ordering;
use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::types::{Graph, NoNavigablePathError, Edge};



// A wrapper for an edge that includes the f() function, and the g and h values to support A*.
#[derive(Debug, Clone)]
pub struct Hop
{
    pub edge: Edge,
    pub g: OrderedFloat<f64>,
    pub h: OrderedFloat<f64>,
}

impl Hop
{
    pub fn new(edge: Edge, g: f64, h: f64) -> Self
    {
        Hop
        {
            edge,
            g: OrderedFloat(g),
            h: OrderedFloat(h),
        }
    }

    // Calculate the combined cost of the edge.
    pub fn f(&self) -> OrderedFloat<f64>
    {
        self.g + self.h
    }
}

impl PartialEq for Hop
{
    fn eq(&self, other: &Self) -> bool
    {
        self.edge == other.edge && self.g == other.g && self.h == other.h
    }
}

impl Eq for Hop {}

impl PartialOrd for Hop
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

impl Ord for Hop
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}


// Implement the A* algorithm for finding the shortest path between two nodes in a graph.
pub fn a_star(graph: &Graph, start_node_index: usize, goal_node_index: usize) -> Result<Vec<Hop>, NoNavigablePathError>
{
    // Grab the nodes and adjacency map from the graph.
    let nodes           = &graph.nodes;
    let adjacency_map   = graph.adjacency_map();
    let heuristic_distance = graph.distance_function;

    // Initialize the edge hop lists.
    let mut unhopped = graph.edges.clone();
    let mut hops     = Vec::new();

    // Current node begins with the start node.
    let mut current_node_index = start_node_index;

    let mut f = BTreeMap::new();
    let mut g = OrderedFloat(0.0f64);

    while current_node_index != goal_node_index
    {
        // Calculate f for each candidate edge we could hop next.
        for candidate_edge in adjacency_map[&nodes[current_node_index]].iter()
        {
            if unhopped.contains(candidate_edge)
            {
                let candidate_hop = Hop::new(candidate_edge.clone(),
                                             *candidate_edge.weight + *g,
                                             *heuristic_distance(&nodes[candidate_edge.index_of_other_node(current_node_index)], &nodes[goal_node_index]));
                f.insert(candidate_hop.f(), candidate_hop);
            }
        }

        // Pick the edge with the lowest f value; if no path to the goal exists, return an error.
        let Some((_, best_hop)) = f.pop_first()
            else { return Err(NoNavigablePathError { start_node: nodes[start_node_index].clone(), goal_node: nodes[goal_node_index].clone() }) };

        // Update cumulative g, the index of the currently-visited node, and the edge hop lists.
        g                  = best_hop.g;
        current_node_index = best_hop.edge.index_of_other_node(current_node_index);
        unhopped.retain(|edge_ref| *edge_ref != best_hop.edge);
        hops.push(best_hop);

        // Clear the auto-sorted f BTreeMap for reuse with the next hop calculation.
        f.clear();
    }

    Ok(hops)
}



#[cfg(test)]
mod tests
{
    use std::collections::HashSet;
    use super::*;
    use crate::distance::manhattan_distance;
    use crate::types::{PropertyValue, Edge, Node, WeightOption};
    use crate::test_fixtures::tests::{valid_nodes, valid_graph1, valid_graph2, valid_graph_from_edge_definitions};

    #[test]
    fn test_hop_creation()
    {
        let hop = Hop::new(Edge::new(HashSet::from([1, 2]), WeightOption::Specified(10.0), BTreeMap::new()), 5.0, 5.0);
        assert_eq!(hop.edge, Edge::new(HashSet::from([1, 2]), WeightOption::Specified(10.0), BTreeMap::new()));
        assert_eq!(hop.g, OrderedFloat(5.0f64));
        assert_eq!(hop.h, OrderedFloat(5.0f64));
    }

    #[test]
    fn test_hop_f_calculation()
    {
        let hop = Hop::new(Edge::new(HashSet::from([1, 2]), WeightOption::Specified(10.0), BTreeMap::new()), 5.0, 5.0);
        assert_eq!(hop.f(), OrderedFloat(10.0f64));
    }

    #[test]
    fn test_hop_equality()
    {
        let hop1 = Hop::new(Edge::new(HashSet::from([1, 2]), WeightOption::Specified(10.0), BTreeMap::new()), 5.0, 5.0);
        let hop2 = Hop::new(Edge::new(HashSet::from([1, 2]), WeightOption::Specified(10.0), BTreeMap::new()), 5.0, 5.0);
        assert_eq!(hop1, hop2);
    }

    #[test]
    fn test_a_star_finds_shortest_path_manhattan_graph1()
    {
        let expected = vec![Hop::new(Edge::new(HashSet::from([0, 1]), WeightOption::Specified(2.0), BTreeMap::new()), 2.0, 3.0),
                            Hop::new(Edge::new(HashSet::from([1, 4]), WeightOption::Specified(3.0), BTreeMap::new()), 5.0, 0.0)];

        assert_eq!(a_star(&valid_graph1(), 0, 4).unwrap(), expected);
    }

    #[test]
    fn test_a_star_finds_shortest_path_manhattan_graph_from_edge_definitions()
    {
        let expected = vec![Hop::new(Edge::new(HashSet::from([0, 2]), WeightOption::Specified(1.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()), 1.0, 4.0),
                            Hop::new(Edge::new(HashSet::from([2, 3]), WeightOption::Specified(2.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()), 3.0, 2.0),
                            Hop::new(Edge::new(HashSet::from([3, 4]), WeightOption::Specified(2.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()), 5.0, 0.0)];

        assert_eq!(a_star(&valid_graph_from_edge_definitions(), 0, 4).unwrap(), expected);
    }

    #[test]
    fn test_a_star_finds_shortest_path_manhattan_graph2()
    {
        let expected = vec![Hop::new(Edge::new(HashSet::from([0, 2]), WeightOption::Specified(1.0), BTreeMap::new()), 1.0, 4.0),
                            Hop::new(Edge::new(HashSet::from([2, 3]), WeightOption::Specified(1.0), BTreeMap::new()), 2.0, 2.0),
                            Hop::new(Edge::new(HashSet::from([3, 4]), WeightOption::Specified(1.0), BTreeMap::new()), 3.0, 0.0)];

        assert_eq!(a_star(&valid_graph2(), 0, 4).unwrap(), expected);
    }

    #[test]
    fn test_a_star_with_no_path_manhattan()
    {
        let nodes = valid_nodes();
        let edges: Vec<Edge> = Vec::new();

        assert!(a_star(&Graph::new(nodes, edges, BTreeMap::new(), manhattan_distance, None), 0, 3).is_err());
    }

    #[test]
    fn test_a_star_with_single_node_path_manhattan()
    {
        let nodes = vec![Node::new(vec![0.0, 0.0], BTreeMap::new())];
        let edges: Vec<Edge> = Vec::new();

        let expected: Vec<Hop> = Vec::new();

        assert_eq!(a_star(&Graph::new(nodes, edges, BTreeMap::new(), manhattan_distance, None), 0, 0).unwrap(), expected);
    }
}

// TODO: Add tests for euclidean distance, and many more permutations of the above tests.
// A module containing an implementation of the A* algorithm for finding the shortest path between two nodes in a graph.

use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::types::{EdgeTransit, Graph, Node, NoNavigablePathError};



// Implement the A* algorithm for finding the shortest path between two nodes in a graph.
pub fn a_star(graph: &Graph, start_node_index: usize, goal_node_index: usize, heuristic_distance: fn(&Node, &Node) -> OrderedFloat<f64>) -> Result<Vec<EdgeTransit>, NoNavigablePathError>
{
    // Grab the nodes and adjacency map from the graph.
    let nodes           = &graph.nodes;
    let adjacency_map   = graph.adjacency_map();

    // Initialize the edge traversal lists.
    let mut untraversed = graph.edges.clone();
    let mut traversed   = Vec::new();

    // Current node begins with the start node.
    let mut current_node_index = start_node_index;

    let mut f = BTreeMap::new();
    let mut g = OrderedFloat(0.0f64);

    while current_node_index != goal_node_index
    {
        // Calculate f for each candidate edge we could traverse next.
        for candidate_edge in adjacency_map[&nodes[current_node_index]].iter()
        {
            if untraversed.contains(candidate_edge)
            {
                let candidate_transit = EdgeTransit::new(candidate_edge.clone(),
                                                         *candidate_edge.weight + *g,
                                                         *heuristic_distance(&nodes[candidate_edge.index_of_other_node(current_node_index)],
                                                                             &nodes[goal_node_index]));
                f.insert(candidate_transit.f(), candidate_transit);
            }
        }

        // Pick the edge with the lowest f value; if no path to the goal exists, return an error.
        let Some((_, best_transit)) = f.pop_first()
            else { return Err(NoNavigablePathError { start_node: nodes[start_node_index].clone(), goal_node: nodes[goal_node_index].clone() }) };

        // Update cumulative g, the index of the currently-visited node, and the edge traversal lists.
        g                  = best_transit.g;
        current_node_index = best_transit.edge.index_of_other_node(current_node_index);
        untraversed.retain(|edge_ref| *edge_ref != best_transit.edge);
        traversed.push(best_transit);

        // Clear the auto-sorted f BTreeMap for reuse with the next traversal calculation.
        f.clear();
    }

    Ok(traversed)
}



#[cfg(test)]
mod tests
{
    use std::collections::HashSet;
    use super::*;
    use crate::distance::manhattan_distance;
    use crate::types::{Edge, Node};
    use crate::test_fixtures::tests::{valid_nodes, valid_graph1, valid_graph2};

    #[test]
    fn test_a_star_finds_shortest_path_manhattan_graph1()
    {
        let expected = vec![EdgeTransit::new(Edge::new(2.0, HashSet::from([0, 1])), 2.0, 3.0),
                            EdgeTransit::new(Edge::new(3.0, HashSet::from([1, 4])), 5.0, 0.0)];

        assert_eq!(a_star(&valid_graph1(), 0, 4, manhattan_distance).unwrap(), expected);
    }

    #[test]
    fn test_a_star_finds_shortest_path_manhattan_graph2()
    {
        let expected = vec![EdgeTransit::new(Edge::new(1.0, HashSet::from([0, 2])), 1.0, 4.0),
                            EdgeTransit::new(Edge::new(1.0, HashSet::from([2, 3])), 2.0, 2.0),
                            EdgeTransit::new(Edge::new(1.0, HashSet::from([3, 4])), 3.0, 0.0)];

        assert_eq!(a_star(&valid_graph2(), 0, 4, manhattan_distance).unwrap(), expected);
    }

    #[test]
    fn test_a_star_with_no_path_manhattan()
    {
        let nodes = valid_nodes();
        let edges: Vec<Edge> = Vec::new();

        assert!(a_star(&Graph::new(nodes, edges), 0, 3, manhattan_distance).is_err());
    }

    #[test]
    fn test_a_star_with_single_node_path_manhattan()
    {
        let nodes = vec![Node::new(0.0, 0.0)];
        let edges: Vec<Edge> = Vec::new();

        let expected: Vec<EdgeTransit> = Vec::new();

        assert_eq!(a_star(&Graph::new(nodes, edges), 0, 0, manhattan_distance).unwrap(), expected);
    }
}

// TODO: Add tests for euclidean distance, and many more permutations of the above tests.
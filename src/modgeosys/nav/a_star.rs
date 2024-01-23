use std::collections::BinaryHeap;
use std::cmp::Ordering;

use crate::modgeosys::nav::types::Node;
use crate::modgeosys::nav::types::Edge;
use crate::modgeosys::nav::types::Graph;
use crate::modgeosys::nav::types::NoNavigablePathError;
use crate::modgeosys::nav::distance::manhattan_distance;


pub fn a_star(graph: &Graph, start_node_index: usize, goal_node_index: usize) -> Result<Vec<Edge>, NoNavigablePathError>
{
    let nodes = &graph.nodes;
    let adjacency_map = graph.adjacency_map();

    let mut untraversed = graph.edges.clone();
    let mut traversed = Vec::new();

    let mut current_node_index = start_node_index;

    let mut f = BinaryHeap::new();
    let mut g = 0;

    while current_node_index != goal_node_index
    {
        for candidate_edge in &adjacency_map[nodes[current_node_index]]
        {
            if untraversed.contains(candidate_edge)
            {
                candidate_edge.g = candidate_edge.weight + g;
                candidate_edge.h = manhattan_distance(&nodes[candidate_edge.coordinates_of_other(current_node_index)], &nodes[goal_node_index]);
                f.push(candidate_edge.f());
            }
        }

        if f.is_empty()
        {
            return Err(NoNavigablePathError { start_node: nodes[start_node_index].clone(), goal_node: nodes[goal_node_index].clone() });
        }

        let best_f = f.pop().unwrap();

        g = best_f.g;
        untraversed.retain(|edge| edge != &best_f);
        traversed.push(best_f.clone());
        current_node_index = best_f.coordinates_of_other(current_node_index);

        f.clear();
    }

    Ok(traversed)
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_a_star_finds_shortest_path_manhattan_graph1()
    {
        let nodes = vec![Node(0.0, 1.0), Node(0.0, 2.0), Node(2.0, 3.0), Node(1.0, 4.0), Node(3.0, 4.0)];
        let edges = vec![Edge::new(2.0, [0, 1].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [0, 2].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [2, 3].iter().cloned().collect(), None, None),
                         Edge::new(3.0, [1, 4].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [3, 4].iter().cloned().collect(), None, None)];
        let graph = Graph::new(nodes, edges);

        let expected = vec![Edge::new(2.0, [0, 1].iter().cloned().collect(), Some(2.0), Some(3.0)),
                            Edge::new(3.0, [1, 4].iter().cloned().collect(), Some(5.0), Some(0.0))];

        assert_eq!(a_star(&graph, 0, 4).unwrap(), expected);
    }

    #[test]
    fn test_a_star_finds_shortest_path_manhattan_graph2()
    {
        let nodes = vec![Node(0.0, 1.0), Node(0.0, 2.0), Node(2.0, 3.0), Node(1.0, 4.0), Node(3.0, 4.0)];
        let edges = vec![Edge::new(3.0, [0, 1].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [0, 2].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [2, 3].iter().cloned().collect(), None, None),
                         Edge::new(3.0, [1, 4].iter().cloned().collect(), None, None),
                         Edge::new(1.0, [3, 4].iter().cloned().collect(), None, None)];
        let graph = Graph::new(nodes, edges);

        let expected = vec![Edge::new(1.0, [0, 2].iter().cloned().collect(), Some(1.0), Some(4.0)),
                            Edge::new(1.0, [2, 3].iter().cloned().collect(), Some(2.0), Some(2.0)),
                            Edge::new(1.0, [3, 4].iter().cloned().collect(), Some(3.0), Some(0.0))];

        assert_eq!(a_star(&graph, 0, 4).unwrap(), expected);
    }

    #[test]
    fn test_a_star_with_no_path_manhattan()
    {
        let nodes = vec![Node(0.0, 1.0), Node(0.0, 2.0), Node(2.0, 3.0), Node(1.0, 4.0), Node(3.0, 4.0)];
        let edges: Vec<Edge> = Vec::new();
        let graph = Graph::new(nodes, edges);

        assert!(a_star(&graph, 0, 3).is_err());
    }

    #[test]
    fn test_a_star_with_single_node_path_manhattan()
    {
        let nodes = vec![Node(0.0, 0.0)];
        let edges: Vec<Edge> = Vec::new();
        let graph = Graph::new(nodes, edges);

        let expected: Vec<Edge> = Vec::new();

        assert_eq!(a_star(&graph, 0, 0).unwrap(), expected);
    }
}

// TODO: Add tests for euclidean distance, and many more permutations of the above tests.
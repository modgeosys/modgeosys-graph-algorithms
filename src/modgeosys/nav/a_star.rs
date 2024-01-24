use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::modgeosys::nav::types::{Edge, Graph, NoNavigablePathError};
use crate::modgeosys::nav::distance::manhattan_distance;


pub fn a_star(graph: &Graph, start_node_index: usize, goal_node_index: usize) -> Result<Vec<Edge>, NoNavigablePathError>
{
    let nodes = &graph.nodes;
    let adjacency_map = graph.adjacency_map();

    let mut untraversed = graph.edges.clone();
    let mut traversed = Vec::new();

    let mut current_node_index = start_node_index;

    let mut f = BTreeMap::new();
    let mut g = OrderedFloat(0.0f64);

    while current_node_index != goal_node_index
    {
        for candidate_edge in &mut adjacency_map[&nodes[current_node_index]]
        {
            if untraversed.contains(candidate_edge)
            {
                candidate_edge.g = Some(candidate_edge.weight + g);
                candidate_edge.h = Some(manhattan_distance(&nodes[candidate_edge.coordinates_of_other(current_node_index)], &nodes[goal_node_index]));
                f.insert(candidate_edge.f().unwrap(), candidate_edge);
            }
        }

        // if f.is_empty()
        // {
        //     return Err(NoNavigablePathError { start_node: nodes[start_node_index].clone(), goal_node: nodes[goal_node_index].clone() });
        // }
        //
        // let Some((best_f, best_transit_edge)) = f.pop_first().ok_or(NoNavigablePathError { start_node: nodes[start_node_index].clone(), goal_node: nodes[goal_node_index].clone() });
        let Some((_, best_transit_edge)) = f.pop_first() else { return Err(NoNavigablePathError { start_node: nodes[start_node_index].clone(), goal_node: nodes[goal_node_index].clone() }) };

        g = best_transit_edge.g.unwrap();
        untraversed.retain(|edge| edge != best_transit_edge);
        traversed.push(best_transit_edge.clone());
        current_node_index = best_transit_edge.coordinates_of_other(current_node_index);

        f.clear();
    }

    Ok(traversed)
}


#[cfg(test)]
mod tests
{
    use super::*;
    use crate::modgeosys::nav::types::Node;

    #[test]
    fn test_a_star_finds_shortest_path_manhattan_graph1()
    {
        let nodes = vec![Node::new(0.0, 1.0), Node::new(0.0, 2.0), Node::new(2.0, 3.0), Node::new(1.0, 4.0), Node::new(3.0, 4.0)];
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
        let nodes = vec![Node::new(0.0, 1.0), Node::new(0.0, 2.0), Node::new(2.0, 3.0), Node::new(1.0, 4.0), Node::new(3.0, 4.0)];
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
        let nodes = vec![Node::new(0.0, 1.0), Node::new(0.0, 2.0), Node::new(2.0, 3.0), Node::new(1.0, 4.0), Node::new(3.0, 4.0)];
        let edges: Vec<Edge> = Vec::new();
        let graph = Graph::new(nodes, edges);

        assert!(a_star(&graph, 0, 3).is_err());
    }

    #[test]
    fn test_a_star_with_single_node_path_manhattan()
    {
        let nodes = vec![Node::new(0.0, 0.0)];
        let edges: Vec<Edge> = Vec::new();
        let graph = Graph::new(nodes, edges);

        let expected: Vec<Edge> = Vec::new();

        assert_eq!(a_star(&graph, 0, 0).unwrap(), expected);
    }
}

// TODO: Add tests for euclidean distance, and many more permutations of the above tests.
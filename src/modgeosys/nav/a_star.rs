use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::modgeosys::nav::types::{EdgeTransit, Graph, NoNavigablePathError};
use crate::modgeosys::nav::distance::manhattan_distance;


pub fn a_star(graph: &Graph, start_node_index: usize, goal_node_index: usize) -> Result<Vec<EdgeTransit>, NoNavigablePathError>
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
        for candidate_edge in adjacency_map[&nodes[current_node_index]].iter()
        {
            if untraversed.contains(candidate_edge)
            {
                let candidate_transit = EdgeTransit::new(candidate_edge.clone(),
                                                         *candidate_edge.weight + *g,
                                                         *manhattan_distance(&nodes[candidate_edge.index_of_other_node(current_node_index)],
                                                                             &nodes[goal_node_index]));
                f.insert(candidate_transit.f(), candidate_transit);
            }
        }

        let Some((_, best_transit)) = f.pop_first() else { return Err(NoNavigablePathError { start_node: nodes[start_node_index].clone(), goal_node: nodes[goal_node_index].clone() }) };

        g = best_transit.g;
        untraversed.retain(|edge_ref| *edge_ref != best_transit.edge);
        traversed.push(best_transit.clone());
        current_node_index = best_transit.edge.index_of_other_node(current_node_index);

        f.clear();
    }

    Ok(traversed)
}


#[cfg(test)]
mod tests
{
    use super::*;
    use crate::modgeosys::nav::types::{Node, Edge};
    use std::collections::HashSet;

    #[test]
    fn test_a_star_finds_shortest_path_manhattan_graph1()
    {
        let nodes = vec![Node::new(0.0, 0.0), Node::new(0.0, 2.0), Node::new(1.0, 0.0), Node::new(2.0, 1.0), Node::new(2.0, 3.0)];
        let edges = vec![Edge::new(2.0, HashSet::from([0, 1])),
                         Edge::new(1.0, HashSet::from([0, 2])),
                         Edge::new(1.0, HashSet::from([2, 3])),
                         Edge::new(3.0, HashSet::from([1, 4])),
                         Edge::new(1.0, HashSet::from([3, 4]))];
        let graph = Graph::new(nodes, edges);

        let expected = vec![EdgeTransit::new(Edge::new(2.0, HashSet::from([0, 1])), 2.0, 3.0),
                            EdgeTransit::new(Edge::new(3.0, HashSet::from([1, 4])), 5.0, 0.0)];

        assert_eq!(a_star(&graph, 0, 4).unwrap(), expected);
    }

    #[test]
    fn test_a_star_finds_shortest_path_manhattan_graph2()
    {
        let nodes = vec![Node::new(0.0, 0.0), Node::new(0.0, 2.0), Node::new(1.0, 0.0), Node::new(2.0, 1.0), Node::new(2.0, 3.0)];
        let edges = vec![Edge::new(3.0, HashSet::from([0, 1])),
                         Edge::new(1.0, HashSet::from([0, 2])),
                         Edge::new(1.0, HashSet::from([2, 3])),
                         Edge::new(3.0, HashSet::from([1, 4])),
                         Edge::new(1.0, HashSet::from([3, 4]))];
        let graph = Graph::new(nodes, edges);

        let expected = vec![EdgeTransit::new(Edge::new(1.0, HashSet::from([0, 2])), 1.0, 4.0),
                            EdgeTransit::new(Edge::new(1.0, HashSet::from([2, 3])), 2.0, 2.0),
                            EdgeTransit::new(Edge::new(1.0, HashSet::from([3, 4])), 3.0, 0.0)];

        assert_eq!(a_star(&graph, 0, 4).unwrap(), expected);
    }

    #[test]
    fn test_a_star_with_no_path_manhattan()
    {
        let nodes = vec![Node::new(0.0, 0.0), Node::new(0.0, 2.0), Node::new(1.0, 0.0), Node::new(2.0, 1.0), Node::new(2.0, 3.0)];
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

        let expected: Vec<EdgeTransit> = Vec::new();

        assert_eq!(a_star(&graph, 0, 0).unwrap(), expected);
    }
}

// TODO: Add tests for euclidean distance, and many more permutations of the above tests.
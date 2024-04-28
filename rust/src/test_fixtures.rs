#[cfg(test)]
pub mod tests
{
    use std::collections::{HashSet, BTreeMap};
    use ordered_float::{OrderedFloat};
    use crate::edge_weight::length_cost_per_unit;
    use crate::distance::manhattan_distance;
    use crate::types::{Edge, EdgeDefinition, Graph, Node, PropertyValue, WeightOption};

    pub fn valid_nodes() -> Vec<Node>
    {
        let nodes = vec![Node::new(vec![0.0, 0.0], BTreeMap::new()),
                         Node::new(vec![0.0, 2.0], BTreeMap::new()),
                         Node::new(vec![1.0, 0.0], BTreeMap::new()),
                         Node::new(vec![2.0, 1.0], BTreeMap::new()),
                         Node::new(vec![2.0, 3.0], BTreeMap::new())];
        nodes
    }

    pub fn valid_edges1() -> Vec<Edge>
    {
        let edges = vec![Edge::new(HashSet::from([0, 1]), WeightOption::Specified(2.0), BTreeMap::new()),
                         Edge::new(HashSet::from([0, 2]), WeightOption::Specified(1.0), BTreeMap::new()),
                         Edge::new(HashSet::from([2, 3]), WeightOption::Specified(1.0), BTreeMap::new()),
                         Edge::new(HashSet::from([1, 4]), WeightOption::Specified(3.0), BTreeMap::new()),
                         Edge::new(HashSet::from([3, 4]), WeightOption::Specified(1.0), BTreeMap::new())];
        edges
    }

    pub fn valid_edges1_with_computed_weights() -> Vec<Edge>
    {
        let edges = vec![Edge::new(HashSet::from([0, 1]), WeightOption::Specified(4.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(2.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([0, 2]), WeightOption::Specified(1.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([2, 3]), WeightOption::Specified(2.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([1, 4]), WeightOption::Specified(9.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(3.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([3, 4]), WeightOption::Specified(2.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect())];
        edges
    }

    pub fn valid_edges2() -> Vec<Edge>
    {
        let edges = vec![Edge::new(HashSet::from([0, 1]), WeightOption::Specified(3.0), BTreeMap::new()),
                         Edge::new(HashSet::from([0, 2]), WeightOption::Specified(1.0), BTreeMap::new()),
                         Edge::new(HashSet::from([2, 3]), WeightOption::Specified(1.0), BTreeMap::new()),
                         Edge::new(HashSet::from([1, 4]), WeightOption::Specified(3.0), BTreeMap::new()),
                         Edge::new(HashSet::from([3, 4]), WeightOption::Specified(1.0), BTreeMap::new())];
        edges
    }

    pub fn valid_edges3() -> Vec<Edge>
    {
        let edges = vec![Edge::new(HashSet::from([0, 1]), WeightOption::Computed, [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(2.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([0, 2]), WeightOption::Computed, [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([2, 3]), WeightOption::Computed, [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([1, 4]), WeightOption::Computed, [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(3.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([3, 4]), WeightOption::Computed, [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect())];
        edges
    }

    pub fn valid_edges3_with_computed_weights() -> Vec<Edge>
    {
        let edges = vec![Edge::new(HashSet::from([0, 1]), WeightOption::Specified(4.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(2.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([0, 2]), WeightOption::Specified(1.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([2, 3]), WeightOption::Specified(2.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([1, 4]), WeightOption::Specified(9.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(3.0)))].iter().cloned().collect()),
                         Edge::new(HashSet::from([3, 4]), WeightOption::Specified(2.0), [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect())];
        edges
    }

    pub fn valid_graph1() -> Graph
    {
        let graph = Graph::new(valid_nodes(), valid_edges1(), BTreeMap::new(), manhattan_distance, None);
        graph
    }

    pub fn valid_graph2() -> Graph
    {
        let graph = Graph::new(valid_nodes(), valid_edges2(), BTreeMap::new(), manhattan_distance, None);
        graph
    }

    pub fn valid_graph3() -> Graph
    {
        let graph = Graph::new(valid_nodes(), valid_edges3(), BTreeMap::new(), manhattan_distance, Some(length_cost_per_unit));
        graph
    }

    pub fn valid_graph_from_edge_definitions() -> Graph
    {
        let edge_definitions = vec![EdgeDefinition(vec![vec![0.0, 0.0], vec![0.0, 2.0]], WeightOption::Computed, [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(2.0)))].iter().cloned().collect()),
                                    EdgeDefinition(vec![vec![0.0, 0.0], vec![1.0, 0.0]], WeightOption::Computed, [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                                    EdgeDefinition(vec![vec![1.0, 0.0], vec![2.0, 1.0]], WeightOption::Computed, [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                                    EdgeDefinition(vec![vec![0.0, 2.0], vec![2.0, 3.0]], WeightOption::Computed, [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(3.0)))].iter().cloned().collect()),
                                    EdgeDefinition(vec![vec![2.0, 1.0], vec![2.0, 3.0]], WeightOption::Computed, [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect())];
        let graph = Graph::from_edge_definitions(edge_definitions, BTreeMap::new(), manhattan_distance, Some(length_cost_per_unit));

        graph
    }
}

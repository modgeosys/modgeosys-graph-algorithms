// Heuristic distance functions.

use ordered_float::OrderedFloat;

use crate::types::Node;



// Calculate the Manhattan distance between two points.
pub fn manhattan_distance(a: &Node, b: &Node) -> OrderedFloat<f64>
{
    let distance: f64 = a.coordinates.iter().zip(b.coordinates.iter()).map(|(a, b)| (*a - *b).abs()).sum();
    OrderedFloat(distance)
}

// Calculate the Euclidean distance between two points.
pub fn euclidean_distance(a: &Node, b: &Node) -> OrderedFloat<f64>
{
    let distance: f64 = a.coordinates.iter().zip(b.coordinates.iter()).map(|(a, b)| (*a - *b).powi(2)).sum::<f64>().sqrt();
    OrderedFloat(distance)
}

// Calculate the Squared Euclidean distance between two points.
pub fn squared_euclidean_distance(a: &Node, b: &Node) -> OrderedFloat<f64>
{
    let distance: f64 = a.coordinates.iter().zip(b.coordinates.iter()).map(|(a, b)| (*a - *b).powi(2)).sum();
    OrderedFloat(distance)
}



#[cfg(test)]
mod tests
{
    // TODO: Add tests for n-dimensional points.

    use std::collections::BTreeMap;
    use super::*;

    #[test]
    fn test_manhattan_distance_between_identical_points_is_zero()
    {
        let a = Node::new(vec![1.0, 2.0], BTreeMap::new());
        let b = Node::new(vec![1.0, 2.0], BTreeMap::new());
        assert_eq!(manhattan_distance(&a, &b), OrderedFloat(0.0f64));
    }

    #[test]
    fn test_manhattan_distance_between_points_on_same_axis_is_absolute_difference()
    {
        let a = Node::new(vec![1.0, 2.0], BTreeMap::new());
        let b = Node::new(vec![1.0, 5.0], BTreeMap::new());
        assert_eq!(manhattan_distance(&a, &b), OrderedFloat(3.0f64));
    }

    #[test]
    fn test_manhattan_distance_between_points_on_different_axes_is_sum_of_absolute_differences()
    {
        let a = Node::new(vec![0.0, 0.0], BTreeMap::new());
        let b = Node::new(vec![1.0, 1.0], BTreeMap::new());
        assert_eq!(manhattan_distance(&a, &b), OrderedFloat(2.0f64));
    }

    #[test]
    fn test_euclidean_distance_between_identical_points_is_zero()
    {
        let a = Node::new(vec![1.0, 2.0], BTreeMap::new());
        let b = Node::new(vec![1.0, 2.0], BTreeMap::new());
        assert_eq!(euclidean_distance(&a, &b), OrderedFloat(0.0f64));
    }

    #[test]
    fn test_euclidean_distance_between_points_on_same_axis_is_absolute_difference()
    {
        let a = Node::new(vec![1.0, 2.0], BTreeMap::new());
        let b = Node::new(vec![1.0, 5.0], BTreeMap::new());
        assert_eq!(euclidean_distance(&a, &b), OrderedFloat(3.0f64));
    }

    #[test]
    fn test_euclidean_distance_follows_pythagorean_theorem()
    {
        let a = Node::new(vec![0.0, 0.0], BTreeMap::new());
        let b = Node::new(vec![3.0, 4.0], BTreeMap::new());
        assert_eq!(euclidean_distance(&a, &b), OrderedFloat(5.0f64));
    }

    #[test]
    fn test_squared_euclidean_distance_between_identical_points_is_zero()
    {
        let a = Node::new(vec![1.0, 2.0], BTreeMap::new());
        let b = Node::new(vec![1.0, 2.0], BTreeMap::new());
        assert_eq!(squared_euclidean_distance(&a, &b), OrderedFloat(0.0f64));
    }

    #[test]
    fn test_squared_euclidean_distance_between_points_on_same_axis_is_squared_absolute_difference()
    {
        let a = Node::new(vec![1.0, 2.0], BTreeMap::new());
        let b = Node::new(vec![1.0, 5.0], BTreeMap::new());
        assert_eq!(squared_euclidean_distance(&a, &b), OrderedFloat(9.0f64));
    }

    #[test]
    fn test_squared_euclidean_distance_on_different_axes_follows_pythagorean_theorem()
    {
        let a = Node::new(vec![0.0, 0.0], BTreeMap::new());
        let b = Node::new(vec![3.0, 4.0], BTreeMap::new());
        assert_eq!(squared_euclidean_distance(&a, &b), OrderedFloat(25.0f64));
    }
}

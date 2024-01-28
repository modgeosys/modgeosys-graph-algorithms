// Heuristic distance functions.

use ordered_float::OrderedFloat;

use crate::types::Node;



// Calculate the Manhattan distance between two points.
pub fn manhattan_distance(a: &Node, b: &Node) -> OrderedFloat<f64>
{
    OrderedFloat((a.x - b.x).abs() + (a.y - b.y).abs())
}

// Calculate the Euclidean distance between two points.
pub fn euclidean_distance(a: &Node, b: &Node) -> OrderedFloat<f64>
{
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    OrderedFloat(((dx * dx) + (dy * dy)).sqrt())
}



#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_manhattan_distance_between_identical_points_is_zero()
    {
        let a = Node::new(1.0, 2.0);
        let b = Node::new(1.0, 2.0);
        assert_eq!(manhattan_distance(&a, &b), OrderedFloat(0.0f64));
    }

    #[test]
    fn test_manhattan_distance_between_points_on_same_axis_is_absolute_difference()
    {
        let a = Node::new(1.0, 2.0);
        let b = Node::new(1.0, 5.0);
        assert_eq!(manhattan_distance(&a, &b), OrderedFloat(3.0f64));
    }

    #[test]
    fn test_manhattan_distance_between_points_on_different_axes_is_sum_of_absolute_differences()
    {
        let a = Node::new(0.0, 0.0);
        let b = Node::new(1.0, 1.0);
        assert_eq!(manhattan_distance(&a, &b), OrderedFloat(2.0f64));
    }

    #[test]
    fn test_euclidean_distance_between_identical_points_is_zero()
    {
        let a = Node::new(1.0, 2.0);
        let b = Node::new(1.0, 2.0);
        assert_eq!(euclidean_distance(&a, &b), OrderedFloat(0.0f64));
    }

    #[test]
    fn test_euclidean_distance_between_points_on_same_axis_is_absolute_difference()
    {
        let a = Node::new(1.0, 2.0);
        let b = Node::new(1.0, 5.0);
        assert_eq!(euclidean_distance(&a, &b), OrderedFloat(3.0f64));
    }

    #[test]
    fn test_euclidean_distance_follows_pythagorean_theorem()
    {
        let a = Node::new(0.0, 0.0);
        let b = Node::new(3.0, 4.0);
        assert_eq!(euclidean_distance(&a, &b), OrderedFloat(5.0f64));
    }
}
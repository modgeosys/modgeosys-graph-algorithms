use std::cmp::Ordering;

use crate::modgeosys::nav::types::Node;

pub fn manhattan_distance(a: &Node, b: &Node) -> f64
{
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn euclidean_distance(a: &Node, b: &Node) -> f64
{
    let d0 = a.0 - b.0;
    let d1 = a.1 - b.1;
    ((d0 * d0 + d1 * d1) as f64).sqrt()
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_manhattan_distance_between_identical_points_is_zero()
    {
        let a = Node(1.0, 2.0);
        let b = Node(1.0, 2.0);
        assert_eq!(manhattan_distance(&a, &b), 0.0);
    }

    #[test]
    fn test_manhattan_distance_between_points_on_same_axis_is_absolute_difference()
    {
        let a = Node(1.0, 2.0);
        let b = Node(1.0, 5.0);
        assert_eq!(manhattan_distance(&a, &b), 3.0);
    }

    #[test]
    fn test_manhattan_distance_between_points_on_different_axes_is_sum_of_absolute_differences()
    {
        let a = Node(0.0, 0.0);
        let b = Node(1.0, 1.0);
        assert_eq!(manhattan_distance(&a, &b), 2.0);
    }

    #[test]
    fn test_euclidean_distance_between_identical_points_is_zero()
    {
        let a = Node(1.0, 2.0);
        let b = Node(1.0, 2.0);
        assert_eq!(euclidean_distance(&a, &b), 0.0);
    }

    #[test]
    fn test_euclidean_distance_between_points_on_same_axis_is_absolute_difference()
    {
        let a = Node(1.0, 2.0);
        let b = Node(1.0, 5.0);
        assert_eq!(euclidean_distance(&a, &b), 3.0);
    }

    #[test]
    fn test_euclidean_distance_follows_pythagorean_theorem()
    {
        let a = Node(0.0, 0.0);
        let b = Node(3.0, 4.0);
        assert_eq!(euclidean_distance(&a, &b), (5.0 as f64).sqrt());
    }
}
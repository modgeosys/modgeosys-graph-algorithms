use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use crate::a_star::a_star;
use crate::types::{EdgeTransit, Graph, Node, NoNavigablePathError};
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

#[pymodule]
fn astar_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(a_star_wrapper, m)?)?;
    Ok(())
}

#[pyfunction]
fn a_star_wrapper(graph: &Graph, start_node_index: usize, goal_node_index: usize, heuristic_distance: fn(&Node, &Node) -> OrderedFloat<f64>) -> PyResult<Vec<EdgeTransit>>
{
    match a_star(graph, start_node_index, goal_node_index, heuristic_distance)
    {
        Ok(result) => Ok(result),
        Err(err) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", err))),
    }
}
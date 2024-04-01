use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::Python;
use pyo3::types::{PyFloat, IntoPyDict};
use crate::a_star::a_star;
use crate::types::{EdgeTransit, Graph, Node, Edge, NoNavigablePathError};
use crate::distance::{manhattan_distance, euclidean_distance, squared_euclidean_distance};
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

#[pymodule]
fn astar_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Node>()?;
    m.add_class::<Edge>()?;
    m.add_class::<EdgeTransit>()?;
    m.add_class::<Graph>()?;
    m.add_class::<NoNavigablePathError>()?;
    m.add_function(wrap_pyfunction!(a_star_wrapper, m)?)?;
    // m.add_function(wrap_pyfunction!(manhattan_distance, m)?)?;
    // m.add_function(wrap_pyfunction!(euclidean_distance, m)?)?;
    // m.add_function(wrap_pyfunction!(squared_euclidean_distance, m)?)?;
    // m.add_function(wrap_pyfunction!(manhattan_distance_wrapper, m)?)?;
    Ok(())
}

#[pyfunction]
fn manhattan_distance_wrapper(a: &Node, b: &Node) -> PyFloat
{
    let distance = manhattan_distance(a, b);
    let py = Python::<'unbound>::assume_gil_acquired();
    PyFloat::new_bound(py, distance.into_inner())
}

#[pyfunction]
fn euclidean_distance_wrapper(a: &Node, b: &Node) -> PyFloat
{
    let distance = euclidean_distance(a, b);
    let gil = Python::acquire_gil();
    let py = gil.python();
    PyFloat::new_bound(py, distance.into_inner())
}

#[pyfunction]
fn squared_euclidean_distance_wrapper(a: &Node, b: &Node) -> PyFloat
{
    let distance = squared_euclidean_distance(a, b);
    let gil = Python::acquire_gil();
    let py = gil.python();
    PyFloat::new_bound(py, distance.into_inner())
}

#[pyfunction]
fn a_star_wrapper(graph: &Graph, start_node_index: usize, goal_node_index: usize, heuristic_distance: PyObject) -> PyResult<Vec<EdgeTransit>>
{
    let gil = Python::acquire_gil();
    let py = gil.python();
    let heuristic_distance = heuristic_distance.as_ref(py).extract::<fn(&Node, &Node) -> OrderedFloat<f64>>()?;
    match a_star(graph, start_node_index, goal_node_index, heuristic_distance)
    {
        Ok(result) => Ok(result),
        Err(err) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", err))),
    }
}
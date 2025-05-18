use layout::gv::parser::ast::NodeStmt;
use pyo3::prelude::*;
use std::cmp::Ordering;
use std::io;
use layout::core::utils::save_to_file;
use layout::gv::parser::ast::{Graph, NodeId, Stmt};
use layout::gv::GraphBuilder;

use layout::backends::svg::SVGWriter;
#[pyclass(module = "laypy._laypy")]
pub struct Digraph {
    graph: Graph,
}

#[pymethods]
impl Digraph {
    #[new]
    fn new() -> PyResult<Self> {
        let graph = Graph::new("my_graph");
        Ok(Self { graph })
    }

    fn add_node(&mut self) -> PyResult<()> {
        let node_statement = NodeStmt::new(
            NodeId::new("node1", &None),
        );
        let statement = Stmt::Node(
            node_statement.clone(),
        );
        self.graph.list.list.push(
            statement
        );
        Ok(())
    }

    fn render(&self, format: &str, file_name: &str) -> PyResult<()> {
        let mut gb = GraphBuilder::new();
        gb.visit_graph(&self.graph);
        let mut vg = gb.get();
        let mut svg = SVGWriter::new();
        vg.do_it(
            false,
            false,
            false,
            &mut svg,
        );
        let content = svg.finalize();
    
        let res = save_to_file(file_name, &content);
        Ok(())
    }

    fn node(&mut self, name: &str, label: &str, fillcolor: &str) -> PyResult<()> {
        let node_statement = NodeStmt::new(
            NodeId::new(name, &None),
        );
        let statement = Stmt::Node(
            node_statement.clone(),
        );
        self.graph.list.list.push(
            statement
        );
        Ok(())
    }

    fn subgraph(&mut self, name: &str) -> PyResult<()> {
        Ok(())
    }

    fn num_rows(&self) -> PyResult<usize> {
        // Assuming you want to return the number of nodes in the graph
        let num_nodes = self.graph.list.list.len();
        Ok(num_nodes)
    }

    fn update_graph_attr(&mut self, size: &str) -> PyResult<()> {
        // // Assuming you want to update some attribute of the graph
        // // For example, setting the graph's name
        // self.graph.name = attr.to_string();
        Ok(())
    }

    fn source(&self) -> PyResult<String> {
        // Assuming you want to return the source of the graph as a string
        // let source = self.graph.to_string();
        Ok("dummy source".to_string())
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _laypy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(guess_the_number, m)?)?;
    m.add_class::<Digraph>()?;
    Ok(())
}
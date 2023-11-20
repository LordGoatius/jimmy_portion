use std::collections::HashMap;
use itertools::{iproduct, Itertools};
use permutator::Combination;
use petgraph::prelude::*;
use pyo3::prelude::*;

use crate::prod::CartesianProduct;

pub mod prod;

/// Taken in a list of nodes and edges and prints a minimum coloring and returns a dict
/// representing the graph
#[pyfunction]
fn bf_chromo_coloring(nodes: Vec<usize>, edges: Vec<(usize, usize)>) -> PyResult<HashMap<usize, Vec<usize>>> {
    let mut graph: UnGraphMap<usize, usize> = GraphMap::default();
    for node in nodes {
        graph.add_node(node);
    }

    for (i, j) in edges {
        graph.add_edge(i, j, 1);
    }

    let mut map: HashMap<usize, Vec<usize>> = HashMap::default();

    for node in graph.nodes() {
        let neighbors: Vec<usize> = graph.neighbors(node).collect();
        map.insert(node, neighbors);
    }

    let coloring = find_valid_coloring(&graph);
    println!("Valid Chromatic Coloring: {:?}", coloring);

    Ok(map)
}

// A Python module implemented in Rust.
#[pymodule]
fn jimmy_portion(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bf_chromo_coloring, m)?)?;
    Ok(())
}

fn find_max_clique_num(graph: &UnGraphMap<usize, usize>) -> usize {
    let mut max_clique: Vec<usize> = Vec::new();

    for start_node in graph.nodes() {
        let mut current_clique: Vec<usize> = Vec::new();
        let mut dfs = Dfs::new(graph, start_node);

        while let Some(node) = dfs.next(graph) {
            if current_clique.iter().all(|&other| graph.contains_edge(node, other)) {
                current_clique.push(node);
            }
        }

        if current_clique.len() > max_clique.len() {
            max_clique = current_clique;
        }
    }
    
    max_clique.len()
}

fn find_valid_coloring(graph: &UnGraphMap<usize, usize>) -> Vec<usize> {
    let brooks_number = {
        let brooks = graph.nodes().map(|elem| graph.neighbors(elem).count()).max().unwrap();
        if graph.nodes().map(|elem| graph.neighbors(elem).count() == 2).filter(|item| !item).count() == 0 && graph.node_count() % 2 == 1 {
            brooks + 1
        } else if graph.nodes().map(|elem| graph.neighbors(elem).count() == graph.node_count() - 1).filter(|item| !item).count() == 0 {
            brooks + 1
        } else {
            brooks
        }
    };

    let max_clique = find_max_clique_num(graph);

    println!("Brooks: {brooks_number}\nMax Clique: {max_clique}");

    for i in max_clique..=brooks_number {
        let colorable = is_graph_x_colorable(i, graph);
        if colorable != vec![] { return colorable; }
    }
    vec![]
}

fn is_graph_x_colorable(x: usize, graph: &UnGraphMap<usize, usize>) -> Vec<usize> {
    let nodes = graph.node_count();

    let product: CartesianProduct = CartesianProduct::new(x, nodes);

    for coloring in product {
        if check_if_valid_coloring(coloring.clone(), &graph) {
            return coloring;
        }
    }

    vec![]
}

fn check_if_valid_coloring(coloring: Vec<usize>, graph: &UnGraphMap<usize, usize>) -> bool {
    let max = coloring.clone().into_iter().max().unwrap();
    let mut color_index_vec: Vec<(usize, Vec<usize>)> = vec![];

    for i in 0..=max {
        let mut index_vec: Vec<usize> = vec![];
        for (index, color) in coloring.clone().into_iter().enumerate() {
            if color == i {
                index_vec.push(index as usize);
            }
        }
        color_index_vec.push((i, index_vec));
    }

    for (_, vec) in color_index_vec {
        if vec.len() >= 2 {
            for c in vec.combination(2) {
                if let [vert_1, vert_2] = &c[..] {
                    if graph.neighbors(**vert_1).collect_vec().contains(vert_2) {
                        return false;
                    }
                }
            }
        }
    }

    true
}
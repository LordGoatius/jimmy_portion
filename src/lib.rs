use std::{collections::{BTreeMap, HashSet}, time::Instant};
use itertools::{Itertools, iproduct};
use permutator::Combination;
use petgraph::prelude::*;
use pyo3::prelude::*;
use rand::Rng;

use crate::prod::CartesianProduct;

pub mod prod;

/// Taken in a list of nodes and edges and prints a minimum coloring and returns a dict
/// representing the coloring
#[pyfunction]
fn bf_chromo_coloring(nodes: Vec<usize>, edges: Vec<(usize, usize)>) -> PyResult<BTreeMap<usize, usize>> {
    let mut graph: UnGraphMap<usize, usize> = GraphMap::default();
    for node in nodes {
        graph.add_node(node);
    }

    for (i, j) in edges {
        graph.add_edge(i, j, 1);
    }

    let coloring = find_valid_coloring(&graph);

    let mut map: BTreeMap<usize, usize> = BTreeMap::default();

    for (i, color) in coloring.iter().enumerate() {
        map.insert(i as usize, *color);
    }

    Ok(map)
}

/// Taken in a list of nodes and edges and prints a greedy coloring and returns a dict
/// representing the coloring
#[pyfunction]
fn greedy_coloring(nodes: Vec<usize>, edges: Vec<(usize, usize)>) -> PyResult<BTreeMap<usize, usize>> {
    let mut graph: UnGraphMap<usize, usize> = GraphMap::default();
    for node in nodes {
        graph.add_node(node);
    }

    for (i, j) in edges {
        graph.add_edge(i, j, 1);
    }

    let greedy_coloring = find_greedy_coloring(&graph);

    Ok(greedy_coloring)
}

/// Taken in a list of nodes and edges and prints a Recursive Largest First coloring and returns a dict
/// representing the coloring
#[pyfunction]
fn recursive_largest_first(nodes: Vec<usize>, edges: Vec<(usize, usize)>) -> PyResult<BTreeMap<usize, usize>> {
    let mut graph: UnGraphMap<usize, usize> = GraphMap::default();
    for node in nodes {
        graph.add_node(node);
    }

    for (i, j) in edges {
        graph.add_edge(i, j, 1);
    }

    let greedy_coloring = find_rlf(&graph);

    Ok(greedy_coloring)
}

/// A test of greedy, RLF, and Brute Force runtime, iterating over the range of the cartestian
/// product of the number of nodes to test and the number of edges to test
/// Returns a List of tuples, where each tuple is (algo, nodes, edges, runtime)
/// (0 represents brute force, 1 represents greedy, 2 represents RLF)
#[pyfunction]
fn test(nodes: usize, edges: usize) -> Vec<(usize, usize, usize, f64)> {
    let mut tests: Vec<(usize, usize, usize, f64)> = Vec::new();

    for (node_amount, edge_amount) in iproduct!(1..=nodes, 1..=edges) {
        if !(edge_amount > (node_amount)*(node_amount - 1) / 2) {
            let mut graph: UnGraphMap<usize, usize> = UnGraphMap::default();

            for i in 0..node_amount {
                graph.add_node(i);
            }

            for _ in 0..edge_amount {
                add_random_edge(&mut graph);
            }

            let node_vec = graph.nodes().collect_vec();
            let edge_vec = graph.all_edges().map(|(edge_1, edge_2, _)| (edge_1, edge_2)).collect_vec();

            let now_bf = Instant::now();
            let _ = bf_chromo_coloring(node_vec, edge_vec);
            let time_bf = now_bf.elapsed();


            let node_vec = graph.nodes().collect_vec();
            let edge_vec = graph.all_edges().map(|(edge_1, edge_2, _)| (edge_1, edge_2)).collect_vec();

            let now_greedy = Instant::now();
            let _ = greedy_coloring(node_vec, edge_vec);
            let time_greedy = now_greedy.elapsed();


            let node_vec = graph.nodes().collect_vec();
            let edge_vec = graph.all_edges().map(|(edge_1, edge_2, _)| (edge_1, edge_2)).collect_vec();

            let now_rlf = Instant::now();
            let _ = recursive_largest_first(node_vec, edge_vec);
            let time_rlf = now_rlf.elapsed();


            tests.push((0, node_amount, edge_amount, time_bf.as_secs_f64()));
            tests.push((1, node_amount, edge_amount, time_greedy.as_secs_f64()));
            tests.push((2, node_amount, edge_amount, time_rlf.as_secs_f64()));
        }
    }
    tests
}

fn add_random_edge(graph: &mut UnGraphMap<usize, usize>) {
    let mut rng = rand::thread_rng();

    // Generate random node indices
    let node_count = graph.node_count();
    let source_index = rng.gen_range(0..node_count);
    let target_index = rng.gen_range(0..node_count);

    if !graph.contains_edge(source_index, target_index) {
        graph.add_edge(source_index, target_index, 1);
    } else {
        add_random_edge(graph);
    }
}

// A Python module implemented in Rust.
#[pymodule]
fn jimmy_portion(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bf_chromo_coloring, m)?)?;
    m.add_function(wrap_pyfunction!(greedy_coloring, m)?)?;
    m.add_function(wrap_pyfunction!(recursive_largest_first, m)?)?;
    m.add_function(wrap_pyfunction!(test, m)?)?;
    Ok(())
}

fn find_rlf(graph: &UnGraphMap<usize, usize>) -> BTreeMap<usize, usize> {
    let mut color: usize = 0;
    let mut coloring: BTreeMap<usize, usize> = BTreeMap::new();
    let mut cloned = graph.clone();

    while cloned.node_count() != 0 {
        let mis = find_minimal_independent_set(&cloned);
        for node in mis {
            coloring.insert(node, color);
            cloned.remove_node(node);
        }
        color += 1;
    }

    coloring
}

fn find_minimal_independent_set(graph: &UnGraphMap<usize, usize>) -> HashSet<usize> {
    let mut mis: HashSet<usize> = HashSet::new();
    let mut avail_nodes: Vec<usize> = graph.nodes().collect();

    while !avail_nodes.is_empty() {
        mis.insert(avail_nodes[0]);

        for neighbor in graph.neighbors(avail_nodes[0]) {
            if avail_nodes.contains(&neighbor) {
                avail_nodes.retain(|&x| x != neighbor);
            }
        }

        if !avail_nodes.is_empty() {
            avail_nodes.remove(0);
        }     
    }

    mis
}

fn find_greedy_coloring(graph: &UnGraphMap<usize, usize>) -> BTreeMap<usize, usize> {
    let mut order: Vec<(usize, usize)> = graph.nodes().enumerate().map(|(i, j)| (i, graph.neighbors(j).count()) ).collect_vec();
    order.sort_by(|(_, order), (_, order_1)| order_1.cmp(order));
    let order = order.iter().map(|(node, _)| node);

    let mut coloring: BTreeMap<usize, usize> = BTreeMap::new();
    let mut color: usize = 0;
    let mut colors: Vec<usize> = Vec::new();

    for i in order {
        if color == 0 {
            coloring.insert(*i, color);
            colors.push(color);
            color += 1;
        } else {
            let mut insert_max = true;
            let invalid_colors = graph.neighbors(*i).map(|node| if coloring.contains_key(&node) { Some(coloring[&node]) } else { None }).flatten().collect_vec();
            for j in 0..color {
                if !invalid_colors.contains(&j) {
                    coloring.insert(*i, j);
                    insert_max = false
                }
            }
            if insert_max {
                coloring.insert(*i, color);
                color += 1;
            }
        }
    }

    coloring
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

    //println!("Brooks: {brooks_number}\nMax Clique: {max_clique}");

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

//! Contains functions for computing SCCs and serializing graphs.
use std::io::{stderr, Read, Write, BufRead, BufReader, BufWriter};
use petgraph::{Graph};
use petgraph::algo::scc;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

fn load_graph<R>(source: &mut BufReader<R>) -> (Graph<u64, ()>, HashMap<u64, NodeIndex>) where R: Read {
    // read graph size
    let mut size = String::new();
    source.read_line(&mut size).ok().expect("failed to read graph size");

    let sz = size.trim().parse::<i64>().unwrap();
    writeln!(&mut stderr(), "Graph size is {} edges. Loading...", sz).unwrap();

    let mut graph = Graph::<u64, ()>::new();
    let mut nodes = HashMap::new();

    for line in source.lines() {
        let l = line.unwrap();
        let (s, e, o) = scan_fmt!(&l, "{} {} {}", u64, u64, u8);
        let (start, end, oneway) = (s.unwrap(), e.unwrap(), o.unwrap());

        if start == end {
            continue
        }

        // adding a node erases the index, so for each unique index we need to keep
        // a map of the corresponding node <-> internal node id mappings. shitty,
        // but it seems to work.
        nodes.entry(start).or_insert_with(|| { graph.add_node(0) });
        nodes.entry(end).or_insert_with(|| { graph.add_node(0) });

        let si = nodes.get(&start).unwrap();
        let ei = nodes.get(&end).unwrap();

        graph.update_edge(*si, *ei, ());

        if oneway != 1 {
            graph.update_edge(*ei, *si, ());
        }
    }

    (graph, nodes)
}

/// Computes the strongly connected components. Takes a graph stream as an
/// input where the format is as follows:
/// 
/// ```ignore
/// num_edges
/// id1 id2 oneway
/// id1 id2 oneway
/// ...
/// id1 id2 oneway
/// ```
///
/// where `id1` is the start of the edge, `id2` is the end of the edge,
/// and `oneway` is **0** if the edge is bidirectional (id2->id1 is permitted)
/// or **1** if `id2` -> `id1` is not allowed. `num_edges` is the number of edges in the
/// file. **output** is a function that can consume the component vectors and the index
/// of the current component vector.
pub fn compute<R, F>(source: &mut BufReader<R>, mut output: F) where R: Read, F: FnMut(Vec<u64>, usize) {
    let (graph, nodes) = load_graph(source);
    writeln!(&mut stderr(), "Graph loaded. Computing SCCs.").unwrap();
    let mut comps: Vec<Vec<_>> = scc(&graph);
    writeln!(&mut stderr(), "Calculated {} components.", comps.len()).unwrap();

    // sort by size, and reverse
    comps.sort_by(|v, w| v.len().cmp(&w.len()));
    comps.reverse();
    // find out the real nodes
    let inverse_nodes: HashMap<_,_> = nodes.into_iter().map(|(k, v)| (v, k)).collect();
    for (i, comp) in comps.iter().enumerate() {
        let real_nodes = comp.into_iter().map(|n| *inverse_nodes.get(&n).unwrap()).collect();
        output(real_nodes, i);
    }
}

/// Serializes the contents of the component into a stream where each node is
/// separated by `delim`. Panics if target cannot be opened.
pub fn serialize<W>(comp: Vec<u64>, target: &mut BufWriter<W>, delim: u8) where W: Write {
    for node in comp {
        target.write(node.to_string().as_bytes()).unwrap();
        target.write(&[delim]).unwrap();
    }
}

mod graph;
mod inv_girth;
mod inv_tree_width;
mod inv_proxi_remote;
mod inv_variance;

use std::any::type_name;
use std::time::Instant;
use crate::graph::UGraph;
use inv_girth::girth;
use inv_tree_width::tree_width;
use inv_variance::var_degree;
use inv_proxi_remote::{Options,minmax_mean_distance};
use std::fs::File;
use std::io::{self, BufRead,BufReader};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()>{

    let g = UGraph::new_n_graph(5,vec![(0,1),(1,2),(2,3),(3,4)]);
    println!("{}",tree_width(g));

    //
    // let file_path = "../g6Files/small.g6";
    // let input = File::open(file_path)?;
    // let buffered = BufReader::new(input);
    // let count = buffered.lines().count() as u128;
    //
    // if let Ok(lines) = read_lines(file_path) {
    //     let now = Instant::now();
    //     for line in lines.map_while(Result::ok) {
    //
    //         let graph = UGraph::graph_from_g6((&line[1..]).to_string());
    //         let var = minmax_mean_distance(graph,Options::Remoteness);
    //         // let var = girth(graph);
    //     }
    //     let end = now.elapsed().as_nanos();
    //     println!("{}",end);
    //     println!("{}",end/count);
    // }
    Ok(())
    
}

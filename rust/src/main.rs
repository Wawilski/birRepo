mod graph;
mod inv_girth;
mod inv_tree_width;
mod inv_proxi_remote;

use std::any::type_name;
use std::time::Instant;
use crate::graph::UGraph;
use inv_girth::girth;
use inv_tree_width::tree_width;
use inv_proxi_remote::{mean_distances,minmax_mean_distance};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {


    //
    // let g = UGraph::new_n_graph(10,vec![(0, 1), (0, 4), (0, 5), (1, 2), (1, 6), (2, 3), (2, 7), (3, 4), (3, 8), (4, 9), (5, 7), (5, 8), (6, 8), (6, 9), (7, 9)]);
    // println!("{:?}",g);
    // println!("{}",minmax_mean_distance(g,0));
    // println!("{}",tree_width(g));

    let mut mean = 0;
    let mut end = 0;
    if let Ok(lines) = read_lines("../../graphs2to10.g6") {
        let now = Instant::now();
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {

            // let now = Instant::now();
            let graph = UGraph::graph_from_g6((&line[1..]).to_string());
            let n = graph.number_of_nodes();
            let minmax = minmax_mean_distance(graph,1);
            if minmax == (n+1)as f32{
                println!("{line},inf");
            } else {
                println!("{line},{:.2}",minmax);
            }
            // println!("{line},{}",tree_width);
        }
        end = now.elapsed().as_nanos();
    }
    
}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

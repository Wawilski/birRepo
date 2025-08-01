mod graph;
mod inv_girth;
mod inv_tree_width;

use std::any::type_name;
use crate::graph::UGraph;
use inv_girth::girth;
use inv_tree_width::tree_width;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let g = UGraph::new_n_graph(7,vec![(0, 2), (0, 1), (0, 3), (1, 4), (2, 5),(3,6)]);
    println!("{:?}",g);
    println!("{}",tree_width(g));

    // File hosts.txt must exist in the current path
    // if let Ok(lines) = read_lines("../../g6Files/small.g6") {
    //     // Consumes the iterator, returns an (Optional) String
    //     for line in lines.map_while(Result::ok) {
    //             if girth == None{
    //                 println!("{line},inf");
    //             }
    //             else{
    //                 println!("{line},{}",girth.unwrap());
    //             }
    //
    //         }
    //     }
    }


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

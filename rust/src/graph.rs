use petgraph::graph6;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct UGraph {
    pub nodes: Vec<i32>,
    pub edges:Vec<(i32,i32)>,
    pub neighbors:HashMap<i32,Vec<i32>>
}


impl UGraph{
    pub fn new() -> Self{
        let nodes:Vec<i32> = vec![];
        let edges:Vec<(i32,i32)> = vec![];
        let mut neighbors:HashMap<i32,Vec<i32>>=HashMap::new();
        Self{
            nodes,
            edges,
            neighbors,
            
        }
    }
    pub fn new_n_graph(n:i32, edges:Vec<(i32,i32)>) -> Self{
        let nodes = (0..n).collect();
        let mut neighbors:HashMap<i32,Vec<i32>>=HashMap::new();
        for i in 0..n{
            neighbors.insert(i, find_neighbors(i,&edges));
        } 
        Self{
            nodes,
            edges,
            neighbors,
        }
    }
    
    pub fn graph_from_g6(g6:String) -> Self{
        let (size,edges) = graph6::from_graph6_representation::<u32>(g6);
        let i32_edges = edges.into_iter().map(|x| {convert(x)}).collect();
        Self::new_n_graph(size.try_into().unwrap(),i32_edges)

    }


    pub fn number_of_nodes(&self)-> i32 {
        self.nodes.len().try_into().unwrap()
    }

    pub fn number_of_edges(&self)->i32{
        self.edges.len().try_into().unwrap()
    }
    
    pub fn degree(&self,node:i32)-> i32{
        self.neighbors.get(&node).unwrap().len().try_into().unwrap()
    }

    pub fn add_nodes<T: IntoIterator<Item=i32>>(&mut self, nodes: T){
        for i in nodes{
            self.nodes.push(i);}
    }
    
    pub fn add_edge(&mut self, edge:(i32,i32)){
        self.edges.push(edge);
    }
    pub fn add_edges(&mut self, edges:Vec<(i32,i32)>){
        for i in edges{
            self.edges.push(i);}
    }

    pub fn remove_node(&mut self,node:i32){
        let mut nodes = vec![];
        
        for i in 0..self.nodes.len(){
            if self.nodes[i] != node{
                nodes.push(i.try_into().unwrap());
            } 
        }
        self.nodes = nodes;
        let mut edges = vec![];
        
        for i in 0..self.edges.len(){
            if self.edges[i].0 != node && self.edges[i].1 != node {
                edges.push(self.edges[i]);
            }
        }
        self.edges = edges;
    }
    pub fn min_degree(&self) -> (i32,i32) {
        let mut min_degree = -1;
        let mut min_node = -1;
        for node in self.nodes.clone(){
            let deg = self.degree(node);
            if deg < min_degree || min_degree == -1 {
                min_degree = deg;
                min_node = node;
            }
        }
        (min_degree,min_node)
    }
}



fn convert(x: (u32,u32)) -> (i32,i32){
    (x.0 as i32,x.1 as i32 )
}

pub fn find_neighbors(node:i32,edges:&Vec<(i32,i32)>)->Vec<i32>{
    let mut neighbors:Vec<i32> = vec![];
    for item in edges.clone(){
        if item.0 == node {
            neighbors.push(item.1);
        }
        if item.1 == node {
            neighbors.push(item.0);
        }
    }
    neighbors
}

use petgraph::graph6;
use std::collections::HashMap;
use crate::utils::tu32_to_ti32;

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
        let neighbors:HashMap<i32,Vec<i32>>=HashMap::new();
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

    pub fn new_set_graph(nodes:Vec<i32>, edges:Vec<(i32,i32)>) -> Self{
        let n = nodes.len() as i32;
        let mut neighbors:HashMap<i32,Vec<i32>>=HashMap::new();
        for i in 0..n{
            neighbors.insert(nodes[i as usize], find_neighbors(nodes[i as usize],&edges));
        } 
        Self{
            nodes,
            edges,
            neighbors,
        }
    }
    
    pub fn graph_from_g6(g6:String) -> Self{
        let (size,edges) = graph6::from_graph6_representation::<u32>(g6);
        let i32_edges = edges.into_iter().map(|x| {tu32_to_ti32(x)}).collect();
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
            self.nodes.push(i);
            self.neighbors.insert(i,vec![]);
        }
    }
    
    pub fn add_edge(&mut self, edge:(i32,i32)){
        self.edges.push(edge);
        self.neighbors.get_mut(&edge.0).expect("REASON").push(edge.1);
        self.neighbors.get_mut(&edge.1).expect("REASON").push(edge.0);

    }
    pub fn add_edges(&mut self, edges:Vec<(i32,i32)>){
        for i in edges{
            self.edges.push(i);
            self.neighbors.get_mut(&i.0).expect("REASON").push(i.1);
            self.neighbors.get_mut(&i.1).expect("REASON").push(i.0);}
    }

    pub fn copy_without(&self,nodes:&Vec<i32>) -> Self{
        let mut new_edges = vec![];
        let mut new_nodes = vec![];
        for node in &self.nodes{
            if !nodes.contains(&node){
                new_nodes.push(*node);
            }
        }
        for edge in &self.edges{ 
            if !nodes.contains(&edge.0) && !nodes.contains(&edge.1){
                new_edges.push(*edge);
            }
        }
        Self::new_set_graph(new_nodes,new_edges)
    }

    pub fn remove_node(&mut self,node:i32){
        let mut nodes = vec![];
        let mut edges = vec![];
        
        for i in 0..self.nodes.len(){
            if self.nodes[i] != node{
                nodes.push(self.nodes[i].try_into().unwrap());
                if self.neighbors.get(&self.nodes[i]).unwrap().contains(&node){
                    self.neighbors.get_mut(&self.nodes[i]).expect("REASON").retain(|value| *value != node);
                }
            } 
            else{
                self.neighbors.remove_entry(&self.nodes[i]);
            }
        }
        
        for i in 0..self.edges.len(){
            if self.edges[i].0 != node && self.edges[i].1 != node {
                edges.push(self.edges[i]);
            }
        }

        self.nodes = nodes;
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

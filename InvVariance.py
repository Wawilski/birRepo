import networkx as nx
import matplotlib.pyplot as plt

def mean_degree(G):
    if G.number_of_nodes() == 1:
        return 0
    count = 0
    for node in G:
        count += G.degree(node)
    return count/G.number_of_nodes()

def var_degree(G):
    if G.number_of_nodes() == 1:
        return 0
    mean = mean_degree(G)
    count = 0
    for node in G:
        count += (G.degree(node) - mean)**2
    return count/(G.number_of_nodes()-1)

if __name__ == "__main__": 
    with open("g6Files/graphs2to10.g6",'r') as file:
        f = file.readlines()
        for line in f:
            b = bytes(line[1:],"utf-8")
            H = nx.from_graph6_bytes(b) 

            print(f"{line[1:-1]},{round(var_degree(H), 2)}")
    
    

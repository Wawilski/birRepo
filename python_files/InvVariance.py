import networkx as nx
import matplotlib.pyplot as plt
import time

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
    
    

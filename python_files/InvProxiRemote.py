import networkx as nx
import math
import matplotlib.pyplot as plt
import time

def mean_distances(G,node):
    n = G.number_of_nodes()
    if n == 1 or n == 0:
        return float("inf")
    visited = [node]
    queue = [node]
    distances=[0] * n
    while len(queue)>0:
        current = queue.pop(0)
        for neighbor in G.neighbors(current):
            if neighbor not in visited:
                queue.append(neighbor)
                visited.append(neighbor)
                distances[neighbor] = distances[current] + 1 
    if len(visited) < n:
        return float("inf")
    
    mean = sum(distances)/(n-1)
    return mean



def minmax_mean_distance(G, option="proximity"):
    if option == "proximity":
        minmax_dist = float("inf")
    if option == "remoteness":
        minmax_dist = 0
    
    n = G.number_of_nodes()
    for node in G.nodes():

        mean = mean_distances(G,node)
        if mean > minmax_dist and option=="remoteness":
            minmax_dist = mean
        if mean < minmax_dist and option=="proximity":
            minmax_dist = mean
        
    return minmax_dist



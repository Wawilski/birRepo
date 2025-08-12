import networkx as nx
import matplotlib.pyplot as plt
import time

def girth(G):
    girth = float('inf')
    n = G.number_of_nodes()

    for node in G.nodes():
        
        dist = [float('inf')] * n
        prev = [-1] * n

        dist[node] = 0
        queue = []
        queue.append(node)

        while len(queue) != 0:
            current = queue.pop(0)
            for w in G.neighbors(current):
                if dist[w] == float('inf'):
                    dist[w] = dist[current] + 1
                    prev[w] = current
                    queue.append(w)
                if prev[w] != current and prev[current] != w:
                    girth = min(girth, dist[w] + dist[current] + 1)
    return girth


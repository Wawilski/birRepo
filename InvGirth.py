import networkx as nx
import matplotlib.pyplot as plt
import math

def girth(G):
    girth = math.inf
    n = G.number_of_nodes()

    for node in G.nodes():
        
        dist = [math.inf] * n
        prev = [-1] * n

        dist[node] = 0
        queue = []
        queue.append(node)

        while len(queue) != 0:
            current = queue.pop(0)
            for w in G.neighbors(current):
                if dist[w] == math.inf:
                    dist[w] = dist[current] + 1
                    prev[w] = current
                    queue.append(w)
                if prev[w] != current and prev[current] != w:
                    girth = min(girth, dist[w] + dist[current] + 1)
    return girth


if __name__ == "__main__":
    with open("g6Files/graphs2to10.g6",'r') as file:
        f = file.readlines()
        for line in f:
            b = bytes(line[1:],"utf-8")
            H = nx.from_graph6_bytes(b) 

            print(f"{line[1:-1]},{girth(H)}")

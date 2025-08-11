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


if __name__ == "__main__":
    n =0
    with open("g6Files/small.g6",'r') as file:
        f = file.readlines()
        start = time.clock_gettime_ns(0)
        for line in f:
            b = bytes(line[1:],"utf-8")
            H = nx.from_graph6_bytes(b) 
            girth(H)
            # print(f"{line[1:-1]},{girth(H)}")
            n+=1
        end = time.clock_gettime_ns(0)
    print((end-start)/n)
    print(end-start)

import networkx as nx
import time
from InvGirth import girth
from InvTreeWidth import tree_width
from InvProxiRemote import minmax_mean_distance
from InvVariance import var_degree
import matplotlib.pyplot as plt

if __name__ == "__main__":
    b = bytes("F?AZO","utf-8")
    # b = bytes("G??it{","utf-8")  
    # b = bytes("IAHQ\\}}}w","utf-8")
    G = nx.from_graph6_bytes(b) 
    options = {
        "font_size": 36,
        "node_size": 3000,
        "node_color": "white",
        "edgecolors": "black",
        "linewidths": 5,
        "width": 5,
    }
    nx.draw_networkx(G,  **options)
    print(G.edges)

# Set margins for the axes so that nodes aren't clipped
    ax = plt.gca()
    ax.margins(0.20)
    plt.axis("off")
    plt.show()
    # with open("../g6Files/small.g6",'r') as file:
    #     f = file.readlines()
    #     n = len(f)
    #
    #     start = time.clock_gettime_ns(0)
    #     for line in f:
    #         b = bytes(line[1:],"utf-8")
    #         G = nx.from_graph6_bytes(b) 
    #         #Change function here
    #         to_print = minmax_mean_distance(G,'proximity')
    #
    #         # print(f"{line[1:-1]},{round(to_print), 2)}")
    #     end = time.clock_gettime_ns(0)
    # print((end-start)/n)
    # print(end-start)

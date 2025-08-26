import networkx as nx
import matplotlib.pyplot as plt
from InvTreeWidth import tree_width
from InvGirth import girth
from InvProxiRemote import minmax_mean_distance
from InvVariance import var_degree
# G =nx.Graph()
# G.add_nodes_from([0, 1, 2, 3, 4, 5, 6])
# G.add_edges_from([(0, 2), (0, 1), (0, 3), (1, 4), (2, 5),(3,6)]) 
# G = nx.from_graph6_bytes(b"F_l~w")
#G = nx.from_graph6_bytes(b"ES\o")

# G = nx.petersen_graph()

G = nx.from_graph6_bytes(b"ILCHg||~w")

# G = nx.Graph()
# G.add_nodes_from([0,1,2,3])
# G.add_edges_from([(0,1),(1,2),(2,3)])

# G = nx.cycle_graph(5)

print(f"nodes: {G.number_of_nodes()}")
print(f"edges: {G.number_of_edges()}")

print("------------------") 
print(f"{G.nodes()}")
print(f"{G.edges()}")
print(f"degrees G : {[G.degree(x) for x in G.nodes()]}")
print(f"proximity G: {minmax_mean_distance(G,'proximity')}")
print(f"remoteness G: {minmax_mean_distance(G,'remoteness')}")
print(f"tree_width G: {tree_width(G)}")
print(f"var G: {var_degree(G)}")
print(f"girth G: {girth(G)}")

print("--------------------")

options = {
    "font_size": 36,
    "node_size": 3000,
    "node_color": "blue",
    "edgecolors": "black",
    "linewidths": 5,
    "width": 5,
}
nx.draw(G,**options)
ax = plt.gca()
ax.margins(0.20)
plt.axis("off")
plt.show()


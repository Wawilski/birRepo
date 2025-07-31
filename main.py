import networkx as nx
import matplotlib.pyplot as plt
from utils import mean_degree, var_degree, girth
G =nx.Graph()
G.add_nodes_from([0, 1, 2, 3, 4, 5])
G.add_edges_from([(0, 2), (0, 1), (1, 5), (2, 4), (2, 5)]) 
# H = nx.from_graph6_bytes(b"I?CWw{^Fw") 
print(f"nodes: {G.number_of_nodes()}")
print(f"edges: {G.number_of_edges()}")

print("------------------") 
print(f"{G.nodes()}")
print(f"{G.edges()}")
print(f"mean_degree G: {mean_degree(G)}")
print(f"variance_degree G: {var_degree(G)}")
print(f"girth G: {girth(G)}")
print("--------------------")
# print(f"nodes: {H.number_of_nodes()}")
# print(f"edges: {H.number_of_edges()}")
# print(f"{H.nodes()}")
# print(f"{H.edges()}")
# print(f"mean_degree H: {mean_degree(H)}")
# print(f"variance_degree H : {var_degree(H)}")
# print("-------------------")
# print(nx.is_isomorphic(G,H))

nx.draw(G)
ax = plt.gca()
ax.margins(0.20)
plt.axis("off")
plt.show()


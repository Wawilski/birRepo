import networkx as nx
import matplotlib.pyplot as plt


def next(arr):
    pivot = len(arr)-1
    tmp = 0
    
    while (pivot > 0 and (arr[pivot] < arr[pivot-1])):
        pivot-=1

    if( pivot > 0 ):
        pivot-=1
    else:
        return False
    
    j = len(arr) - 1
    while (j>pivot):
        if (arr[j] > arr[pivot]):
            arr[j],arr[pivot] = arr[pivot],arr[j]
            break
        j-=1

    first = pivot+1
    last = len(arr)-1    
    j = 0
    while j < (first + last)/2 - first:
        arr[first+j],arr[last-j]= arr[last-j],arr[first+j]
        j+=1

    return True

#Naive method (to upgrade)
# Work but very very slow for big once
def tree_width(G):
    n = len(list(G.nodes()))
    g_edges = list(G.edges())
    o_perm = list(range(n))
    nMinMaxDeg = n
    while True:
        print(o_perm)
        C = nx.Graph()
        C.add_nodes_from(range(n))
        for i in range(n-1):
            for j in range(i+1, n):
                if ((i,j) in g_edges):
                    C.add_edge(o_perm[i],o_perm[j])
        c_edges = list(C.edges())
        nMaxDeg = 0
        for i in range(n):
            nDeg = 0
            first = 0
            for j in range(i+1, n):
                if ((i,j) in c_edges):
                    nDeg += 1
                    if (first > 0):
                        C.add_edge(first,j)
                        c_edges = list(C.edges())
                    else:
                        first = j
            if (nDeg > nMaxDeg):
                nMaxDeg = nDeg
            if(nMaxDeg >= nMinMaxDeg):
                continue
        if (nMaxDeg < nMinMaxDeg):
            nMinMaxDeg = nMaxDeg
        if (nMinMaxDeg == 1):
            return nMinMaxDeg
        if not next(o_perm):
            break
    return nMinMaxDeg


#TO FIX AND CLEAN ------>


def in_neighbors(node, T , assign, current):
    neighbors = list(T.neighbors(current))
    included = []
    for i in neighbors:
        if node in assign[i]:
            included.append(i)
    return included


def shrinking(G):
    big_set = list(G.nodes())
    order = sorted(big_set, key=G.degree)
    V = []

    T = nx.Graph()
    assign = dict()
    counter = 0
    T.add_node(counter)
    assign[counter] = big_set 
    while len(order) > 0:
        node = order[0]
        counter+=1
        all_near = list(G.neighbors(node))
        new = [node]
        for element in all_near:
            if element in big_set:
                new.append(element)
        if len(new) > 1 and len(new) < len(big_set):
            big_set.remove(node)
            T.add_node(counter)
            assign[counter] = new
            neighbors = in_neighbors(node,T,assign,0)
            if len(neighbors) > 0:
                for neighbor in neighbors:
                    T.add_edge(counter,neighbor)
                    T.remove_edge(neighbor,0)
            T.add_edge(0,counter)
        order= order[1:]

    return T,assign

def max_size(matrix):
    max = len(matrix[0])
    for i in range(1,len(matrix)):
        size_i = len(matrix[i])
        if size_i > max:
            max = size_i
    return max

def width(G):
    T,ref = shrinking(G)
    print(T.nodes())
    print(T.edges())
    print(ref)
    values = list(ref.values())
    return max_size(values) - 1

#< ----- TO FIX AND CLEAN
options = {
    "font_size": 36,

    "node_size": 3000,
    "node_color": "white",
    "edgecolors": "black",
    "linewidths": 5,
    "width": 5,
}

# G = nx.Graph()
# G.add_nodes_from([0,1,2,3,4])
# G.add_edges_from([(0,1),(1,2),(1,3),(2,3),(3,4)])

# G = nx.Graph()
# G.add_nodes_from([0,1,2,3])
# G.add_edges_from([(0,1),(1,2),(2,3),(0,3)])

#G =nx.Graph()
#G.add_nodes_from([0, 1, 2, 3, 4, 5,6,7])
#G.add_edges_from([(0, 1), (0, 2), (1, 5),(1,2), (1,4), (1,6), (2,3), (2, 4), (3, 4),(4,6),(4,7),(5,6),(6,7)]) 

# G = nx.Graph()
# G.add_nodes_from([0,1,2,3,4,5,6])
# G.add_edges_from([(0,1),(0,2),(0,3),(1,5),(2,6),(3,4)])

G = nx.petersen_graph()
# G = nx.from_graph6_bytes(b"I?CWw{^Fw") 

print("------------------") 
print(f"{G.nodes()}")
print(f"{G.edges()}")
print(f"{tree_width(G)}")
print("------------------") 

nx.draw_networkx(G,**options)
ax = plt.gca()
ax.margins(0.20)
plt.axis("off")
plt.show()

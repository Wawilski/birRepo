import networkx as nx
import matplotlib.pyplot as plt


#Based on S. Langermann algorithm ----------->
#Next permutation of an array
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


def minDegree(G):
    min_degree = float('inf')
    min_node = -1
    for node in G.nodes():
        deg = G.degree(node)
        if  deg < min_degree:
            min_degree = deg
            min_node = node
    return min_degree,min_node

def MMD(G):
    H = nx.Graph()
    H.add_nodes_from(G.nodes())
    H.add_edges_from(G.edges())
    maxmin = 0
    while (H.number_of_edges()>=2):
        deg,node = minDegree(H)
        maxmin = max(maxmin,deg)
        H = nx.restricted_view(H,[node],[])
    return maxmin
    
#Naive method (to upgrade)
# Work but very very slow for big ones
def tree_width(G):
    counter = 0
    n = len(list(G.nodes()))
    g_edges = list(G.edges())
    o_perm = list(range(n))
    nMinMaxDeg = n
    lowerBound = MMD(G)
    while True:
        counter += 1
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
                    if(nDeg >= nMinMaxDeg):
                        break
                    if (first > 0):
                        C.add_edge(first,j)
                        c_edges = list(C.edges())
                    else:
                        first = j
            if (nDeg > nMaxDeg):
                nMaxDeg = nDeg
            if(nDeg >= nMinMaxDeg):
                break

        if (nMaxDeg < nMinMaxDeg):
            nMinMaxDeg = nMaxDeg
        if (nMinMaxDeg <= lowerBound or nMinMaxDeg == 1):
            return nMinMaxDeg
        if not next(o_perm):
            break
    return nMinMaxDeg

# <-----------

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
            if len(neighbors) > 1:
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

if __name__ == "__main__":
    with open("g6Files/middle.g6",'r') as file:
        f = file.readlines()
        for line in f:
            b = bytes(line[1:],"utf-8")
            G = nx.from_graph6_bytes(b) 

            print(f"{line[1:-1]},{tree_width(G)}")

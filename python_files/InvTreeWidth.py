import networkx as nx
import time
import matplotlib.pyplot as plt


#Based on S. Langermann algorithm ----------->
#Next permutation of an array
def next(arr):
    pivot = len(arr)-1
    
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
            min_degree,min_node = deg, node
    return min_degree,min_node

def MMD(G):
    H = G.copy()
    maxmin = 0
    while (H.number_of_edges()>=2):
        deg,node = minDegree(H)
        maxmin = max(maxmin,deg)
        H = nx.restricted_view(H,[node],[])
    return maxmin
    
#Naive method (to upgrade)
# Work but very very slow for big ones
def tree_width(G):
    n = G.number_of_nodes()
    g_edges = list(G.edges())
    o_perm = list(range(n))
    nMinMaxDeg = n
    lowerBound = MMD(G)
    while True:
        C = nx.Graph()
        C.add_nodes_from(range(n))
        for i in range(n-1):
            for j in range(i+1, n):
                if ((i,j) in g_edges):
                    C.add_edge(o_perm[i],o_perm[j])
        c_edges = list(C.edges())
        nMaxDeg = 0

        for i in range(n):
            nDeg, first = 0, 0
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

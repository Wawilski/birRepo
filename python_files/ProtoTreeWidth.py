import networkx as nx

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


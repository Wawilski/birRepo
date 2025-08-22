import networkx as nx
import time
from InvGirth import girth
from InvTreeWidth import tree_width
from InvProxiRemote import minmax_mean_distance
from InvVariance import var_degree
import matplotlib.pyplot as plt

if __name__ == "__main__":
    with open("../g6Files/eight.g6",'r') as file:
    # with open("../../ten.g6",'r') as file:
        f = file.readlines()
        n = len(f)

        start = time.clock_gettime_ns(0)
        for line in f:
            b = bytes(line[1:],"utf-8")
            G = nx.from_graph6_bytes(b) 
            #Change function here
            # to_print = minmax_mean_distance(G,'proximity')
            # var_degree(G)
            # minmax_mean_distance(G,'proximity')
            # girth(G)
            tree_width(G)


            # print(f"{line[1:-1]},{round(to_print), 2)}")
        end = time.clock_gettime_ns(0)
    print((end-start)/n)
    print(end-start)

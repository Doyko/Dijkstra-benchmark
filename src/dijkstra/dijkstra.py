#!/usr/bin/env python3.8

import sys
from time import perf_counter_ns

if(len(sys.argv) != 3):

    sys.stderr.write("2 arguments expected : fileName and iterations\n")
    exit(1)

nb_iter = int(sys.argv[2])

# read file
topo = open(sys.argv[1], "r").read().split("\n")
nb_node = int(topo.pop(0))

for i in range(0, nb_node):
    lign = topo[i].split(" ")
    topo[i] = list(map(int, lign))

# target node
id_node = 0

start = perf_counter_ns()

for iter in range(0, nb_iter):
    nodes_done = []
    nodes_left = []

    for i in range(0, nb_node): # initialisation
        node = dict()

        if(i == id_node): # target
            node["id"] = i
            node["path"] = str(id_node)
            node["length"] = 0
            nodes_done.append(node)
        else:
            node["id"] = i
            if(topo[id_node][i] != 0): # if linked
                node["length"] = topo[id_node][i]
                node["path"] = str(id_node)
            else: # if not linked
                node["length"] = float("inf")

            nodes_left.append(node)

    while nodes_left:

        node_min_pos = 0
        for i in range(1, len(nodes_left)): # looking for the nearest node
            if(nodes_left[node_min_pos]["length"] > nodes_left[i]["length"]):
                node_min_pos = i

        node_min = nodes_left[node_min_pos]
        node_min["path"] += " -> " + str(node_min["id"])

        for n in nodes_left: # update other nodes

            dist = topo[node_min["id"]][n["id"]]

            if(dist != 0):

                if(n["length"] > node_min["length"] + dist):

                    n["length"] = node_min["length"] + dist
                    n["path"] = node_min["path"]

        nodes_done.append(nodes_left.pop(node_min_pos))

    # print result
    #for n in nodes_done:
    #    print(n)
end = perf_counter_ns()
print((end - start) // nb_iter, end = "")

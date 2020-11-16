#!/bin/bash
scr_path=`dirname $0`

# cpp
rm ${scr_path}/dijkstra_cpp

# java
rm ${scr_path}/*.class

# topology
rm ${scr_path}/topology.txt

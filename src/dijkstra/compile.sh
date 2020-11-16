#!/bin/bash
scr_path=`dirname $0`

# cpp
g++ -Wall -g -O3 -std=c++17 ${scr_path}/dijkstra.cpp -o ${scr_path}/dijkstra_cpp

# java
javac ${scr_path}/dijkstra.java

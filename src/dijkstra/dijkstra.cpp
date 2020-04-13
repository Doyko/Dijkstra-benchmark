#include <vector>
#include <string>
#include <fstream>
#include <iostream>
#include <limits>
#include <chrono>

// class for nodes
class Node
{

public:

    Node(int id, std::string path, int length);

    int id;
    std::string path;
    int length;
};

// Constructor
Node::Node(int id, std::string path, int length)
:
    id(id),
    path(path),
    length(length)
{}

// Function that read the topology
int** readTopologie(std::string name, int& nbNode)
{
    std::ifstream ifs(name);
    if(!ifs)
    {
        std::cerr << "file " << name << " not found !" << '\n';
        exit(1);
    }

    ifs >> nbNode;
    int** topology = new int*[nbNode];
    for(int i = 0; i < nbNode; i++)
    {
        topology[i] = new int[nbNode];
        for(int j = 0; j < nbNode; j++)
        {
            ifs >> topology[i][j];
        }
    }
    ifs.close();
    return topology;
}

int main(int argc, char* argv[])
{
    if(argc != 3)
    {
        std::cerr << "2 arguments expected : fileName and iterations" << '\n';
        exit(1);
    }
    int nbIter = std::stoi(argv[2]);
    int nbNode;
    int** topology = readTopologie(argv[1], nbNode);

    // target
    int idNode = 0;

    auto start = std::chrono::steady_clock::now();

    for(int iter = 0; iter < nbIter; iter++)
    {
        std::vector<Node*> nodeDone;
        std::vector<Node*> nodeLeft;

        for(int i = 0; i < nbNode; i++)
        {
            if(i == idNode) // target
                nodeDone.push_back(new Node(i, std::to_string(idNode), 0));
            else if(topology[i][idNode] != 0) // if linked
                nodeLeft.push_back(new Node(i, std::to_string(idNode), topology[i][idNode]));
            else // if not linked
                nodeLeft.push_back(new Node(i, std::to_string(idNode), std::numeric_limits<int>::max()));
        }

        while(!nodeLeft.empty())
        {
            int nodeMinPos = 0;
            for(size_t i = 1, max = nodeLeft.size(); i < max; i++) // looking for the nearest node
            {
                if(nodeLeft[nodeMinPos]->length > nodeLeft[i]->length)
                    nodeMinPos = i;
            }

            Node* minNode = nodeLeft[nodeMinPos];
            minNode->path += " -> " + std::to_string(minNode->id);

            for(Node* n : nodeLeft) // update other nodes
            {
                int dist = topology[minNode->id][n->id];
                if(dist != 0)
                {
                    if(n->length > minNode->length + dist)
                    {
                        n->length = minNode->length + dist;
                        n->path = minNode->path;
                    }
                }
            }
            nodeDone.push_back(minNode);
            nodeLeft.erase(nodeLeft.begin() + nodeMinPos);
        }

        // print result
        /*
        for(size_t i = 0; i < nodeDone.size(); i++)
        {
            std::cout << nodeDone[i]->path << " | " << nodeDone[i]->length << " | " << nodeDone[i]->path << '\n';
        }
        */

        // free memory
        for(int i = 0; i < nbNode; i++)
        {
            delete nodeDone[i];
        }
    }

    auto end = std::chrono::steady_clock::now();
    std::cout << std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count() / nbIter;

    for(int i = 0; i < nbNode; i++)
    {
        delete[] topology[i];
    }
    delete[] topology;

    return 0;
}

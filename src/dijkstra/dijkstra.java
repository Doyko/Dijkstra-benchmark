import java.io.*;
import java.util.ArrayList;
import java.util.concurrent.TimeUnit;

public class dijkstra
{
    // class for nodes
    private static class Node
    {
        public final int id;
        public String path;
        public int length;

        // Constructor
        public Node(int id, String path, int length)
        {
            this.id = id;
            this.path = path;
            this.length = length;
        }
    }

    static int nbNode;

    // function that read the topology
    public static int[][] readTopologie(String fileName) throws Exception
    {
        BufferedReader br = new BufferedReader(new FileReader(new File(fileName)));

        String s = br.readLine();
        nbNode = Integer.parseInt(s);

        int[][] topology = new int[nbNode][nbNode];
        for(int i = 0; i < nbNode; i++)
        {
            s = br.readLine();
            if(s == null)
            {
                throw new IOException("Unexpected end of file!");
            }
            String[] array = s.split(" ");
            if(array.length != nbNode)
            {
                throw new IOException("Invalid line: " + s);
            }
            for(int j = 0; j < nbNode; j++)
            {
                topology[i][j] = Integer.parseInt(array[j]);
                if(topology[i][j] < 0)
                {
                    throw new IOException("Invalid distance: " + topology[i][j] + " on line: " + s);
                }
            }
        }

        return topology;
    }

    public static void main(String[] args) throws Exception
    {
        if(args.length != 2)
        {
            throw new Exception("2 arguments expected : fileName and iterations");
        }

        int nbIter = Integer.parseInt(args[1]);

        int[][] topology = readTopologie(args[0]);

        // target
        int idNode = 0;

        long start = System.nanoTime();

        for(int iter = 0; iter < nbIter; iter++)
        {
            ArrayList<Node> nodeDone = new ArrayList<Node>(nbNode);
            ArrayList<Node> nodeLeft = new ArrayList<Node>(nbNode);

            for(int i = 0; i < nbNode; i++)
            {
                if(i == idNode) // target
                    nodeDone.add(new Node(i, Integer.toString(i), 0));
                else if(topology[i][idNode] != 0) // if linked
                    nodeLeft.add(new Node(i, Integer.toString(i), topology[i][idNode]));
                else // if not linked
                    nodeLeft.add(new Node(i, Integer.toString(i), Integer.MAX_VALUE));
            }

            while(!nodeLeft.isEmpty())
            {
                int nodeMinPos = 0;
                for(int i = 1, max = nodeLeft.size(); i < max; i++) // looking for the nearest node
                {
                    if(nodeLeft.get(nodeMinPos).length > nodeLeft.get(i).length)
                        nodeMinPos = i;
                }
                Node minNode = nodeLeft.get(nodeMinPos);
                minNode.path += " -> " + Integer.toString(minNode.id);

                nodeLeft.forEach(n -> // update other nodes
                {
                    int dist = topology[minNode.id][n.id];
                    if(dist != 0)
                    {
                        if(n.length > minNode.length + dist)
                        {
                            n.length = minNode.length + dist;
                            n.path = minNode.path;
                        }
                    }
                });

                // slower
                /*
                nodeLeft.stream().filter(n -> topology[minNode.id][n.id] != 0 && n.length > minNode.length + topology[minNode.id][n.id]).forEach(n -> // update other nodes
                {
                    n.length = minNode.length + topology[minNode.id][n.id];
                    n.path = minNode.path;
                });*/

                nodeDone.add(minNode);
                nodeLeft.remove(nodeMinPos);
            }
            // print result
            //nodeDone.forEach(n -> System.out.println(n.id + " | " + n.length + " | " + n.path));
        }

        long end = System.nanoTime();

        System.out.print((end - start) / nbIter);
    }
}

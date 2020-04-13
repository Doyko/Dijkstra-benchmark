pub mod dijkstra_mod {

    use std::f32::INFINITY;
    use std::fs::File;
    use std::io::Read;
    use std::time::Instant;

    // class for nodes
    #[derive(Debug)]
    struct Node {
        id: u32,
        path: String,
        length: f32,
    }

    impl Node {
        // Constructor
        fn new(id: u32, path: String, length: f32) -> Node {
            Node {
                id: id,
                path: path,
                length: length,
            }
        }
    }

    // Function that read the topology
    fn read_topology(file_name: &str) -> (u32, Vec<Vec<f32>>) {
        let mut file = File::open(file_name).unwrap();
        let mut file_str = String::new();
        file.read_to_string(&mut file_str).unwrap();

        let mut lines: Vec<&str> = file_str.lines().collect();

        let nb_nodes = lines.remove(0).parse::<u32>().unwrap();

        let topology: Vec<Vec<f32>> = lines.iter().map(|line| {line.split_whitespace().map(|c| c.parse::<f32>().unwrap()).collect()}).collect();

        return (nb_nodes, topology);
    }

    pub fn dijkstra(file_name: &str, nb_iter: u32) -> u128 {
        let (nb_nodes, topology) = read_topology(file_name);

        // target
        let target = 0;

        let start = Instant::now();

        for _i in 0..nb_iter {
            let mut nodes_left: Vec<Node> = Vec::with_capacity(nb_nodes as usize);
            let mut nodes_done: Vec<Node> = Vec::with_capacity(nb_nodes as usize);

            for i in 0..nb_nodes {
                if i == target { // target
                    nodes_done.push(Node::new(i, target.to_string(), 0.0));
                }
                else if topology[i as usize][target as usize] != 0.0 { // if linked
                    nodes_left.push(Node::new(i, target.to_string(), topology[i as usize][target as usize]));
                }
                else { // if not linked
                    nodes_left.push(Node::new(i, "".to_string(), INFINITY));
                }
            }

            while !nodes_left.is_empty() {
                let mut nearest_node_pos: usize = 0;
                for i in 1..nodes_left.len() { // looking for the nearest node
                    if nodes_left[nearest_node_pos].length > nodes_left[i].length {
                        nearest_node_pos = i;
                    }
                }

                let mut min_node = nodes_left.remove(nearest_node_pos);
                min_node.path += &format!(" -> {}", min_node.id.to_string());

                for i in 0..nodes_left.len() { // update other nodes
                    let dist = topology[min_node.id as usize][nodes_left[i].id as usize];
                    if dist != 0.0 {
                        if nodes_left[i].length > min_node.length + dist {
                            nodes_left[i].length = min_node.length + dist;
                            nodes_left[i].path = min_node.path.clone();
                        }
                    }
                }

                nodes_done.push(min_node);
            }

            // print result
            /*
            for node in &nodes_done {
                println!("{:?}", node);
            }*/
        }

        return start.elapsed().as_nanos() / nb_iter as u128;
    }
}

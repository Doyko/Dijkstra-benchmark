use rand::Rng;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod dijkstra;
use crate::dijkstra::dijkstra_mod::dijkstra;

#[cfg(test)]
mod test;

// create file with topology
fn create_topology(graph: &Vec<Vec<u32>>) -> std::io::Result<()> {
    let mut file = File::create("src/dijkstra/topology.txt")?;

    let mut string = String::new();
    for line in graph {
        for num in line {
            string.push_str(&format!("{} ", num));
        }
        string.pop();
        string.push_str("\n");
    }
    string.pop();

    file.write(format!("{}\n{}", graph.len(), string).as_bytes())?;
    file.flush()?;
    Ok(())
}

// add nodes to graph
fn add_nodes(graph: &mut Vec<Vec<u32>>, nb_nodes: u32) {

    for _i in 0..nb_nodes {

        let size: usize = graph.len();
        if size == 0 {
            graph.push(Vec::new());
            graph[0].push(0);
            continue;
        }

        let mut rng = rand::thread_rng();

        graph.push(Vec::with_capacity(size + 1 as usize));

        let p = rng.gen_range(0, size);
        for i in 0..size {
            let dist: u32;
            if i == p || rng.gen_range(0, 3) == 0 {
                dist = rng.gen_range(1, 10);
            } else {
                dist = 0;
            }

            graph[i].push(dist);
            graph[size].push(dist);
        }
        graph[size].push(0);
    }
}

// exec command
fn exec(program: &str, args: &[&str]) -> u64 {
    let output = Command::new(program).args(args).output().unwrap();
    assert!(output.status.success());
    return String::from_utf8_lossy(&output.stdout).parse::<u64>().unwrap();
}

const STEP: usize = 5;
const MAX: usize = 501;
const ITER: u32 = 2500;

fn main() -> std::io::Result<()> {
    // compile
    let output = Command::new("src/dijkstra/compile.sh").output()?;
    assert!(output.status.success());

    // graph
    let mut graph: Vec<Vec<u32>> = Vec::new();

    fs::create_dir_all("output")?;
    let mut file = File::create("output/output.csv")?;

    file.write(b"size;Python;Java;C++;Rust\n")?;

    for i in (STEP..MAX).step_by(STEP) {
        println!("{}", i);
        add_nodes(&mut graph, STEP as u32);
        create_topology(&graph)?;
        if i > 151 {
            file.write(format!("{};NaN;{};{};{}\n",
            i,
            exec("java", &["-cp", "src/dijkstra/", "dijkstra", "src/dijkstra/topology.txt", &ITER.to_string()]),
            exec("src/dijkstra/dijkstra_cpp", &["src/dijkstra/topology.txt", &ITER.to_string()]),
            dijkstra("src/dijkstra/topology.txt", ITER)).as_bytes())?;
        }
        else {
            file.write(format!("{};{};{};{};{}\n",
            i,
            exec("src/dijkstra/dijkstra.py", &["src/dijkstra/topology.txt", &ITER.to_string()]),
            exec("java", &["-cp", "src/dijkstra/", "dijkstra", "src/dijkstra/topology.txt", &ITER.to_string()]),
            exec("src/dijkstra/dijkstra_cpp", &["src/dijkstra/topology.txt", &ITER.to_string()]),
            dijkstra("src/dijkstra/topology.txt", ITER)).as_bytes())?;
        }
    }
    file.flush()?;

    // clear
    let output = Command::new("src/dijkstra/clear.sh").output()?;
    assert!(output.status.success());

    Ok(())
}

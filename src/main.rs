use std::{env, cmp};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet, BinaryHeap};
use petgraph::{Graph, Undirected};
use petgraph::graph::{NodeIndex, Node};
use petgraph::visit::IntoNodeReferences;
use std::time::Instant;

fn main() {

    let args: Vec<String> = env::args().collect();
    let path = args.get(1);

    if let Ok(lines) = read_lines("D:\\ghost\\chromatic-upper-bound\\data.csv") {
        let mut total_time = 0;
        for line in lines {
            if let Ok(entry) = line {
                let split : Vec<&str> =  entry.split(',').collect();

                let txt_path = format!("D:\\workspaces\\Lambda\\graphs\\{}.col", &split.get(1).unwrap());
                let path =  Path::new(txt_path.as_str());
                if path.exists() {
                    let graph_opt = read_graph(&txt_path);

                    if let Some(graph) = &graph_opt {
                        let now = Instant::now();
                        let order = superman_sorting(graph);
                        let result = greedy_colouring(&mut graph_opt.unwrap(), &order);
                        println!("{}", result);
                        //println!("Result {} in {}ms", result, now.elapsed().as_millis());
                        total_time += now.elapsed().as_millis();
                    }
                }
            }
        }

        println!("Solved all in {}ms", total_time);
    }

    /*match path {
        Some(x) => {
            let graph_opt = read_graph(x);

            if let Some(graph) = &graph_opt {
                let now = Instant::now();
                let order = superman_sorting(graph);
                let result = greedy_colouring(&mut graph_opt.unwrap(), &order);
                println!("Result {} in {}ms", result, now.elapsed().as_millis());
            }
        },
        None => eprintln!("Please provide a path to a graph file.")
    }*/

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_graph(file_path: &String) -> Option<Graph<i16, u8, Undirected, u32>> {
    if let Ok(lines) = read_lines(file_path) {
        let mut graph = Graph::<i16, u8, Undirected, u32>::new_undirected();
        let mut vertex_ids = HashMap::new();

        for line in lines {
            if let Ok(entry) = line {
                if entry.starts_with("e") {
                    let split : Vec<&str> = entry.split_whitespace().collect();
                    let from: u32 = split.get(1).unwrap().parse().unwrap();
                    let to: u32 = split.get(2).unwrap().parse().unwrap();

                    if !vertex_ids.contains_key(&from) {
                        let node = &graph.add_node(-1);
                        vertex_ids.insert(from, *node);
                    }
                    if !vertex_ids.contains_key(&to) {
                        let node = &graph.add_node(-1);
                        vertex_ids.insert(to, *node);
                    }

                    graph.add_edge(
                        *vertex_ids.get(&from).unwrap(),
                        *vertex_ids.get(&to).unwrap(),
                        0
                    );

                }
            }
        }

        return Some(graph);
    }

    return None;

}

fn superman_sorting(graph: &Graph<i16, u8, Undirected, u32>) -> Vec<NodeIndex> {
    let mut stack : Vec<NodeIndex> = Vec::with_capacity(graph.node_count());

    let mut degrees = HashMap::with_capacity(graph.node_count());

    for vertex in graph.node_references() {
        degrees.insert(vertex.0, graph.neighbors(vertex.0).count());
    }

    while !degrees.is_empty() {
        let mut node = None;
        for vertex in degrees.keys() {
            if node == None || degrees.get(vertex) > degrees.get(&node.unwrap()) {
                node = Some(*vertex);
            }
        }

        for neighbour in graph.neighbors_undirected(node.unwrap()) {
            if degrees.contains_key(&neighbour) {
                *degrees.get_mut(&neighbour).unwrap() -= 1;
            }
        }

        degrees.remove(&node.unwrap());
        stack.push(node.unwrap());
    }
    stack.reverse();

    return stack;

}

fn greedy_colouring(graph: &mut Graph<i16, u8, Undirected, u32>, order: &Vec<NodeIndex>) -> i16 {
    let mut max = 0;
    let mut unvisited = order.clone();
    while !unvisited.is_empty() {

        let node = unvisited.pop().unwrap();
        let mut colours = HashSet::new();
        for neighbour in graph.neighbors(node) {
            if graph[neighbour] != -1 {
                colours.insert(graph[neighbour]);
            }
        }

        if colours.is_empty() {
            graph[node] = 0;
        } else {

            let mut max_colour : i16 = 0;
            for colour in &colours {
                if *colour > max_colour {
                    max_colour = *colour;
                }
            }

            let mut colour = 0;
            while colour <= max_colour {
                if !colours.contains(&colour) {
                    break;
                }

                colour += 1;
            }
            graph[node] = colour;
            max = cmp::max(colour, max);

        }

    }

    return max + 1;
}
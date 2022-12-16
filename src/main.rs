extern crate rand; 
use std::collections::BinaryHeap;
use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

fn make_graph(filename: &str) -> Vec<Vec<usize>> {
    let input = fs::read_to_string(filename).expect("Reading the file failed");
    parse_edges(filename);
    let mut lines = input.trim().split("\n");
    let n = match lines.next() {
        Some(line) => line.trim().parse::<usize>().unwrap(),
        None => return Vec::new(), // Return an empty matrix if there are no lines
    };
    let mut matrix = vec![vec![0; n]; n];
    for l in lines {
        let mut vertices = l.split(" ");
        let a = vertices.next().unwrap().trim().parse::<usize>().unwrap();
        let b = vertices.next().unwrap().trim().parse::<usize>().unwrap();
        matrix[a][b] = 1;
        matrix[b][a] = 1;
    }

    matrix
}
struct Node {
    // The distance of the node from the starting node.
    distance: usize,
    // The index of the node in the graph.
    index: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


struct Graph {

    adjacency_matrix: Vec<Vec<usize>>,

    num_nodes: usize,
    circles: HashMap<String, HashSet<usize>>,
    features: HashMap<usize, Vec<bool>>,
    ego_features: Vec<bool>,
    feature_names: Vec<String>,
    adj_matrix: Vec<Vec<i32>>,
}

impl Graph {
    fn new(num_nodes: usize) -> Self {
        let mut matrix = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            matrix.push(vec![0; num_nodes]);
        }
    
        Self {
            adj_matrix: matrix,
            num_nodes,
            circles: HashMap::new(),
            features: HashMap::new(),
            ego_features: Vec::new(),
            feature_names: Vec::new(),
            adjacency_matrix: Vec::new()
        }
    }

    fn add_edge(mut self, a: usize, b: usize) {
        self.adj_matrix[a][b] = 1;
    }

   

    fn longest_path(&self) -> usize {

        let mut distances: Vec<usize> = vec![std::usize::MAX; self.num_nodes];

        distances[0] = 0;

        let mut queue = BinaryHeap::new();
        queue.push(Node { distance: 0, index: 0 });


        while let Some(node) = queue.pop() {

            if distances[node.index] != std::usize::MAX {
                continue;
            }

            distances[node.index] = node.distance;


            for i in 0..self.num_nodes {
                if self.adjacency_matrix[node.index][i] != 0 {
                    let new_distance = node.distance + self.adjacency_matrix[node.index][i];
                    if new_distance < distances[i] {
                        distances[i] = new_distance;
                        queue.push(Node { distance: new_distance, index: i });
                    }
                }
            }
        }

        // Once all nodes have been visited, the longest path in the graph will be
        // the one with the largest distance from the starting node.
        *distances.iter().max().unwrap()
    }
}

fn read_graph(num_nodes: usize, edges: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut adjacency_matrix = vec![vec![0; num_nodes]; num_nodes];
    for (a, b) in edges {
        adjacency_matrix[a][b] = 1;
        adjacency_matrix[b][a] = 1;
    }
    adjacency_matrix
}


fn parse_edges(filename: &str) -> Vec<(usize, usize)> {
    let input = fs::read_to_string(filename).expect("Reading the file failed");
    let mut lines = input.trim().split("\n");

    // Skip the first line, which contains the number of nodes
    lines.next();

    let mut edges = Vec::new();
    for line in lines {
        let mut vertices = line.split(" ");
        let a = vertices.next().unwrap().trim().parse::<usize>().unwrap();
        let b = vertices.next().unwrap().trim().parse::<usize>().unwrap();
        edges.push((a, b));
    }

    edges
}

fn parse_circles(filename: &str) -> HashMap<String, HashSet<usize>> {
    let input = fs::read_to_string(filename).expect("Reading the file failed");

    let mut lines = input.trim().split("\n");


    let mut circles = HashMap::new();


    for l in lines {
        
        let mut parts = l.split(":");
        let name = parts.next().unwrap().trim().to_string();
        let nodes = parts.next().unwrap().trim().split(" ").map(|x| x.parse().unwrap()).collect();

        
        circles.insert(name, nodes);
    }

    circles
}

fn parse_features(filename: &str) -> HashMap<usize, Vec<bool>> {
   
    let input = fs::read_to_string(filename).expect("Reading the file failed");

 
    let mut lines = input.trim().split("\n");

    let mut features = HashMap::new();

  
    for l in lines {
        let mut parts = l.split(" ");
        let node_id = parts.next().unwrap().trim().parse::<usize>().unwrap();
        let mut node_features = Vec::new();
        for p in parts {
            let feature: bool = p.trim().parse().unwrap();
            node_features.push(feature);
        }
        features.insert(node_id, node_features);
    }

    features
}
fn parse_feature_names(filename: &str) -> Vec<String> {

    let input = fs::read_to_string(filename).expect("Reading the file failed");

   
    let lines = input.trim().split("\n");

   
    lines.map(|line| line.trim().to_string()).collect()
}
fn parse_ego_features(filename: &str) -> Vec<bool> {
    let input = fs::read_to_string(filename).expect("Reading the file failed");
    input.trim().split(' ').map(|s| s == "1").collect()
}

fn build_graph(
    edges: Vec<(usize, usize)>,
    circles: HashMap<String, HashSet<usize>>,
    features: HashMap<usize, Vec<bool>>,
    ego_features: Vec<bool>,
    feature_names: Vec<String>,
) -> Graph {
    let num_nodes = edges.len() + 1;
    let mut graph = Graph::new(4039);

    for (a, b) in edges {
        graph.add_edge(a, b);
        graph.add_edge(b, a);
    }

    graph.circles = circles;
    graph.features = features;
    graph.ego_features = ego_features;
    graph.feature_names = feature_names;

    graph
}


//This was the main function I used to display the longest path of facebook_combined file you see in the report. 
//fn main() {
  //  let matrix = make_graph("facebook_combined.txt");
    //let graph = Graph { adjacency_matrix: matrix.clone(), num_nodes: matrix.len() };
    //let longest_path = graph.longest_path();
    //println!("The longest path in the graph is {}", longest_path);
//}


fn main() {
    fn main() {
        let edges = parse_edges("0.edges");
        let circles = parse_circles("0.circles");
        let features = parse_features("0.feat");
        let ego_features = parse_ego_features("0.egofeat");
        let feature_names = parse_feature_names("0.featnames");
    
        // Use the parsed data to build a graph and find the longest path
        let graph = build_graph(edges, circles, features, ego_features, feature_names);
        let longest_path = graph.longest_path();
        println!("The longest path in the graph is {}", longest_path);
    }
}






extern crate rand; 
use std::collections::BinaryHeap;
use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

fn make_graph(filename: &str) -> Vec<Vec<usize>> {
    // Read the input file
    let input = fs::read_to_string(filename).expect("Reading the file failed");
    parse_edges(filename);

    // Split the input into lines
    let mut lines = input.trim().split("\n");

    // Read the number of nodes in the graph
    let n = match lines.next() {
        Some(line) => line.trim().parse::<usize>().unwrap(),
        None => return Vec::new(), // Return an empty matrix if there are no lines
    };

    // Initialize the adjacency matrix with all zeros
    let mut matrix = vec![vec![0; n]; n];

    // Read the edges from the input file and update the matrix
    for l in lines {
        let mut vertices = l.split(" ");
        let a = vertices.next().unwrap().trim().parse::<usize>().unwrap();
        let b = vertices.next().unwrap().trim().parse::<usize>().unwrap();
        matrix[a][b] = 1;
        matrix[b][a] = 1;
    }

    matrix
}

// Define a struct to represent a node in the graph.
struct Node {
    // The distance of the node from the starting node.
    distance: usize,
    // The index of the node in the graph.
    index: usize,
}




// Implement the PartialEq and Eq traits for the Node struct,
// so that we can compare nodes by their distance.
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Node {}

// Implement the PartialOrd and Ord traits for the Node struct,
// so that we can sort nodes by their distance using a priority queue.
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

// Define a struct to represent a graph.
struct Graph {
    // The adjacency matrix of the graph.
    // Each element in the matrix represents the distance between two nodes.
    adjacency_matrix: Vec<Vec<usize>>,
    // The number of nodes in the graph.
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

   
    // Define a method to find the longest path in the graph using Dijkstra's algorithm.
    fn longest_path(&self) -> usize {
        // Create a vector to hold the distances of each node from the starting node.
        let mut distances: Vec<usize> = vec![std::usize::MAX; self.num_nodes];
        // Set the distance of the starting node to 0.
        distances[0] = 0;

        // Create a priority queue to hold the nodes in the graph,
        // sorted by their distance from the starting node.
        let mut queue = BinaryHeap::new();
        queue.push(Node { distance: 0, index: 0 });

        // While the queue is not empty, remove the node with the smallest distance
        // and update the distances of its neighbors.
        while let Some(node) = queue.pop() {
            // If the current node has not been visited before, mark it as visited
            // and update its distance.
            if distances[node.index] != std::usize::MAX {
                continue;
            }

            distances[node.index] = node.distance;

            // For each neighbor of the current node, calculate the distance to that neighbor
            // using the current node as the starting point. If the calculated distance
            // is less than the current distance for that neighbor, update the distance
            // and add the neighbor to the queue.
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
    // Read the input file
    let input = fs::read_to_string(filename).expect("Reading the file failed");

    // Split the input into lines
    let mut lines = input.trim().split("\n");

    // Initialize the map of circles
    let mut circles = HashMap::new();

    // Read the circles from the input file and add them to the map
    for l in lines {
        // Split the line into the name of the circle and the list of nodes
        let mut parts = l.split(":");
        let name = parts.next().unwrap().trim().to_string();
        let nodes = parts.next().unwrap().trim().split(" ").map(|x| x.parse().unwrap()).collect();

        // Add the circle to the map
        circles.insert(name, nodes);
    }

    circles
}

fn parse_features(filename: &str) -> HashMap<usize, Vec<bool>> {
    // Read the input file
    let input = fs::read_to_string(filename).expect("Reading the file failed");

    // Split the input into lines
    let mut lines = input.trim().split("\n");

    let mut features = HashMap::new();

    // Read the features for each node from the input file
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
    // Read the input file
    let input = fs::read_to_string(filename).expect("Reading the file failed");

    // Split the input into lines
    let lines = input.trim().split("\n");

    // Iterate through the lines and parse the feature names
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






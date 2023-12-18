fn parse_value<T: std::str::FromStr>(value: Option<&str>) -> Option<T> {
    value?.parse::<T>().ok()
}

// Just a macro to parse lines
#[macro_export]
macro_rules! parse_line {
    ($separator: literal, $($t: ty), +) => ({
        let mut a_str = String::new();
        match (std::io::stdin().read_line(&mut a_str)) {
            Ok(_) => {
                a_str.pop(); // Remove new_line
                let mut a_iter = a_str.split($separator);
                Ok((
                    $(
                        parse_value::<$t>(a_iter.next()),
                    )+
                ))
            },
            Err(err) => Err(err),
        }
    })
}

#[derive(Default, Clone)]
struct Node {
    outgoing: Vec<usize>,
    incoming: Vec<usize>,
    value: Option<u32>,
    visited: bool
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            nodes: vec![Default::default(); n],
        }
    }
    pub fn push(&mut self, a: usize, b: usize) {
        self.nodes[a].outgoing.push(b);
        self.nodes[b].incoming.push(a);
    }
    pub fn out(&self, node: usize) -> impl Iterator<Item=&usize> + '_ {
        self.nodes[node].outgoing.iter()
    }
    pub fn dfs(&mut self, start_node_index: usize) {
        let mut stack = vec![start_node_index];

        while let Some(node_index) = stack.pop() {
            if !self.nodes[node_index].visited {
                self.nodes[node_index].visited = true;
                println!("Visiting Node: {}", node_index);

                // Process neighbors (outgoing edges)
                for &neighbor_index in &self.nodes[node_index].outgoing {
                    if !self.nodes[neighbor_index].visited {
                        stack.push(neighbor_index);
                    }
                }
            }
        }
    }
}

fn main() {
    
    let (Some(n), Some(m)) = parse_line!(" ", usize, usize).unwrap() else {
        panic!()
    };
    let mut graph = Graph::new(n, m);

    while let Ok((Some(a), Some(b))) = parse_line!(" ", usize, usize) {

    }
}

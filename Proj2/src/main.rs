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
                let mut a_iter = a_str.trim_end().split($separator);
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
#[derive(Clone, Debug)]
enum Visited {
    White,
    Gray,
    Black
}

impl Default for Visited {
    fn default() -> Self { Self::White }
}
impl Visited {
    fn black(&self) -> bool { 
        if let Visited::Black = self {
            return true;
        } else {
            return false;
        }
    }
    fn white(&self) -> bool { 
        if let Visited::White = self {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Default, Clone)]
struct Node {
    outgoing: Vec<usize>,
    incoming: Vec<usize>,
    value: u64,
    scc: Option<usize>,
    visited: Visited
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            nodes: vec![Default::default(); n],
        }
    }
    pub fn push(&mut self, a: usize, b: usize) {
        self.nodes[a-1].outgoing.push(b-1);
        self.nodes[b-1].incoming.push(a-1);
    }
    pub fn outcoming(&self, node: usize) -> impl Iterator<Item=&usize> + '_ {
        self.nodes[node].outgoing.iter()
    }
    pub fn incoming(&self, node: usize) -> impl Iterator<Item=&usize> + '_ {
        self.nodes[node].incoming.iter()
    }
    pub fn dfs_initial(&mut self) -> Vec<usize> {
        // Vector in end-time order
        let mut result = vec![];

        for i in 0..self.nodes.len() {
            // If visited
            if self.nodes[i].visited.black() {
                continue;
            }
            let mut stack = vec![i];

            while let Some(j) = stack.last() {
                match self.nodes[*j].visited {
                    Visited::White => {
                        self.nodes[*j].visited = Visited::Gray;
                        
                        for adj in self.outcoming(*j) {
                            if self.nodes[*adj].visited.white() {
                                stack.push(*adj);
                            }
                        }
                    }
                    Visited::Gray => {
                        self.nodes[*j].visited = Visited::Black;
                        result.push(stack.pop().unwrap());
                    }
                    Visited::Black => {
                        stack.pop();
                    }
                }
            }
        }
        for i in 0..self.nodes.len() {
            self.nodes[i].visited = Visited::White;
        }
        result
    }

    pub fn dfs_final(&mut self) -> u64 {
        let mut max_path = 0;
        let mut stack = vec![];

        for i in self.dfs_initial().iter().rev() {
            // println!("Doing: {}, {:?}", i+1, self.nodes[*i].visited);
            // If visited
            if self.nodes[*i].visited.black() {
                continue;
            }

            let mut scc_nodes: Vec<usize> = vec![];
            stack.push(*i);
            scc_nodes.push(*i);
            self.nodes[*i].scc = Some(*i);

            while let Some(j) = stack.last().cloned() {
                // If not visited
                if self.nodes[j].visited.white() {
                    self.nodes[j].visited = Visited::Gray;
                    self.nodes[j].scc = Some(*i);
                    // println!("Visited: {}", j+1);

                    // SCC
                    for adj in self.incoming(j) {
                        // If not visited
                        if self.nodes[*adj].visited.white() {
                            // println!("Added: {}", j+1);
                            stack.push(*adj);
                            scc_nodes.push(*adj);
                        }
                    }
                } else {
                    // println!("End: {}", j+1);
                    self.nodes[j].visited = Visited::Black;
                    stack.pop();
                }
            }
            let mut scc_value: u64 = 0;
            for no in scc_nodes {
                for adj in self.incoming(no) {
                    if self.nodes[*adj].scc != Some(*i) {
                        scc_value = scc_value.max(self.nodes[self.nodes[*adj].scc.unwrap()].value+1);
                    }
                }
            }
            self.nodes[*i].value = scc_value;
            max_path = max_path.max(scc_value);
        }
        max_path
    }
}

fn main() {
    let (Some(n), Some(m)) = parse_line!(" ", usize, usize).unwrap() else {
        panic!()
    };
    let mut graph = Graph::new(n);
    let start_time = std::time::Instant::now();

    while let Ok((Some(a), Some(b))) = parse_line!(" ", usize, usize) {
        graph.push(a, b);
    }

    println!("{},{},{},{:?}", graph.dfs_final(), n, m, start_time.elapsed().as_micros());
}

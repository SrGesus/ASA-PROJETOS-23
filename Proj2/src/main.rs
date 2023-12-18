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
    // Link Indexes
    links: Vec<usize>
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
// Source -> Destination
struct Link(usize, usize);

struct Graph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

impl Graph {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            nodes: vec![Default::default(); n],
            links: Vec::with_capacity(m)
        }
    }
    pub fn push(&mut self, l: Link) {
        self.nodes[l.0].links.push(self.links.len());
        self.links.push(l);
    }
    pub fn links(&self, node: usize) -> impl Iterator<Item=&Link> + '_ {
        self.nodes[node].links.iter().map(|i| &self.links[*i])
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

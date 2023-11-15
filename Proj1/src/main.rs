use std::collections::HashMap;

pub fn parse_value<T: std::str::FromStr>(value: Option<&str>) -> Option<T> {
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

#[derive(Hash, PartialEq, Eq, Debug)]
struct Piece {
    x: u32,
    y: u32,
}

impl Piece {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x: x.max(y), y: x.min(y) }
    }
    pub fn cut_x(&self, dimension: u32) -> (Piece, Piece) {
        (Piece::new(self.x-dimension, self.y), Piece::new(dimension, self.y))
    }
    pub fn cut_y(&self, dimension: u32) -> (Piece, Piece) {
        (Piece::new(self.x, self.y-dimension), Piece::new(self.x, dimension))
    }
    pub fn get_price(self: Self, prices: &mut HashMap<Piece, u32>) -> u32 {
        if let Some(price) = prices.get(&self) { return *price; };
        
        let mut price: u32 = 0;
        // Cut Horizontally
        for i in 1.max(self.x/2)..self.x {
            let (piece1, piece2) = Self::cut_x(&self, i);
            price = price.max(Self::get_price(piece1, prices) + Self::get_price(piece2, prices));
        }
        // Cut Vertically
        for i in 1.max(self.x/2)..self.y {
            let (piece1, piece2) = Self::cut_y(&self, i);
            price = price.max(Self::get_price(piece1, prices) + Self::get_price(piece2, prices));
        }
        prices.insert(self, price);
        price
    }
}

fn main() {
    let (Some(x), Some(y)) = parse_line!(" ", u32, u32).unwrap() else { panic!()};
    println!("{}, {}", x, y);
    let (Some(_num_pieces),) = parse_line!(" ", usize).unwrap() else { panic!()};

    let mut prices: HashMap<Piece, u32> = HashMap::new();

    while let Ok((Some(a), Some(b), Some(p))) = parse_line!(" ", u32, u32, u32) {
        println!("{} {} {}", a, b, p);
        prices.insert(Piece::new(a, b), p);
    }

    println!("{}", Piece::get_price(Piece::new(x, y), &mut prices));
}

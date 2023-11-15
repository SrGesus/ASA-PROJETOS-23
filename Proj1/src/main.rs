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
                        $crate::parse_value::<$t>(a_iter.next()),
                    )+
                ))
            },
            Err(err) => Err(err),
        }
    })
}

#[derive(Hash, PartialEq, Eq)]
struct Piece {
    x: i32,
    y: i32,
}



impl Piece {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn new_cut(x: i32, y: i32) -> Option<Self> {
        if x < 0 || y < 0 {
            return None;
        }
        Some(Self { x, y })
    }
    pub fn cut(&self, dimension: i32) -> (Option<Piece>, Option<Piece>) {
        (Piece::new_cut(self.x-dimension, self.y), Piece::new_cut(self.x, self.y-dimension))
    }
    pub fn get_price(target: Option<Self>, prices: &mut HashMap<Piece, u32>) -> u32 {
        let Some(target) = target else { return 0; };
        match prices.get(&target) {
            Some(price) => *price,
            None => {
                let mut price: u32 = 0;
                for i in 1..target.x.max(target.y) {
                    let (piece1, piece2) = Self::cut(&target, i);
                    price = price.max(Self::get_price(piece1, prices) + Self::get_price(piece2, prices));
                }
                prices.insert(target, price);
                price
            }
        }
    }
}

fn main() {
    let (Some(x), Some(y)) = parse_line!(" ", i32, i32).unwrap() else { panic!()};
    println!("{}, {}", x, y);
    let (Some(_num_pieces),) = parse_line!(" ", usize).unwrap() else { panic!()};

    let mut prices: HashMap<Piece, u32> = HashMap::new();

    while let Ok((Some(a), Some(b), Some(p))) = parse_line!(" ", i32, i32, u32) {
        println!("{} {} {}", a, b, p);
        prices.insert(Piece::new(a, b), p);
    }

    println!("{}", Piece::get_price(Some(Piece::new(x, y)), &mut prices));
}

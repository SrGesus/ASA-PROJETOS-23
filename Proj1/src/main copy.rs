use std::io::{BufRead, self};

pub fn parse_value<T: std::str::FromStr>(value: Option<&str>) -> Option<T> {
    value?.parse::<T>().ok()
}

fn read_line<'a>(
    lock: &mut std::io::StdinLock, 
    separator: &'a str,
    line: &'a mut String
) -> Option<std::str::Split<'a, &'a str>> {
    lock.read_line(line).ok()?;
    Some(line.trim().split(separator))
}

fn parse_line(
    lock: &mut std::io::StdinLock, 
    sep: &str,
) -> Option<(usize, usize, u32)> {
    let mut line = String::new();
    let mut a_iter = read_line(lock, sep, &mut line)?;
    Some((
        a_iter.next()?.parse::<usize>().ok()?,
        a_iter.next()?.parse::<usize>().ok()?,
        a_iter.next()?.parse::<u32>().ok()?
    ))
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
    x: usize,
    y: usize,
}

struct PriceTable {
    // Optimal Piece Prices
    prices: Vec<Option<u32>>,
    side: usize
}

struct Products {
    // Client Piece Prices
    products: Vec<u32>,
    sizes: Vec<usize>,
    side: usize
}

impl Products {
    pub fn new(x: usize, y: usize, n_pieces: usize) -> Self {
        let max = x.max(y) + 1;
        Self {
            products: vec![0; max*max],
            sizes: Vec::with_capacity(n_pieces),
            side: max
        }
    }
    pub fn get(self: &Self, piece: &Piece) -> u32 {
        unsafe {
            *self.products.get_unchecked(piece.x + piece.y * self.side)
        }

    }
    pub fn insert(self: &mut Self, piece: Piece, price: u32) {
            if piece.x >= self.products.len() || piece.y >= self.products.len() {
                return;
            }

            unsafe {
                *self.products.get_unchecked_mut(piece.x + piece.y * self.side) = price;
                *self.products.get_unchecked_mut(piece.y + piece.x * self.side) = price;
            }
            self.sizes.push(piece.x);
            self.sizes.push(piece.y);
    }
}

impl PriceTable {
    pub fn new(x: usize, y: usize) -> Self {
        let max = x.max(y) + 1;
        Self {
            prices: vec![None; max*max],
            side: max
        }
    }

    pub fn get(self: &Self, piece: &Piece) -> &Option<u32> {
        unsafe {
            self.prices.get_unchecked(piece.x + piece.y * self.side)
        }
    }

    pub fn insert(self: &mut Self, piece: Piece, price: u32) {
        unsafe {
            *self.prices.get_unchecked_mut(piece.x + piece.y * self.side) = Some(price);
            *self.prices.get_unchecked_mut(piece.y + piece.x * self.side) = Some(price);
        }
    }
}

// static mut CALLS: i64 = -1;

impl Piece {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn cut_x(&self, dimension: usize) -> (Piece, Piece) {
        (
            Piece::new(self.x - dimension, self.y),
            Piece::new(dimension, self.y),
        )
    }
    pub fn cut_y(&self, dimension: usize) -> (Piece, Piece) {
        (
            Piece::new(self.x, self.y - dimension),
            Piece::new(self.x, dimension),
        )
    }
    pub fn get_price(self: Self, prices: &mut PriceTable, products: &Products) -> u32 {
        if let Some(price) = prices.get(&self) {
            return *price;
        };
        let mut price: u32 = products.get(&self);
        // Cut Horizontally
        for i in &products.sizes {
            if i >= &self.x {
                break;
            }
            let (piece1, piece2) = Self::cut_x(&self, *i);
            // println!("{:?} + {:?}", piece1, piece2);
            price =
                price.max(piece1.get_price(prices, products) + piece2.get_price(prices, products));
        }
        // Cut Vertically
        for i in &products.sizes {
            if i >= &self.y {
                break;
            }
            let (piece1, piece2) = Self::cut_y(&self, *i);
            // println!("{:?} + {:?}", piece1, piece2);
            price =
                price.max(piece1.get_price(prices, products) + piece2.get_price(prices, products));
        }
        prices.insert(self, price);
        price
    }
}

fn main() {
    let (Some(x), Some(y)) = parse_line!(" ", usize, usize).unwrap() else {
        panic!()
    };
    // println!("{}, {}", x, y);
    let (Some(n_pieces),) = parse_line!(" ", usize).unwrap() else {
        panic!()
    };
    
    let mut prices: PriceTable = PriceTable::new(x, y);
    
    let mut products: Products = Products::new(x, y, n_pieces);
    
    let mut lock = io::stdin().lock();
    while let Some((a, b, p)) = parse_line(&mut lock, " ") {
        products.insert(Piece::new(a, b), p);
    }
    products.sizes.sort_unstable();
    products.sizes.dedup();
    //println!("{:?}", products.sizes);

    println!(
        "{}",
        Piece::get_price(Piece::new(x, y), &mut prices, &products)
    );
    // Piece::get_price(Piece::new(x, y), &mut prices);
    // unsafe {
    //     println!("{},{},{}", x,y,CALLS);
    // }
}

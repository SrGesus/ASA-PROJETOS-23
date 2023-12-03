use std::{
    io::{self, BufRead},
    vec,
};

fn read_line<'a>(
    lock: &mut io::StdinLock,
    separator: &'a str,
    line: &'a mut String,
) -> Option<std::str::Split<'a, &'a str>> {
    lock.read_line(line).ok()?;
    Some(line.trim().split(separator))
}

fn parse_line<'a>(a_iter: Option<std::str::Split<'a, &'a str>>) -> Option<(usize, usize, u32)> {
    let mut a_iter = a_iter?;
    Some((
        a_iter.next()?.parse::<usize>().ok()?,
        a_iter.next()?.parse::<usize>().ok()?,
        a_iter.next()?.parse::<u32>().ok()?,
    ))
}

#[derive(Debug, PartialEq)]
struct Piece(usize, usize);

impl Piece {
    pub fn new(x: usize, y: usize) -> Self {
        Piece(x, y)
    }
    pub fn rotate(self: &Self) -> Option<Self> {
        if self.1 != self.0 {
            Some(Piece(self.1, self.0))
        } else {
            None
        }
    }
    pub fn cut_piece(self: &Self, target: &Piece) -> (Option<(Self, Self)>, Option<(Self, Self)>) {
        (
            if self.0 > target.0 {
                Some((Piece(target.0, self.1), Piece(self.0 - target.0, self.1)))
            } else {
                None
            },
            if self.1 > target.1 {
                Some((Piece(self.0, target.1), Piece(self.0, self.1 - target.1)))
            } else {
                None
            },
        )
    }
    pub fn get_price(self: Self, prices: &mut PriceTable, products: &Products) -> u32 {
        if let Some(price) = prices.get(&self) {
            return *price;
        };
        let mut price = 0;
        for (pi, pr) in &products.products {
            if pi == &self {
                price = price.max(*pr);
            }
            let (g1, g2) = self.cut_piece(pi);
            if let Some((p1, p2)) = g1 {
                price = price.max(p1.get_price(prices, products) + p2.get_price(prices, products));
            }
            if let Some((p1, p2)) = g2 {
                price = price.max(p1.get_price(prices, products) + p2.get_price(prices, products));
            }
        }
        prices.insert(self, price);
        price
    }
}

struct Products {
    products: Vec<(Piece, u32)>,
}

impl Products {
    pub fn new(n_products: usize) -> Self {
        Products {
            products: Vec::with_capacity(n_products * 2),
        }
    }
    pub fn insert(self: &mut Self, piece: Piece, price: u32) {
        self.products.push((piece, price));
    }
    pub fn add_rotation(self: &mut Self) {
        for i in 0..self.products.len() {
            let (p, price) = &self.products[i];
            let Some(piece) = p.rotate() else {
                continue;
            };
            self.products.push((piece, *price));
        }
    }
    // pub fn recalculate_price(self: &mut Self, prices: PriceTable) {}
}

struct PriceTable {
    prices: Vec<Option<u32>>,
    side: usize,
}

impl PriceTable {
    pub fn new(x: usize, y: usize) -> Self {
        let max = x.max(y) + 1;
        PriceTable {
            prices: vec![None; max.pow(2)],
            side: max,
        }
    }
    pub fn get(self: &mut Self, piece: &Piece) -> &Option<u32> {
        unsafe { self.prices.get_unchecked(piece.0 + piece.1 * self.side) }
    }
    pub fn insert(self: &mut Self, piece: Piece, price: u32) {
        unsafe {
            *self.prices.get_unchecked_mut(piece.0 + piece.1 * self.side) = Some(price);
            *self.prices.get_unchecked_mut(piece.1 + piece.0 * self.side) = Some(price);
        }
    }
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut buffer = String::new();
    let mut line = read_line(&mut stdin, " ", &mut buffer).unwrap();
    let (x, y) = (
        line.next().unwrap().parse::<usize>().unwrap(),
        line.next().unwrap().parse::<usize>().unwrap(),
    );
    buffer = String::new();
    line = read_line(&mut stdin, " ", &mut buffer).unwrap();
    let n_products = line.next().unwrap().parse::<usize>().unwrap();

    let mut products = Products::new(n_products);
    let mut prices = PriceTable::new(x, y);

    while let Some((a, b, p)) = parse_line(read_line(&mut stdin, " ", &mut String::new())) {
        products.insert(Piece::new(a, b), p);
    }
    products.add_rotation();

    println!(
        "{}",
        Piece::get_price(Piece::new(x, y), &mut prices, &products)
    );
}

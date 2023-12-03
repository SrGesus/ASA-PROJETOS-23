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
struct Piece(usize, usize);

#[derive(Clone, Copy)]
enum Price {
    Product(u32),
    Final(u32),
}

struct PriceTable {
    // Prices Matrix
    prices: Vec<Price>,
    side: usize,
}

struct CutSizes {
    // Allowed Product sizes
    sizes: Vec<usize>,
    best: (Piece, usize),
}

impl CutSizes {
    pub fn new(n: usize) -> Self {
        Self {
            sizes: Vec::with_capacity(n*2),
            best: (Piece(1, 1), 0),
        }
    }
    pub fn add(&mut self, piece: Piece, price: u32) {
        self.sizes.push(piece.0);
        self.sizes.push(piece.1);
        if self.is_best(&piece, price) {
            self.best = (piece, price as usize);
        }
    }

    fn is_best(&self, piece: &Piece, price: u32) -> bool {
        self.best.0.area() * price as usize >= piece.area() * self.best.1
    }
}

impl PriceTable {
    pub fn new(x: usize, y: usize) -> Self {
        let max = x.max(y) + 1;
        Self {
            prices: vec![Price::Product(0); max * max],
            side: max,
        }
    }

    pub fn get(&self, piece: &Piece) -> &Price {
        unsafe { self.prices.get_unchecked(piece.0 + piece.1 * self.side) }
    }

    pub fn add_price(&mut self, piece: Piece, price: u32) {
        unsafe {
            *self.prices.get_unchecked_mut(piece.0 + piece.1 * self.side) = Price::Final(price);
            *self.prices.get_unchecked_mut(piece.1 + piece.0 * self.side) = Price::Final(price);
        }
    }

    pub fn add_product(&mut self, piece: Piece, price: u32) {
        if piece.0 >= self.side || piece.1 >= self.side {
            return;
        }
        if let Price::Product(p) = self.get(&piece) {
            if price < *p {
                return;
            }
        }

        self.prices[piece.0 + piece.1 * self.side] = Price::Product(price);
        self.prices[piece.1 + piece.0 * self.side] = Price::Product(price);
    }
}

impl Piece {
    pub fn area(&self) -> usize {
        self.0 * self.1
    }
    pub fn cut_x(&self, dimension: usize) -> (Piece, Piece) {
        (Piece(self.0 - dimension, self.1), Piece(dimension, self.1))
    }
    pub fn cut_y(&self, dimension: usize) -> (Piece, Piece) {
        (Piece(self.0, self.1 - dimension), Piece(self.0, dimension))
    }
    pub fn price(self, prices: &mut PriceTable, cut_sizes: &CutSizes) -> u32 {
        match *prices.get(&self) {
            Price::Final(price) => price,
            Price::Product(mut price) => {
                // Cut Horizontally
                for i in &cut_sizes.sizes {
                    if i >= &self.0 {
                        break;
                    }
                    let (p1, p2) = Self::cut_x(&self, *i);
                    price = price
                        .max(p1.price(prices, cut_sizes) + p2.price(prices, cut_sizes));
                    if cut_sizes.is_best(&self, price) {
                        prices.add_price(self, price);
                        return price;
                    }
                }
                // Cut Vertically
                for i in &cut_sizes.sizes {
                    if i >= &self.1 {
                        break;
                    }
                    let (p1, p2) = Self::cut_y(&self, *i);
                    price = price
                        .max(p1.price(prices, cut_sizes) + p2.price(prices, cut_sizes));
                    if cut_sizes.is_best(&self, price) {
                        break;
                    }
                }
                prices.add_price(self, price);
                price
            }
        }
    }
}

fn main() {
    let (Some(x), Some(y)) = parse_line!(" ", usize, usize).unwrap() else {
        panic!()
    };
    let (Some(n),) = parse_line!(" ", usize).unwrap() else {
        panic!()
    };
    
    let mut prices: PriceTable = PriceTable::new(x, y);
    
    let mut cut_sizes: CutSizes = CutSizes::new(n);
    
    while let Ok((Some(a), Some(b), Some(p))) = parse_line!(" ", usize, usize, u32) {
        prices.add_product(Piece(a, b), p);
        cut_sizes.add(Piece(a, b), p);
    }
    let start_time = std::time::Instant::now();
    
    cut_sizes.sizes.sort_unstable();
    cut_sizes.sizes.dedup();

    // println!("{}", Piece::price(Piece(x, y), &mut prices, &cut_sizes));
    Piece::price(Piece(x, y), &mut prices, &cut_sizes);
    println!("{},{},{},{},{:?}", x, y, n, cut_sizes.sizes.len(), start_time.elapsed().as_millis());
}

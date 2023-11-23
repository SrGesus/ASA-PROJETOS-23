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

struct PriceTable {
    // Optimal Piece Prices
    prices: Vec<Option<u32>>,
    side: usize,
}

struct Products {
    // Client Piece Prices
    products: Vec<u32>,
    sizes: Vec<usize>,
    side: usize,
    best: (Piece, usize),
}

impl Products {
    pub fn new(x: usize, y: usize, n_pieces: usize) -> Self {
        let max = x.max(y) + 1;
        Self {
            products: vec![0; max * max],
            sizes: Vec::with_capacity(n_pieces),
            side: max,
            best: (Piece(1, 1), 0),
        }
    }
    pub fn get(&self, piece: &Piece) -> u32 {
        unsafe { *self.products.get_unchecked(piece.0 + piece.1 * self.side) }
    }
    pub fn is_best(&self, piece: &Piece, price: u32) -> bool {
        self.best.0.area() * price as usize >= piece.area() * self.best.1
    }
    pub fn insert(&mut self, piece: Piece, price: u32) {
        if piece.0 >= self.products.len() || piece.1 >= self.products.len() {
            return;
        }
        unsafe {
            *self
                .products
                .get_unchecked_mut(piece.0 + piece.1 * self.side) = price;
            *self
                .products
                .get_unchecked_mut(piece.1 + piece.0 * self.side) = price;
        }
        self.sizes.push(piece.0);
        self.sizes.push(piece.1);
        if self.is_best(&piece, price) {
            self.best = (piece, price as usize);
        }
    }
}

impl PriceTable {
    pub fn new(x: usize, y: usize) -> Self {
        let max = x.max(y) + 1;
        Self {
            prices: vec![None; max * max],
            side: max,
        }
    }

    pub fn get(&self, piece: &Piece) -> &Option<u32> {
        unsafe { self.prices.get_unchecked(piece.0 + piece.1 * self.side) }
    }

    pub fn insert(&mut self, piece: Piece, price: u32) {
        unsafe {
            *self.prices.get_unchecked_mut(piece.0 + piece.1 * self.side) = Some(price);
            *self.prices.get_unchecked_mut(piece.1 + piece.0 * self.side) = Some(price);
        }
    }
}

// static mut CALLS: i64 = -1;

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
    pub fn get_price(self, prices: &mut PriceTable, products: &Products) -> u32 {
        if let Some(price) = prices.get(&self) {
            return *price;
        };
        let mut price: u32 = products.get(&self);
        // Cut Horizontally
        for i in &products.sizes {
            if i >= &self.0 {
                break;
            }
            let (piece1, piece2) = Self::cut_x(&self, *i);
            // println!("{:?} + {:?}", piece1, piece2);
            price =
                price.max(piece1.get_price(prices, products) + piece2.get_price(prices, products));
            if products.is_best(&self, price) {
                prices.insert(self, price);
                return price;
            }
        }
        // Cut Vertically
        for i in &products.sizes {
            if i >= &self.1 {
                break;
            }
            let (piece1, piece2) = Self::cut_y(&self, *i);
            // println!("{:?} + {:?}", piece1, piece2);
            price =
                price.max(piece1.get_price(prices, products) + piece2.get_price(prices, products));
            if products.is_best(&self, price) {
                break;
            }
        }
        prices.insert(self, price);
        price
    }
}

fn main() {
    let (Some(x), Some(y)) = parse_line!(" ", usize, usize).unwrap() else {
        panic!()
    };
    let (Some(n_pieces),) = parse_line!(" ", usize).unwrap() else {
        panic!()
    };

    let mut prices: PriceTable = PriceTable::new(x, y);

    let mut products: Products = Products::new(x, y, n_pieces);

    while let Ok((Some(a), Some(b), Some(p))) = parse_line!(" ", usize, usize, u32) {
        products.insert(Piece(a, b), p);
    }
    products.sizes.sort_unstable();
    products.sizes.dedup();

    println!("{}", Piece::get_price(Piece(x, y), &mut prices, &products));
}

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
    x: usize,
    y: usize,
}

struct PriceTable {
    prices: Vec<Vec<Option<u32>>>
}

impl PriceTable {
    pub fn new(x: usize, y: usize) -> Self {
        Self { prices: vec![vec![None; x.max(y)]; x.max(y)] }
    }
    pub fn get(self: &Self, piece: &Piece) -> &Option<u32> {
        &self.prices[piece.x-1][piece.y-1]
    }
    pub fn insert(self: &mut Self, piece: Piece, price: u32) {
        self.prices[piece.x-1][piece.y-1] = Some(price);
        self.prices[piece.y-1][piece.x-1] = Some(price);
    }
}

static mut CALLS: i64 = -1;

impl Piece {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn cut_x(&self, dimension: usize) -> (Piece, Piece) {
        (Piece::new(self.x-dimension, self.y), Piece::new(dimension, self.y))
    }
    pub fn cut_y(&self, dimension: usize) -> (Piece, Piece) {
        (Piece::new(self.x, self.y-dimension), Piece::new(self.x, dimension))
    }
    pub fn get_price(self: Self, prices: &mut PriceTable) -> u32 {
        unsafe {
            CALLS += 1;
        }
        if let Some(price) = prices.get(&self) { 
            return *price; 
        };
        let mut price: u32 = 0;
        // Cut Horizontally
        for i in 1..self.x/2+1 {
            let (piece1, piece2) = Self::cut_x(&self, i);
            // println!("{:?} + {:?}", piece1, piece2);
            price = price.max(Self::get_price(piece1, prices) + Self::get_price(piece2, prices));
        }
        // Cut Vertically
        for i in 1..self.y/2+1 {
            let (piece1, piece2) = Self::cut_y(&self, i);
            // println!("{:?} + {:?}", piece1, piece2);
            price = price.max(Self::get_price(piece1, prices) + Self::get_price(piece2, prices));
        }
        prices.insert(self, price);
        price
    }
}

fn main() {
    let (Some(x), Some(y)) = parse_line!(" ", usize, usize).unwrap() else { panic!()};
    // println!("{}, {}", x, y);
    let (Some(_num_pieces),) = parse_line!(" ", usize).unwrap() else { panic!()};

    let mut prices: PriceTable = PriceTable::new(x,y);

    while let Ok((Some(a), Some(b), Some(p))) = parse_line!(" ", usize, usize, u32) {
        println!("{} {} {}", a, b, p);
        prices.insert(Piece::new(a, b), p);
    }

    // println!("Price: {}", Piece::get_price(Piece::new(x, y), &mut prices));
    Piece::get_price(Piece::new(x, y), &mut prices);
    unsafe {
        println!("{},{},{}", x,y,CALLS);
    }
}

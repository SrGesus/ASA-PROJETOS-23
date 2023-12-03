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

struct Piece(usize, usize);

struct Products {
    products: Vec<Piece>, 
}

struct PriceTable {
    prices: Vec<Option<u32>>,
}

fn main() {
    let stdin = std::io::stdin().lock();

    while Some(a, b, p) = parse_line(lock, " ") {
        
    }
}

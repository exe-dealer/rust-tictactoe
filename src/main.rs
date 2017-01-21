use std::io::{ self, Write };

fn main() {
    let combos = [
        0b111000000, 0b000111000, 0b000000111,
        0b100100100, 0b010010010, 0b001001001,
        0b100010001, 0b001010100
    ];
    let mut grid = [('x', 0), ('o', 0)];
    loop {
        let (curr_mark, curr_bits) = grid[0];
        let (prev_mark, prev_bits) = grid[1];

        if let Some(&win) = combos.iter().find(|&&combo| combo == prev_bits & combo) {
            print_grid(|loc| ['-', prev_mark][win >> loc & 1]);
            break;
        }

        print_grid(|loc| grid.iter()
            .find(|&&(_, b)| b >> loc & 1 == 1)
            .map_or((b'0' + loc) as char, |&(m, _)| m));

        if curr_bits | prev_bits == 0b111111111 {
            break;
        }

        print!("{}> ", curr_mark);
        io::stdout().flush().unwrap();
        let mut ln = String::new();
        io::stdin().read_line(&mut ln).unwrap();
        let loc = ln.trim().parse().unwrap_or(-1);
        if loc < 0 || loc > 8 || (curr_bits | prev_bits) & (1 << loc) != 0 {
            println!("invalid location");
            continue;
        }

        grid[0].1 |= 1 << loc; 
        grid.reverse();
    }
}

fn print_grid<T: Fn(u8) -> char>(c: T) {
    println!("{}{}{}\n{}{}{}\n{}{}{}\n",
        c(0), c(1), c(2),
        c(3), c(4), c(5),
        c(6), c(7), c(8));
}

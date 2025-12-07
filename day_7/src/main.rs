use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(not(debug_assertions))]
static IN_FILE: &str = "input.txt";
#[cfg(debug_assertions)]
static IN_FILE: &str = "in_small.txt";

#[derive(PartialEq)]
enum Tile {
    Empty,
    Start,
    Split,
    Beam(u64),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            'S' => Tile::Start,
            '^' => Tile::Split,
            '|' => Tile::Beam(1),
            _ => panic!("Unexpected character '{value}'"),
        }
    }
}

fn main() -> std::io::Result<()> {
    // open puzzle input
    let f = File::open(IN_FILE)?;
    let reader = BufReader::new(f);

    let mut board: Vec<Vec<Tile>> = Vec::new();

    // process line by line, accumulating the numbers in the respective task lists
    for line in reader.lines() {
        let line = line?;

        board.push(line.trim().chars().map(Tile::from).collect());
    }

    let mut num_splits = 0;
    for y in 1..board.len() {
        for x in 0..board[y].len() {
            // special rule for line 1
            if board[y - 1][x] == Tile::Start {
                board[y][x] = Tile::Beam(1);
            }

            // if above is |, then propagate down or split
            if let Tile::Beam(num) = board[y - 1][x] {
                match board[y][x] {
                    Tile::Empty => {
                        // just propagate the beam if there is no beam there
                        board[y][x] = Tile::Beam(num);
                    }
                    Tile::Beam(pre) => {
                        // this is us running into the case where we merge a beam from above with a
                        // beam freshly split off
                        board[y][x] = Tile::Beam(pre + num);
                    }
                    Tile::Split => {
                        if x > 0 {
                            // we propagate left to right, so on the left we must check if there's a
                            // beam in place already. If so, add the two numbers together
                            if let Tile::Beam(pre) = board[y][x - 1] {
                                board[y][x - 1] = Tile::Beam(pre + num);
                            } else {
                                board[y][x - 1] = Tile::Beam(num);
                            }
                        }
                        if x < board[y].len() - 1 {
                            board[y][x + 1] = Tile::Beam(num);
                        }
                        num_splits += 1;
                    }
                    _ => (),
                }
            }
        }
    }

    // Sum the total number of splits
    let sum_pt2 = board[board.len() - 1].iter().fold(0, |acc, cell| {
        if let Tile::Beam(x) = cell {
            acc + x
        } else {
            acc
        }
    });

    println!("====== Number of splits ======");
    println!("According to the first method: {num_splits}");
    println!("According to the new method: {sum_pt2}");

    Ok(())
}

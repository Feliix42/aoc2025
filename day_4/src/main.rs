use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;

#[cfg(not(debug_assertions))]
static IN_FILE: &str = "input.txt";
#[cfg(debug_assertions)]
static IN_FILE: &str = "in_small.txt";

#[derive(Copy, Clone, PartialEq)]
enum Field {
    Free,
    Paper,
}

impl Field {
    pub fn is_occupied(&self) -> bool {
        *self == Field::Paper
    }
}

impl AddAssign<&Field> for u32 {
    fn add_assign(&mut self, rhs: &Field) {
        match rhs {
            Field::Free => (),
            Field::Paper => self.add_assign(1),
        }
    }
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '.' => Field::Free,
            '@' => Field::Paper,
            c => panic!("Unrecognized character on the board: {c}"),
        }
    }
}

fn occupied_neighbors(fields: &Vec<Vec<Field>>, x: usize, y: usize) -> u32 {
    let max_x = fields[y].len();
    let max_y = fields.len();
    let mut total_occupied: u32 = 0;

    let is_max_x = x == (max_x - 1);
    let is_min_x = x == 0;
    let is_max_y = y == (max_y - 1);
    let is_min_y = y == 0;

    if !is_min_x {
        total_occupied += &fields[y][x - 1];

        if !is_min_y {
            total_occupied += &fields[y - 1][x - 1];
        }
        if !is_max_y {
            total_occupied += &fields[y + 1][x - 1];
        }
    }

    if !is_min_y {
        total_occupied += &fields[y - 1][x];
    }
    if !is_max_y {
        total_occupied += &fields[y + 1][x];
    }

    if !is_max_x {
        total_occupied += &fields[y][x + 1];

        if !is_min_y {
            total_occupied += &fields[y - 1][x + 1];
        }
        if !is_max_y {
            total_occupied += &fields[y + 1][x + 1];
        }
    }

    total_occupied
}

fn main() -> std::io::Result<()> {
    // open puzzle input
    let f = File::open(IN_FILE)?;
    let reader = BufReader::new(f);

    let mut fields: Vec<Vec<Field>> = Vec::new();

    // process line by line, accumulating the battery charges as instructed
    for line in reader.lines() {
        let line = line?;

        fields.push(line.trim().chars().into_iter().map(Field::from).collect());
    }

    let mut suitable1 = 0;
    for y in 0..fields.len() {
        for x in 0..fields[0].len() {
            if fields[y][x].is_occupied() && occupied_neighbors(&fields, x, y) < 4 {
                if cfg!(debug_assertions) {
                    println!("Field {x} | {y} is suitable");
                }
                suitable1 += 1;
            }
        }
    }

    let mut suitable2 = 0;
    loop {
        let mut changed = 0;
        let mut tmp = fields.clone();

        for y in 0..fields.len() {
            for x in 0..fields[0].len() {
                if fields[y][x].is_occupied() && occupied_neighbors(&fields, x, y) < 4 {
                    changed += 1;
                    tmp[y][x] = Field::Free;
                }
            }
        }

        if changed == 0 {
            break;
        }

        fields = tmp;
        suitable2 += changed;
    }

    println!("====== Suitable Paper rolls ======");
    println!("According to the old method: {suitable1}");
    println!("According to the new method: {suitable2}");

    Ok(())
}

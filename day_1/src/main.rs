use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[cfg(not(debug_assertions))]
static IN_FILE: &str = "input.txt";
#[cfg(debug_assertions)]
static IN_FILE: &str = "in_small.txt";

fn main() -> std::io::Result<()> {
    // open puzzle input
    let f = File::open(IN_FILE)?;
    let reader = BufReader::new(f);

    let mut pass1 = 0;
    let mut pass2 = 0;
    let mut acc = 50;

    // process line by line, checking the instruction and applying the requested operation on our accumulator
    for line in reader.lines() {
        let line = line?;

        let delta = i32::from_str(&line[1..]).expect("failed to parse");
        match line.chars().nth(0).unwrap() {
            'R' => acc += delta,
            'L' => acc -= delta,
            _ => panic!("Malformed input returned invalid value for rotation direction"),
        }

        // extra counting for task 2: Either when we cross from > 0 to < 0 or whenever we are > 100
        if acc.is_negative() && acc != -delta {
            if cfg!(debug_assertions) {
                println!("Triggering [is negative] on {acc} after {delta}");
            }
            pass2 += 1;
        }
        pass2 += (acc.abs() - 1) / 100;

        // counting according to task 1
        acc = acc.rem_euclid(100);
        if acc == 0 {
            pass1 += 1;
        }
    }

    pass2 += pass1;
    println!("====== PASSWORD RESULT ======");
    println!("According to the old method: {pass1}");
    println!("According to the new method: {pass2}");

    Ok(())
}

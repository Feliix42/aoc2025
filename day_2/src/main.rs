use std::fmt::Write;
use std::str::FromStr;
//use std::io::Write;

#[cfg(not(debug_assertions))]
static IN_FILE: &str = "input.txt";
#[cfg(debug_assertions)]
static IN_FILE: &str = "in_small.txt";

fn main() -> std::io::Result<()> {
    // open puzzle input
    let input = std::fs::read_to_string(IN_FILE)?;

    let mut sum_invalid1: u64 = 0;
    let mut sum_invalid2: u64 = 0;

    let mut tmp = String::new();

    // process line by line, checking the instruction and applying the requested operation on our accumulator
    for pair in input.trim().split(",") {
        let (str1, str2) = pair.split_once("-").unwrap();
        let lower = u64::from_str(str1).expect("Number parsing failed!");
        let upper = u64::from_str(str2).expect("Number parsing failed!");

        let mut i = lower;
        while i <= upper {
            tmp.clear();
            write!(&mut tmp, "{i}").unwrap();

            // part 1
            let (lhs, rhs) = tmp.split_at(tmp.len() / 2);
            if lhs == rhs {
                sum_invalid1 += i;
            }

            // part 2
            'outer: for sub_len in 1..=(tmp.len() / 2) {
                if tmp.len() % sub_len != 0 {
                    continue;
                }

                let n = tmp.len() / sub_len;
                let mut invalid = true;

                for x in 0..(n - 1) {
                    invalid &= tmp[x * sub_len..(x + 1) * sub_len]
                        == tmp[(x + 1) * sub_len..(x + 2) * sub_len];
                }

                if invalid {
                    // WE GOT A MATCH!
                    if cfg!(debug_assertions) {
                        println!("Invalid: {tmp}");
                    }
                    sum_invalid2 += i;
                    break 'outer;
                }
            }

            i += 1;
        }
    }

    println!("====== ACCUMULATED INVALID NUMBERS ======");
    println!("According to the old method: {sum_invalid1}");
    println!("According to the new method: {sum_invalid2}");

    Ok(())
}

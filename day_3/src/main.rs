#![feature(linked_list_remove)]
use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(not(debug_assertions))]
static IN_FILE: &str = "input.txt";
#[cfg(debug_assertions)]
static IN_FILE: &str = "in_small.txt";

fn main() -> std::io::Result<()> {
    // open puzzle input
    let f = File::open(IN_FILE)?;
    let reader = BufReader::new(f);

    let mut total1 = 0;
    let mut total2 = 0;

    // process line by line, accumulating the battery charges as instructed
    for line in reader.lines() {
        let line = line?;

        let mut acc = 0;
        for num in line
            .trim()
            .chars()
            .map(|c| char::to_digit(c, 10))
            .map(Option::unwrap)
        {
            let second_greater = acc % 10 > acc / 10;
            // check whether to insert the number at all
            if num > acc % 10 || second_greater {
                // check whether to replace the upper number, too
                if second_greater {
                    acc = acc % 10 * 10 + num;
                } else {
                    acc = (acc / 10) * 10 + num;
                }
            }
        }

        total1 += acc;

        // TASK 2
        let mut acc_2 = LinkedList::new();
        let mut carry = 0;
        let mut char_iter = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10))
            .map(Option::unwrap)
            .rev();
        for _ in 0..12 {
            carry = char_iter.next().unwrap();
            acc_2.push_front(carry as u64);
        }

        for num in char_iter {
            if num >= carry {
                let mut idx = 0;
                let mut tmp = acc_2.front().unwrap();
                for (i, elem) in acc_2.iter().enumerate().skip(1) {
                    if elem > tmp {
                        break;
                    } else {
                        tmp = elem;
                        idx = i;
                    }
                }

                acc_2.remove(idx);
                acc_2.push_front(num as u64);
                carry = num;
            }
        }

        let acc_2_result = acc_2.into_iter().fold(0, |acc, elem| acc * 10 + elem);
        if cfg!(debug_assertions) {
            println!("Result: {acc_2_result}");
        }
        total2 += acc_2_result;
    }

    println!("====== Total Joltage ======");
    println!("According to the old method: {total1}");
    println!("According to the new method: {total2}");

    Ok(())
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[cfg(not(debug_assertions))]
static IN_FILE: &str = "input.txt";
#[cfg(debug_assertions)]
static IN_FILE: &str = "in_small.txt";

enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Default)]
struct MathProblem<T: Add + Sub + Mul + Div> {
    pub inputs: Vec<T>,
    pub op: Option<MathOp>,
}

impl<T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>> MathProblem<T> {
    pub fn add_number(&mut self, num: T) {
        self.inputs.push(num);
    }

    pub fn set_op(&mut self, op_str: &str) {
        match op_str {
            "+" => self.op = MathOp::Add.into(),
            "-" => self.op = MathOp::Sub.into(),
            "*" => self.op = MathOp::Mul.into(),
            "/" => self.op = MathOp::Div.into(),
            _ => panic!("unexpected math operation '{op_str}'"),
        }
    }

    pub fn solve(self) -> T {
        let functor = match self.op.unwrap() {
            MathOp::Sub => std::ops::Sub::sub,
            MathOp::Add => std::ops::Add::add,
            MathOp::Mul => std::ops::Mul::mul,
            MathOp::Div => std::ops::Div::div,
        };

        let mut nums = self.inputs.into_iter();
        let init = nums.next().expect("can't have a task list with 0 numbers!");

        nums.fold(init, functor)
    }
}

fn main() -> std::io::Result<()> {
    // open puzzle input
    let f = File::open(IN_FILE)?;
    let reader = BufReader::new(f);

    let mut tasks = Vec::new();
    let mut initialized = false;

    // process line by line, accumulating the numbers in the respective task lists
    for line in reader.lines() {
        let line = line?;

        if !initialized {
            for _ in 0..line.trim().split_whitespace().count() {
                tasks.push(MathProblem::default());
            }
            initialized = true;
        }

        for (prob, input) in tasks.iter_mut().zip(line.trim().split_whitespace()) {
            if let Ok(num) = i64::from_str(input) {
                prob.add_number(num);
            } else {
                // it's got to be the op
                prob.set_op(input);
            }
        }
    }

    let res1: i64 = tasks.into_iter().map(MathProblem::solve).sum();

    println!("====== Sum of final results ======");
    println!("According to the first method: {res1}");
    //println!("According to the new method: {suitable2}");

    Ok(())
}

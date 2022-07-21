// TODO: remove these
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    fmt,
    io::{self, Write},
};

pub const ORDS: [&str; 5] = ["first", "second", "third", "fourth", "fifth"];

enum ArgsLength {
    Constant(u32),
    AtLeast(u32),
    AtMost(u32),
    AtLeastAtMost(u32, u32),
}

impl fmt::Display for ArgsLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgsLength::AtLeastAtMost(a, b) => write!(f, "{} to {}", a, b),

            ArgsLength::Constant(a) => {
                write!(f, "{}", a)
            }
            ArgsLength::AtLeast(a) => {
                write!(f, "at least {}", a)
            }
            ArgsLength::AtMost(a) => {
                write!(f, "at most {}", a)
            }
        }
    }
}

struct Calculation {
    name: String,
    desc: String,
    args: ArgsLength,
    fun: fn(&Vec<f64>) -> Result<f64, String>,
}

impl Calculation {
    fn read_inputs(&self) -> Vec<f64> {
        let mut stoppable = false;
        let (lower, upper) = match self.args {
            ArgsLength::Constant(a) => (a, a),
            ArgsLength::AtLeast(a) => {
                stoppable = true;
                (a, u32::MAX)
            }
            ArgsLength::AtMost(a) => {
                stoppable = true;

                (0, a)
            }
            ArgsLength::AtLeastAtMost(a, b) => (a, b),
        };

        if stoppable {
            println!("Submit 's' to stop entering arguments");
        }

        {
            let mut inputs = Vec::new();
            'a: for i in 1..=upper {
                inputs.push('inp: loop {
                    {
                        let index = i - 1;
                        print!(
                            "Enter {}:",
                            if (index as usize) < ORDS.len() {
                                format!("the {} argument", ORDS[index as usize])
                            } else {
                                format!("argument #{}", i)
                            }
                        )
                    };
                    flush();

                    match read_parse::<f64>() {
                        Ok(res) => break 'inp res,
                        Err(s) if stoppable && s.trim().to_lowercase() == "s" => {
                            if i <= lower {
                                println!("You need to enter at least {} arguments.", lower);
                            } else {
                                break 'a;
                            }
                        }
                        Err(_) => {
                            println!("Enter a valid number pls");
                            continue 'inp;
                        }
                    }
                });
            }
            inputs
        }
    }
}

impl fmt::Display for Calculation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}. \n\t\ttakes {} arguments",
            self.name, self.desc, self.args
        )
    }
}

fn read_parse<T: std::str::FromStr>() -> Result<T, String> {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    match input_text.trim().parse::<T>() {
        Ok(res) => Ok(res),
        Err(_) => Err(input_text),
    }
}

fn flush() {
    io::stdout().flush().expect("couldn't write to stdout");
}

fn main() -> ! {
    let calcs = vec![
        Calculation {
            name: "Addition".to_owned(),
            desc: "Adds two numbers".to_owned(),
            args: ArgsLength::AtLeast(2),
            fun: |xs| Ok(xs.iter().fold(0.0, |acc, item| acc + item)),
        },
        Calculation {
            name: "Subtraction".to_owned(),
            desc: "Subtracts the second number from the first".to_owned(),
            args: ArgsLength::Constant(2),
            fun: |xs| Ok(xs[0] - xs[1]),
        },
        Calculation {
            name: "Multiplication".to_owned(),
            desc: "Multiplies two numbers".to_owned(),
            args: ArgsLength::AtLeast(2),
            fun: |xs| Ok(xs.iter().fold(0.0, |acc, item| acc * item)),
        },
        Calculation {
            name: "Reciprocal sum".to_owned(),
            desc: "Returns the reciprocal of the sum of reciprocals of three numbers".to_owned(),
            args: ArgsLength::AtLeast(2),
            fun: |xs| {
                let r = |x: &f64| 1.0 / x;
                Ok(r(&xs.iter().map(r).sum()))
            },
        },
    ];

    println!("Welcome to the calculator lmao");
    loop {
        for (i, calcer) in calcs.iter().enumerate() {
            println!("{}.\t{}", i + 1, calcer);
        }

        println!();
        let calcer = &calcs[(loop {
            print!("Choose an operation:");
            flush();

            match read_parse::<i32>() {
                Ok(res) if (res as usize) <= calcs.len() && res > 0 => {
                    break res;
                }
                Ok(_) => {
                    println!("Operation not suppoe");
                    flush();
                }
                Err(_) => {
                    println!("Couldn't read integer, try again.");
                    flush();
                }
            }
        } - 1) as usize];

        println!("\nDoing {}", calcer.name.to_lowercase());
        let inputs = calcer.read_inputs();

        match (calcer.fun)(&inputs) {
            Err(s) => {
                todo!("{}", s)
            }
            Ok(result) => println!("The result is {}\n\n", result),
        };
    }
}

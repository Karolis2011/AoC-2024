use std::{cell::RefCell, collections::HashMap, io::BufRead, sync::{Arc, RwLock}};
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Concatenate,
}

impl Operator {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Subtract => a - b,
            Operator::Multiply => a * b,
            Operator::Divide => a / b,
            Operator::Concatenate => {
                // 12 || 34 = 1234
                let mut bt = b;
                let mut at = a;
                while bt > 0 {
                    at *= 10;
                    bt /= 10;
                }
                at + b
            }
        }
    }

    const OPERATOR_COUNT: usize = 3;

    const ALL: [Operator; Self::OPERATOR_COUNT] = [
        Operator::Add,
        // Operator::Subtract,
        Operator::Multiply,
        // Operator::Divide,
        Operator::Concatenate,
    ];
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Concatenate => "||",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn parse(input: &str) -> Self {
        let s1 = input.split(':').collect::<Vec<&str>>();
        if s1.len() != 2 {
            panic!("Invalid input: {}", input);
        }
        let result = s1[0].trim().parse().unwrap_or_else(|_| {
            panic!("Invalid result: {}", s1[0]);
        });
        let numbers = s1[1]
            .split(' ')
            .filter_map(|x| x.trim().parse().ok())
            .collect();
        Equation { result, numbers }
    }

    fn generate(&self) -> EquationGenerator {
        let mut res = EquationGenerator {
            equation: self,
            operator_states: vec![0; self.numbers.len() - 1],
        };
        res.operator_states[0] = -1;
        res
    }
}

#[derive(Debug)]
struct EquationWithOperators<'a> {
    equation: &'a Equation,
    operators: Vec<Operator>,
}

impl<'a> EquationWithOperators<'a> {
    fn new(equation: &'a Equation, operators: Vec<Operator>) -> Self {
        EquationWithOperators {
            equation,
            operators,
        }
    }

    fn evaluate(&self) -> i64 {
        let mut accumulator = self.equation.numbers[0];
        for (i, &number) in self.equation.numbers.iter().skip(1).enumerate() {
            let operator = self.operators[i];
            accumulator = operator.apply(accumulator, number);
        }
        accumulator
    }

    fn is_valid(&self) -> bool {
        self.equation.result == self.evaluate()
    }
}

impl std::fmt::Display for EquationWithOperators<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = self.equation.result.to_string();
        result.push_str(" = ");
        result.push_str(&self.equation.numbers[0].to_string());
        for (i, &number) in self.equation.numbers.iter().skip(1).enumerate() {
            result.push_str(&format!(" {} {}", self.operators[i], number));
        }
        write!(f, "{}", result)
    }
}

struct EquationGenerator<'a> {
    equation: &'a Equation,
    operator_states: Vec<i8>,
}

impl<'a> EquationGenerator<'a> {
    fn next_state(&mut self) -> bool {
        for i in 0..self.operator_states.len() {
            self.operator_states[i] += 1;
            if self.operator_states[i] == Operator::OPERATOR_COUNT as i8 {
                self.operator_states[i] = 0;
            } else {
                return true;
            }
        }
        false
    }

    fn to_equation(&self) -> EquationWithOperators<'a> {
        let operators = self
            .operator_states
            .iter()
            .map(|&x| Operator::ALL[x as usize])
            .collect::<Vec<Operator>>();
        EquationWithOperators::new(self.equation, operators)
    }
}

impl<'a> Iterator for EquationGenerator<'a> {
    type Item = EquationWithOperators<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_state() {
            Some(self.to_equation())
        } else {
            None
        }
    }
}

fn main() {
    let mut equations = Vec::new();
    let stdin = std::io::stdin();
    {
        let lock = stdin.lock();

        for line in lock.lines() {
            equations.push(Equation::parse(&line.unwrap()));
        }
    }

    // println!("Equations: {:?}", equations);

    let permutations_done = Arc::new(RwLock::new(0));  
    let incr_permutations_done = || {
        *permutations_done.write().unwrap() += 1;
        let permutations_done = permutations_done.read().unwrap().clone();
        if permutations_done % 1000000 == 0 {
            println!("Permutations done: {}", permutations_done);
        }
    };
    let valid_permutations = equations
        .par_iter()
        .filter_map(|e| {
            e.generate()
                // .inspect(|e| println!("Trying {}", e))
                // .inspect(|_| {
                //     incr_permutations_done();
                // })
                .find(|e| e.is_valid())
        })
        // Tun in to hashmap to remove duplicates\
        .fold(HashMap::new, |mut acc, e| {
            acc.entry(e.equation).or_insert(e);
            acc
        })
        .reduce(HashMap::new, |mut acc, e| {
            acc.extend(e);
            acc
        });

    // for (_, e) in valid_permutations.iter() {
    //     println!("{}", e);
    // }

    println!(
        "Result: {}",
        valid_permutations
            .iter()
            .map(|(e, _)| e.result)
            .sum::<i64>()
    );
}

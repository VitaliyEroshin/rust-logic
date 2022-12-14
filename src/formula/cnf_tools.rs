use super::cnf::{CNF, Literal};
use std::io::{self, BufRead};
use std::fs::File;
use rand::Rng;

pub fn get_benchmark_cnf(variables: usize, clauses: usize, var_in_clauses: usize) -> CNF {
    let mut cnf = CNF::new();
    for _ in 0..clauses {
        let mut clause = Vec::new();
        for _ in 0..var_in_clauses {
            let mut rng = rand::thread_rng();
            let var = rng.gen_range(0..variables);
            let sign = rng.gen();
            clause.push(Literal { var, sign });
        }
        cnf.add_clause(clause);
    }
    return cnf;
}

pub fn get_cnf_from_file(path: &str) -> CNF {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut cnf = CNF::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut clause = Vec::new();
        for literal in line.split(" ") {
            let mut literal = literal.parse::<i32>().unwrap();
            let sign = literal < 0;
            if sign {
                literal = literal.abs();
            }

            clause.push(Literal {var: literal as usize, sign: sign});
        }
        clause.pop();
        cnf.add_clause(clause);
    }
    cnf
}
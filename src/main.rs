use std::io;
use std::io::Write;
use std::str::FromStr;
use std::collections::HashSet;
use std::panic;
use std::process;
use std::iter;

fn main() {
    panic::set_hook(Box::new(|info| {
    if let Some(s) = info.payload().downcast_ref::<String>() {
        println!("{}", s);
    }
    }));
    
    print!("Enter the minimum terms, separated by spaces: ");
    std::io::stdout().flush().unwrap();
    let minterms = read_terms();

    if minterms.len() == 0 {
        println!("No minimum terms entered!");
        return;
    }

    print!("Enter don't care terms (if any), separated by spaces: ");
    std::io::stdout().flush().unwrap();
    let dontcares: Vec<u32> = read_terms();

    let max_value = minterms.iter().chain(dontcares.iter()).max().unwrap();

    let n = 32 - max_value.leading_zeros() as usize;

    if n > 26 {
        println!("Too many variables required! Maximum allowed is 26.");
        process::exit(1);
    }

    let minterms: Vec<String> = minterms.iter().map(|&x| format!("{:0>n$b}", x)).collect();
    let dontcares: Vec<String> = dontcares.iter().map(|&x| format!("{:0>n$b}", x)).collect();
    let allterms: Vec<String> = minterms.iter().chain(dontcares.iter()).cloned().collect();

    println!("Variables: {}", n);
    println!("Minterms: {:?}", minterms);
    println!("Don't care terms: {:?}", dontcares);

    let prime_implicants = find_prime_implicants(allterms, n);
    let gate_input_costs = prime_implicants.iter().map(|t| gate_input_cost(t)).collect::<Vec<_>>();

    println!("Prime Implicants: {:#?}", prime_implicants);

    //Petrick's method
    let m = prime_implicants.len();
    let factors = minterms.iter().map(
        |t| {
            let mut sum_of_terms = HashSet::new();
            for (i, p) in prime_implicants.iter().enumerate() {
                if implies(p, t) {
                    sum_of_terms.insert(single_one(i, m));
                }
            }
            sum_of_terms
        }
    );

    let product = factors.fold(
        {
            let mut init = HashSet::new();
            init.insert(iter::repeat('0').take(m).collect::<String>());
            init
        },
        mul_sums
    );

    let solutions = product.iter().map(
        |sol| {
            let used_indices = (0..m)
                .filter(|i| sol.chars().nth(*i).unwrap() == '1');
            let mut formula = to_formula(used_indices.clone().map(|i| &prime_implicants[i]).collect());
            if formula.is_empty() { formula = "1".to_string(); }
            let mut total_cost = used_indices.clone().map(|i| gate_input_costs[i]).sum::<usize>();
            if used_indices.count() > 1 {
                total_cost += 1;
            }
            (formula, total_cost)
        }
    );

    let min_cost = solutions.clone().min_by_key(|s| s.1).unwrap().1;
    let best_solutions = solutions.filter(|s| s.1 == min_cost).map(|s| s.0).collect::<Vec<_>>();

    println!("Minimum cost: G = {min_cost}");
    println!("Best solution(s):");

    for sol in best_solutions {
        println!("Res = {sol}");
    }

}

fn read_terms() -> Vec<u32> {
    let mut buf = String::new();

    match io::stdin().read_line(&mut buf) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to read line");
            process::exit(1);
        }
    }

    let terms = buf
        .split_whitespace()
        .map(|s|
            match u32::from_str(s) {
                Ok(x) => x,
                Err(_) => {
                    println!("\"{}\" is not a valid integer between 0 and 2^32-1", s);
                    process::exit(1);
                }
            }
        );

    let mut terms: Vec<_> = terms.collect();
    terms.dedup();
    terms
}

fn group_by_ones(terms: &Vec<String>, n: usize) -> Vec<Vec<&String>> {
    terms.into_iter().fold(
        vec![vec![]; n + 1],
        |mut acc, x| {
            let count_ones = x.chars().filter(|&c| c == '1').count();
            acc[count_ones].push(x);
            acc
        }
    )
}

fn merge_terms(t1: &str, t2: &str) -> Option<String> {
    let mut diff = 0;
    let mut res = String::new();

    for (c1, c2) in t1.chars().zip(t2.chars()) {
        if c1 != c2 {
            diff += 1;
            if diff > 1 { return None; }
            res.push('-');
        }
        else { res.push(c1); }
    }
    if diff == 0 { return None; }
    Some(res)
}

fn merge_all(terms: &mut Vec<String>, n: usize) -> Vec<String> {

    let grouped = group_by_ones(&terms, n);
    let mut res_terms = HashSet::new();
    let mut marked = HashSet::new();

    for g in grouped.windows(2) {
        for t1 in g[0].iter() {
            for t2 in g[1].iter() {
                if let Some(merged) = merge_terms(t1, t2) {
                    res_terms.insert(merged);
                    marked.insert(t1);
                    marked.insert(t2);
                }
            }
        }
    }

    let prime_terms = terms.iter().filter(|t| !marked.contains(t)).cloned().collect();

    *terms = res_terms.into_iter().collect();

    prime_terms
}

fn implies(t1: &str, t2: &str) -> bool { 0;
    for (c1, c2) in t1.chars().zip(t2.chars()) {
        match (c1, c2) {
            ('0', '1') | ('1', '0') => return false,
            _ => {}
        }
    }
    true
}

fn find_prime_implicants(mut terms: Vec<String>, n: usize) -> Vec<String> {
    let mut count = 0;
    let mut prime_terms = vec![];
    while !terms.is_empty() {
        let prime = merge_all(&mut terms, n);
        println!("Merge {}", count + 1);
        println!("\tPrime implicants: {prime:?}");
        println!("\tMerged terms: {terms:?}");
        prime_terms.extend(prime);
        count += 1;
    }
    prime_terms
}

fn mul_sums(s1: HashSet<String>, s2: HashSet<String>) -> HashSet<String> {
    let mut res = HashSet::new();
    for t1 in s1.iter() {
        for t2 in s2.iter() {
            res.insert(bit_or(t1, t2));
        }
    }
    res
}

fn bit_or(t1: &str, t2: &str) -> String {
    t1.chars().zip(t2.chars())
        .map(
            |(c1, c2)|
            if c1 == '1' || c2 == '1' { '1' } else { '0' }
        ).collect()
}

fn to_formula(terms: Vec<&String>) -> String {
    terms.iter().map(
        |t| t
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != '-')
            .map(|(i, c)| {
                let mut s = String::new();
                s.push(('A' as u8 + i as u8) as char);
                if c == '0' { s.push('\'') }
                s
            })
            .collect::<Vec<_>>()
            .concat()
    ).collect::<Vec<String>>().join(" + ")
}

fn single_one(pos: usize, n: usize) -> String {
    (0..n).map(|i| if i == pos { '1' } else { '0' }).collect()
}

fn gate_input_cost(term: &str) -> usize {
    let n = term.chars().filter(|&c| c != '-').count();
    if n > 1 { n + 1 } else { n }
}
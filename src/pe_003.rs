use std::io;
use std::io::Write;

/**
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
**/

#[allow(dead_code)]
fn factor_attempt_1(p: u64) -> Vec<u64> {
    let mut v: Vec<u64> = vec![];
    for f in 2..=(p / 2) {
        if p % f == 0 {
            let mut skip = false;
            for existing in v.iter() {
                if f % *existing == 0 {
                    skip = true
                }
            }
            if skip {
                continue;
            }
            println!("{}", f);
            io::stdout().flush().unwrap();
            v.push(f);
        }
    }
    v
}

// I looked up the solution since the above would take many minutes to solve.
fn factor(mut n: u64) -> Vec<u64> {
    let mut v: Vec<u64> = vec![];
    loop {
        let p = smallest(n);
        v.push(p);
        if p < n {
            n = n / p;
        } else {
            return v
        }
    }
}

fn smallest(n: u64) -> u64 {
    for i in 2..n {
        if n % i == 0 {
            return i;
        }
    }
    n
}


mod tests {
    use super::*;

    #[test]
    fn small_prime() {
        assert_eq!(factor(13195), vec![5, 7, 13, 29])
    }

    #[test]
    fn large_prime() {
        assert_eq!(*factor(600_851_475_143).last().unwrap(), 6857 as u64)
    }
}
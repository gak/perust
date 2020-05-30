/**
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10001st prime number?
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get_prime(6), 13);
        assert_eq!(get_prime(10001), 104743);
    }
}

pub fn get_prime(c: u32) -> u32 {
    let mut primes = vec![2, 3, 5];
    let mut v = 5;

    loop {
        v += 2;  // Number has to be odd
        if is_prime(v, &primes) {
            primes.push(v);
            if primes.len() as u32 == c {
                return *primes.last().unwrap();
            }
        }

    }
}

fn is_prime(v: u32, existing: &Vec<u32>) -> bool {
    for factor in existing.iter() {
        if v % *factor == 0 {
            return false
        }
    }
    true
}
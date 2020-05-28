/**
The sum of the squares of the first ten natural numbers is,
1^2 +2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,
(1 + 2 + ... + 10)^2 = 55^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
**/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(sum_diff(10), 2640);
        assert_eq!(sum_diff(100), 25164150);
    }
}

pub fn sum_diff(max: u32) -> u32 {
    let mut sq = 0;
    let mut add = 0;
    for n in 1..=max {
        sq += n*n;
        add += n;
    }
    add * add - sq
}
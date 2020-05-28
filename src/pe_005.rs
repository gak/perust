/**
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(divider(10), 2520);
        assert_eq!(divider(20), 232792560);
    }
}

pub fn divider(max: u32) -> u32 {
    let mut value = max;

    loop {
        for factor in (1..=max-1).rev() {
            if factor == 1 {
                return value
            }
            if value % factor != 0 {
                break
            }
        }
        value += max
    }
}
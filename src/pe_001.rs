/**
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
**/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_items() {
        let mut tof = ThreeOrFive::new();
        assert_eq!(tof.next(), Some(3));
        assert_eq!(tof.next(), Some(5));
        assert_eq!(tof.next(), Some(6));
        assert_eq!(tof.next(), Some(9));
    }

    #[test]
    fn example_sum() {
        assert_eq!(ThreeOrFive::new().take_while(|i| *i < 10).sum::<u64>(), 23);
    }

    #[test]
    fn solution() {
        assert_eq!(ThreeOrFive::new().take_while(|i| *i < 1000).sum::<u64>(), 233168);
    }
}

pub struct ThreeOrFive {
    current: u64,
}

impl ThreeOrFive {
    pub fn new() -> Self {
        return ThreeOrFive {
            current: 0,
        };
    }
}

impl Iterator for ThreeOrFive {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current += 1;
            if self.current % 3 == 0 || self.current % 5 == 0 {
                return Some(self.current);
            }
        }
    }
}


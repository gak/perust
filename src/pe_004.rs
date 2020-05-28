/**
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
**/
mod tests {
    use super::*;

    #[test]
    fn test_palin() {
        assert_eq!(palin(1), Some(9));
        assert_eq!(palin(2), Some(9009));
        assert_eq!(palin(3), Some(906609));
    }

    #[test]
    fn test_min_max() {
        assert_eq!(min(3), 100);
        assert_eq!(max(3), 999);
    }

    #[test]
    fn test_is_palin() {
        assert_eq!(is_palin(0), true);
        assert_eq!(is_palin(11), true);
        assert_eq!(is_palin(12), false);
        assert_eq!(is_palin(121), true);
        assert_eq!(is_palin(122), false);
        assert_eq!(is_palin(32100123), true);
        assert_eq!(is_palin(3210123), true);
    }
}

/**
This attempt will iterate over the factors, collect the palindromes and get the highest.
 */
fn palin(digits: u32) -> Option<u32> {
    let start = min(digits);
    let end = max(digits);
    let mut values = vec![];

    for a in start..=end {
        for b in start..=end {
            let v = a * b;
            if is_palin(v) {
                values.push(v);
            }
        }
    }

    values.iter().max().cloned()
}

fn is_palin(v: u32) -> bool {
    let string = v.to_string();
    let mut s = string.as_str().as_bytes();
    loop {
        let (a, b) = (s[0], s[s.len() - 1]);
        if a != b {
            return false;
        }
        if s.len() <= 3 {
            return true;
        }
        s = &s[1..s.len() - 1]
    }
}

fn min(digits: u32) -> u32 {
    u32::pow(10, digits - 1)
}

fn max(digits: u32) -> u32 {
    u32::pow(10, digits) - 1
}

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_found = u64::MAX;
    let mut max_found = 0_u64;
    let mut min_factors = HashSet::<(u64, u64)>::new();
    let mut max_factors = HashSet::<(u64, u64)>::new();

    for i in min..=max {
        for j in i..=max {
            let candidate = i * j;
            if (candidate > min_found && candidate < max_found) || !is_palindrome(candidate) {
                continue;
            }
            if candidate <= min_found {
                if candidate < min_found {
                    min_found = candidate;
                    min_factors.clear();
                }
                min_factors.insert((i, j));
            }
            if candidate >= max_found {
                if candidate > max_found {
                    max_found = candidate;
                    max_factors.clear();
                }
                max_factors.insert((i, j));
            }
        }
    }

    if min_factors.is_empty() {
        None
    } else {
        Some((
            Palindrome {
                value: min_found,
                factors: min_factors,
            },
            Palindrome {
                value: max_found,
                factors: max_factors,
            },
        ))
    }
}

pub fn is_palindrome(n: u64) -> bool {
    if n < 10 {
        return true;
    }

    let mut digits = Vec::<u8>::new();

    let mut n = n;
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }

    let last = digits.len() - 1;
    for i in 0..digits.len() / 2 {
        if digits[i] != digits[last - i] {
            return false;
        }
    }
    true
}

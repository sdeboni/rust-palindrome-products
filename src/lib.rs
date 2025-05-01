// Iteration 2 inspired by iamhere2's solution with a much more efficient is_palindrome() check
// Iteration 3 inspried by iamhere2's solution improves numeric palindrome check by quickly
// eliminating numbers > 10 and ending with 0

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
    let mut max_found = u64::MIN;
    let mut min_factors = HashSet::<(u64, u64)>::new();
    let mut max_factors = HashSet::<(u64, u64)>::new();

    for i in min..=max {
        if i % 10 == 0 {
            continue;
        }
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

pub fn is_palindrome(value: u64) -> bool {
    if value % 10 == 0 {
        return false;
    }

    let mut reversed = 0;
    let mut n = value;

    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }

    reversed == value
}

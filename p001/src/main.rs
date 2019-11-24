//! Multiplies of 3 and 5
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.

fn multiplies_of_3_and_5(range: u64) -> u64 {
    (0..range)
        .into_iter()
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

fn main() {
    println!("{}", multiplies_of_3_and_5(1000));
}

#[cfg(test)]
mod tests {
    use crate::multiplies_of_3_and_5;

    #[test]
    fn test_multiplies() {
        assert_eq!(23, multiplies_of_3_and_5(10));
    }
}

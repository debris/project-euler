use num_bigint::BigUint;
use num_integer::Integer;

fn smallest_multiple(range: u32) -> BigUint {
    let mut result: BigUint = 1u32.into();
    for i in 1..=range {
        result = result.lcm(&i.into());
    }
    result
}

fn main() {
    println!("{}", smallest_multiple(20));
}

#[cfg(test)]
mod tests {
    use crate::smallest_multiple;

    #[test]
    fn test_smallest_multiple() {
        assert_eq!(smallest_multiple(10), 2520u32.into());
    }
}

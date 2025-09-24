use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

/***
A positive integer is called heptaphobic if it is not divisible by seven and no number divisible by
seven can be produced by swapping two of its digits.
Note that leading zeros are not allowed before or after the swap.

For example, 17 and 1305 are heptaphobic, but 14 and 132 are not because 14 and 231 are divisible
by seven.

Let C(N) count heptaphobic numbers smaller than N. You are given C(100) = 74 and C(10^4) = 3737.

Find C(10^13).
**/

fn find_heptaphobic(n: u128) -> u128 {
    let mut heptaphilic_numbers: HashSet<u128> = HashSet::new();
    for i in 1..n {
        // If the number is already in the set, skip
        if heptaphilic_numbers.contains(&i) {
            continue;
        }

        let permutations = permute_n(i);
        let iterator = permutations.iter();
        // If any permutation is heptaphobic, add all permutations to the set
        for permutation in iterator {
            if permutation % 7 == 0 {
                // dbg!(permutation);
                // dbg!(&permutations);
                heptaphilic_numbers.extend(&permutations);
                break;
            }
        }
    }

    dbg!(&heptaphilic_numbers.len());
    dbg!(&heptaphilic_numbers);
    n - (heptaphilic_numbers.len() as u128)
}

/**
Permute digits of N and return a set
*/
fn permute_n(n: u128) -> HashSet<u128> {

    fn permute_char_digits(digits: Vec<char>) -> HashSet<Vec<char>> {
        let mut permutations: HashSet<Vec<char>> = HashSet::new();

        if digits.len() == 1 {
            permutations.insert(digits);
            return permutations;
        }

        for digit in digits.iter() {}


        permutations
    }

    fn permute_digits(digits: Vec<char>) -> HashSet<u128> {
        let mut permutations: HashSet<u128> = HashSet::new();

        if digits.len() == 1 {
            let Some(leading_digit) = digits[0].to_digit(10) else {panic!("Cannot convert {} to digit", digits[0])}; // pick the leading digit
            // dbg!(&leading_digit);
            permutations.insert(leading_digit as u128);
            return permutations;
        }

        let pow = (digits.len() - 1) as u32;
        // dbg!(&pow);
        for i in 0..digits.len() {
            let Some(leading_digit) = digits[i].to_digit(10) else {panic!("Cannot convert {} to digit", digits[i])}; // pick the leading digit
            // dbg!(&leading_digit);
            let multiplier = (leading_digit as u128) * 10u128.pow(pow);
            // dbg!(&multiplier);
            let mut remaining_digits = digits.clone();
            remaining_digits.remove(i);
            // dbg!(&remaining_digits);
            let remaining_permutations = permute_digits(remaining_digits);

            for permutation in remaining_permutations.iter() {
                let new_permutation = permutation + multiplier;
                // dbg!(&permutation);
                // dbg!(&multiplier);
                // dbg!(&new_permutation);
                permutations.insert(new_permutation);
            }
        }

        permutations
    }

    let digits: Vec<char> = n.to_string().chars().collect();
    dbg!(&digits);
    let permutations = permute_digits(digits);
    permutations
}

/// Generates digits of a number
///
/// # Arguments
///
/// * `n`:
///
/// returns: Vec<u128, Global>
///
/// # Examples
///
/// ```
///
/// ```
fn get_digits(mut number: u128) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    if number == 0 {
        digits.push(0);
        return digits;
    }

    while number > 0 {
        let digit = (number % 10) as u8;
        number = number / 10;
        digits.push(digit);
    }

    digits.reverse();

    digits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_digits_unique() {
        let output = get_digits(12345678);
        let expected = vec![1,2,3,4,5,6,7,8];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_get_digits_with_zeros() {
        let output = get_digits(1230);
        let expected = vec![1,2,3,0];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_get_digits_with_leading_zeros() {
        let output = get_digits(01230);
        let expected = vec![1,2,3,0];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_get_digits_with_leading_duplicates() {
        let output = get_digits(112330);
        let expected = vec![1,1,2,3,3,0];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_permute() {
        let output = permute_n(123);
        let expected: HashSet<u128> = vec![123, 132, 213, 231, 312, 321].into_iter().collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_permute_with_powers() {
        let output = permute_n(100);
        let expected: HashSet<u128> = vec![100].into_iter().collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_permute_zeros() {
        let mut output = permute_n(0000);
        let expected: HashSet<u128> = vec![0].into_iter().collect();
        assert_eq!(output, expected);

        output = permute_n(00);
        assert_eq!(output, expected);

        output = permute_n(0);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_heptatonic_100() {
        let count = find_heptaphobic(100);
        assert_eq!(count, 74);
    }

    #[test]
    fn test_heptatonic_10000() {
        let count = find_heptaphobic(10000);
        assert_eq!(count, 3737);
    }
}
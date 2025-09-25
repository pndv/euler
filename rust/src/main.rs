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

    // For debugging
    let mut elems: Vec<u128> = heptaphilic_numbers.iter().copied().collect();
    elems.sort_unstable();
    println!("heptaphilic_numbers ({} elems): {:?}", elems.len(), elems);



    // since we are finding all numbers less than n, subtract 1 from n
    (n - 1) - (heptaphilic_numbers.len() as u128)
}

/**
Permute digits of N and return a set
*/
fn permute_n(n: u128) -> HashSet<u128> {

    fn permute_char_digits(digits: Vec<char>) -> HashSet<String> {
        let mut permutations: HashSet<String> = HashSet::new();

        if digits.len() == 1 {
            permutations.insert(digits[0].to_string());
            return permutations;
        }

        for i in 0..digits.len() {
            let mut remaining_digits = digits.clone();
            remaining_digits.remove(i);
            let prepend_digit = digits[i];
            let remaining_permutations = permute_char_digits(remaining_digits);

            for permutation in remaining_permutations.iter() {
                let new_permutation = prepend_digit.to_string() + permutation;
                permutations.insert(new_permutation);
            }
        }


        permutations
    }


    let digits: Vec<char> = n.to_string().chars().collect();
    let mut output: HashSet<u128> = HashSet::new();
    let string_permutations = permute_char_digits(digits.clone());
    // dbg!(&string_permutations);
    for permutation in string_permutations.iter() {
        // ignore leading zeros for any string starting with 0, and length more than 1
        // i.e., unless the number is 0 itself, ignore any other number which starts with 0
        if permutation.starts_with("0") && permutation.len() > 1 {
            continue;
        }

        let number = permutation.parse::<u128>().unwrap();
        output.insert(number);
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;


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
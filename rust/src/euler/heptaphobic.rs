use std::collections::HashSet;

/***
A positive integer is called heptaphobic if it is not divisible by seven and no number divisible by
seven can be produced by swapping two of its digits.
Note that leading zeros are not allowed before or after the swap.

For example, 17 and 1305 are heptaphobic, but 14 and 132 are not because 14 and 231 are divisible
by seven.

Let C(N) count heptaphobic numbers smaller than N. You are given C(100) = 74 and C(10^4) = 3737.

Find C(10^13).
**/

pub fn find_heptaphobic(n: u128) -> u128 {
    let mut non_heptaphobic_count: u128 = 0;
    for i in 1..n {
        let dbg_div = 10u128.pow(5);
        if i % dbg_div == 0 {
            println!("Processing: {}", i);
        }

        // If the number is divisible by 7, count it
        if i % 7 == 0 {
            non_heptaphobic_count += 1;
            continue;
        }

        let permutations = permute_n(i);
        // Check if any permutation is heptaphobic
        for permutation in permutations {
            if permutation % 7 == 0 {
                non_heptaphobic_count += 1;
                break;
            }
        }
    }



    // since we are finding all numbers less than n, subtract 1 from n
    (n - 1) - (non_heptaphobic_count)
}


fn count_heptaphobic(n: u128) -> u128 {
    let mut heptaphilic: HashSet<u128> = HashSet::new();
    for i in 1..n {
        if i % 7 == 0 {
            heptaphilic.insert(i);

        }

        let permutations = permute_n(i);
        let iterator = permutations.iter();
        // If any permutation is heptaphobic, add all permutations to the set
        for permutation in iterator {
            if permutation % 7 == 0 {
                // dbg!(permutation);
                // dbg!(&permutations);
                heptaphilic.insert(i);
                break;
            }
        }
    }

    // For debugging
    let mut elems: Vec<u128> = heptaphilic.iter().copied().collect();
    elems.sort_unstable();
    println!("heptaphilic_numbers ({} elems): {:?}", elems.len(), elems);



    // since we are finding all numbers less than n, subtract 1 from n
    (n - 1) - (heptaphilic.len() as u128)
}



/**
Permute digits of N and return a set
*/
fn permute_n(n: u128) -> HashSet<u128> {
    let digits: Vec<char> = n.to_string().chars().collect();

    let mut permutations: HashSet<u128> = HashSet::new();

    let count = digits.len();
    for i in 0..count {
        for j in i..count {
            let mut new_digits = digits.clone();
            new_digits.swap(i, j);

            // ignore leading zeros for any string starting with 0, and length more than 1
            // i.e., unless the number is 0 itself, ignore any other number which starts with 0
            if new_digits[0] == '0' && count > 1 {
                continue;
            }

            let permutation = new_digits.iter().collect::<String>();
            let number = permutation.parse::<u128>().unwrap();


            permutations.insert(number);
        }
    }

    permutations
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_permute_123() {
        // 123 can have the following permutations by swapping 2 digits at a time:
        // 123
        // 132 (swap 2 and 3)
        // 213 (swap 1 and 2)
        // 321 (swap 1 and 3)
        // other permutations require swapping more than 2 digits at a time

        let output = permute_n(123);
        let expected: HashSet<u128> = vec![123, 132, 213, 321].into_iter().collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_permute_980() {
        let output = permute_n(980);
        let expected: HashSet<u128> = vec![980, 908, 890].into_iter().collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_permute_1127() {
        let output = permute_n(1127);
        let expected: HashSet<u128> = vec![1127, 2117, 7121, 1217, 1721, 1172].into_iter().collect();
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
    fn test_heptaphobic_10() {
        let count = find_heptaphobic(10);
        assert_eq!(count, 8);
    }

    #[test]
    fn test_heptaphobic_100() {
        let count = find_heptaphobic(100);
        assert_eq!(count, 74);
    }

    #[test]
    fn test_heptaphobic_1000() {
        let count = find_heptaphobic(1000);
        dbg!(count);
        assert_eq!(count, 573);
    }
    #[test]
    fn test_heptaphobic_10000() {
        let count = find_heptaphobic(10000);
        assert_eq!(count, 3737);
    }

    #[test]
    fn test_heptaphobic_10_pow_6() {
        let count = find_heptaphobic(10u128.pow(6));
        dbg!(count);
    }

    #[test]
    fn test_heptaphobic_10_pow_13() {
        let count = find_heptaphobic(10u128.pow(13));
        dbg!(count);
    }
}
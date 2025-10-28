use std::collections::HashMap;
use crate::leetcode::{Index, IsConstrained};
use crate::leetcode::Count;

#[allow(dead_code)]
/// Given an integer n, count the total number of digit 1 appearing in all non-negative integers less than or equal to n.
///
/// Example 1:
/// Input: n = 13
/// Output: 6
/// 1 2 3 4 5 6 7 8 9 10 11 12 13
/// ^                 ^  ^^ ^  ^ = 6
///
/// Example 2:
/// Input: n = 0
/// Output: 0
///
/// Constraints:
/// 0 <= n <= 10^9
pub fn count_digit_one(n: i32) -> i32 {
    type OnesToLeft = u32; // the number of ones to the left of the current index

    fn helper(
        digits: &Vec<u8>,
        index: Index,
        is_constrained: IsConstrained,
        memoization_table: &mut HashMap<(IsConstrained, Index, OnesToLeft), Count>,
        count: OnesToLeft,
    ) -> Count {
        if index >= digits.len() {
            return count;
        }

        if let Some(&val) = memoization_table.get(&(is_constrained, index, count)) {
            return val;
        }

        let mut total_ones = 0;
        let iterate_up_to = if is_constrained { digits[index] } else { 9u8 };

        for i in 0..=iterate_up_to {
            let updated_count = if i == 1 { count + 1 } else { count };
            let should_constrain_next = i == iterate_up_to && is_constrained;
            total_ones += helper(
                digits,
                index + 1,
                should_constrain_next,
                memoization_table,
                updated_count,
            );
        }
        if !is_constrained {
            memoization_table.insert((is_constrained, index, count), total_ones);
        }

        total_ones
    }

    if n <= 0 {
        return 0;
    }

    let digits = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
    let mut memo_table: HashMap<(IsConstrained, Index, OnesToLeft), Count> = HashMap::new();

    let output = helper(&digits, 0, true, &mut memo_table, 0);
    println!("{:?}", memo_table);
    output as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digit_one_0() {
        let output = count_digit_one(0);
        assert_eq!(output, 0);
    }

    #[test]
    fn test_count_digit_one_1() {
        let output = count_digit_one(1);
        assert_eq!(output, 1);
    }

    #[test]
    fn test_count_digit_one_10() {
        let output = count_digit_one(10);
        assert_eq!(output, 2);
    }

    #[test]
    fn test_count_digit_one_13() {
        let output = count_digit_one(13);
        assert_eq!(output, 6);
    }

    #[test]
    fn test_count_digit_one_20() {
        let output = count_digit_one(20);
        assert_eq!(output, 12);
    }

    #[test]
    fn test_count_digit_one_40() {
        let output = count_digit_one(40);
        assert_eq!(output, 14);
    }

    #[test]
    fn test_count_digit_one_billion() {
        let output = count_digit_one(10i32.pow(9));
        assert_eq!(output, 900000001);
    }
}

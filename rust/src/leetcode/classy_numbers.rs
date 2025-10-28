use crate::leetcode::{Count, Index, IsConstrained};
use std::collections::HashMap;

/// https://codeforces.com/problemset/problem/1036/C?locale=en
/// C. Classy Numbers
/// time limit per test: three seconds
/// memory limit per test: 256 megabytes
/// Let's call some positive integer classy if its decimal representation contains no more than 3
/// non-zero digits. For example, numbers `4`, `200,000`, `10,203`
/// are classy and numbers `4,231`, `102,306`, `7,277,420,000` are not.
///
/// You are given a segment [L;R]. Count the number of classy integers x such that L≤x≤R.
///
/// Each testcase contains several segments, for each of them you are required to solve the problem separately.
///
/// Input
/// The first line contains a single integer T (1≤T≤10^4) — the number of segments in a testcase.
///
/// Each of the next T
/// lines contains two integers L_i and R_i (1≤L_i≤R_i≤10^18).
///
/// Output
/// Print T lines — the i-th line should contain the number of classy integers on a segment [Li;Ri].
///
/// Example
/// Input
/// 4
/// 1 1000
/// 1024 1024
/// 65536 65536
/// 999999 1000001
///
/// Output
/// 1000
/// 1
/// 0
/// 2
pub fn classy_numbers(l: i64, r: i64) -> u32 {
    fn classy_helper(
        digits: &Vec<u8>,
        index: Index,
        is_constrained: IsConstrained,
        is_number_started: bool,
        non_zero_to_left_so_far: u32,
        memo: &mut HashMap<(Index, IsConstrained, Count), Count>,
    ) -> u32 {
        if index >= digits.len() {
            return if non_zero_to_left_so_far > 3 { 0 } else { 1 };
        }

        if let Some(&classy_count) = memo.get(&(index, is_constrained, non_zero_to_left_so_far)) {
            return classy_count;
        }

        let mut count = 0u32;
        let iterate_up_to = if is_constrained { digits[index] } else { 9u8 };

        for i in 0..=iterate_up_to {
            let is_next_number_started = is_number_started || i > 0;
            let should_constrain_next = i == iterate_up_to && is_constrained;
            let next_non_zero_up_to_index = non_zero_to_left_so_far + if i > 0 { 1 } else { 0 };
            let next_classy_count = classy_helper(
                digits,
                index + 1,
                should_constrain_next,
                is_next_number_started,
                next_non_zero_up_to_index,
                memo,
            );
            count += next_classy_count;
        }

        memo.insert((index, is_constrained, non_zero_to_left_so_far), count);
        count
    }

    if l > r || l < 0 || r < 0 {
        return 0;
    }

    //  non_zero_so_far
    let left_digits = (l - 1)
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
    let right_digits = r
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    // Key = Index, IsConstrained, Count of non-zero digits up to that index
    // Value = Count of classy numbers up to that index
    let mut memo: HashMap<(Index, IsConstrained, Count), Count> = HashMap::new();
    let left = classy_helper(&left_digits, 0, true, false, 0, &mut memo);
    memo = HashMap::new();
    let right = classy_helper(&right_digits, 0, true, false, 0, &mut memo);
    right - left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let output = classy_numbers(1, 1);
        assert_eq!(output, 1);
    }

    #[test]
    fn test_1_10() {
        let output = classy_numbers(1, 10);
        assert_eq!(output, 10);
    }

    #[test]
    fn test_1_1000() {
        let output = classy_numbers(1, 1000);
        assert_eq!(output, 1000);
    }
    #[test]
    fn test_classy() {
        let tuples = vec![
            (1, 1000, 1000),
            (1024, 1024, 1),
            (65536, 65536, 0),
            (999999, 1000001, 2),
        ];

        for (l, r, expected) in tuples {
            let output = classy_numbers(l, r);
            assert_eq!(output, expected);
        }
    }
}

// https://www.hackerearth.com/problem/algorithm/benny-and-the-broken-odometer/
// Problem
// One fine day, Benny decided to calculate the number of kilometers that she traveled by her bicycle.
// Therefore, she bought an odometer and installed it onto her bicycle. But the odometer was broken.
// It was not able to display digit 3. This would precisely mean that the odometer won't be
// able to display the numbers having one of their digits as 3.
//
// For example, after the number 1299 the odometer will show 1400.

// Benny was travelling a lot, and now she wants to know the number of kilometers that she has
// travelled. You will be given only the number that Benny saw on the odometer. Your task is to
// determine the real distance.
//
// Input format
//
// The input consists of several test cases.
// The first line contains one integer T denoting the number of test cases.
// The next T lines contain a single integer N denoting the number that Benny saw on the odometer.
//
// Output format
//
// For each test case, print the real distance in a single line.
//
// Constraints
// 1 <= T <= 10^5
// 1 <= N <= 10^9
//
// Sample Input
// 5
// 5
// 14
// 76
// 67
// 40
// Sample Output
// 4
// 12
// 59
// 51
// 27
// Explanation
// In the first sample test case, the odometer skipped the number 3 and displayed 4 after it.
// Hence, the numbers were displayed in the order of [1, 2, 4, 5] accounting to distance of four
// kilometers travelled.
// In the second sample test case, the sequence in which odometer displayed numbers would
// be [1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14] and the distance travelled would be 12 kilometers.

use std::collections::HashMap;

fn broken_odometer(n: u32) -> u32 {
    fn get_number_to_deduct(
        digits: &Vec<u8>,
        index: usize,
        is_constrained: bool,
        previous_digit: u8,
        memo: &mut HashMap<(usize, bool, u8), u32>,
    ) -> u32 {
        if index >= digits.len() {
            return 0;
        }

        if let Some(&cached_distance) = memo.get(&(index, is_constrained, previous_digit)) {
            return cached_distance;
        }

        let iterate_up_to = if is_constrained { digits[index] } else { 9u8 };
        let position_multiplier = 10u32.pow((digits.len() - index - 1) as u32);
        let mut deduct = 0;

        for i in 0..=iterate_up_to {
            if i == 3 {
                deduct += position_multiplier;
                continue; // no need to recurse for the subsequent digits, since they all will be skipped
            }

            let should_constrain_next = i == iterate_up_to && is_constrained;
            deduct += get_number_to_deduct(digits, index + 1, should_constrain_next, i, memo);
        }

        memo.insert((index, is_constrained, previous_digit), deduct);

        deduct
    }

    let digits = n
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    // State: (Index, IsConstrained, DigitAtTheIndex) -> Count of skipped numbers up to that index + digit
    let mut memo: HashMap<(usize, bool, u8), u32> = HashMap::new();

    let deduction = get_number_to_deduct(&digits, 0, true, 0, &mut memo);

    n - deduction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_2() {
        let actual = broken_odometer(2);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_5_4() {
        let actual = broken_odometer(5);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_14_12() {
        let actual = broken_odometer(14);
        assert_eq!(actual, 12);
    }

    #[test]
    fn test_29_26() {
        let actual = broken_odometer(29);
        assert_eq!(actual, 26);
    }

    #[test]
    fn test_40_27() {
        let actual = broken_odometer(40);
        assert_eq!(actual, 27);
    }

    #[test]
    fn test_45_31() {
        let actual = broken_odometer(45);
        assert_eq!(actual, 31);
    }

    #[test]
    fn test_76_59() {
        let actual = broken_odometer(76);
        assert_eq!(actual, 59);
    }
    #[test]
    fn test_67_51() {
        let actual = broken_odometer(67);
        assert_eq!(actual, 51);
    }
}

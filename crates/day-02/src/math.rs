const POWERS_TEN: [u64; 20] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000,
];
/// Given 585, returns 1000
/// 1254 returns 10_000
#[inline]
pub fn round_to_power_of_10(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let digits = digit_length(n);

    POWERS_TEN[digits]
}

#[inline]
/// Given 1234, returns (12, 34)
pub fn left_right_of_num(n: u64) -> (u64, u64) {
    let digit_len = digit_length(n) / 2;
    let half_power = POWERS_TEN[digit_len];
    let left = left_of_num(n, half_power);
    let right = right_of_num(n, half_power);
    (left, right)
}

#[inline]
pub fn left_of_num(n: u64, left: u64) -> u64 {
    n / left
}

#[inline]
pub fn right_of_num(n: u64, right: u64) -> u64 {
    n % right
}

#[inline]
pub fn digit_length(n: u64) -> usize {
    match n {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1000..=9999 => 4,
        10000..=99999 => 5,
        100000..=999999 => 6,
        1000000..=9999999 => 7,
        10000000..=99999999 => 8,
        100000000..=999999999 => 9,
        1000000000..=9999999999 => 10,
        10000000000..=99999999999 => 11,
        100000000000..=999999999999 => 12,
        1000000000000..=9999999999999 => 13,
        10000000000000..=99999999999999 => 14,
        100000000000000..=999999999999999 => 15,
        1000000000000000..=9999999999999999 => 16,
        10000000000000000..=99999999999999999 => 17,
        100000000000000000..=999999999999999999 => 18,
        1000000000000000000..=9999999999999999999 => 19,
        _ => 20,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_round_power_of_ten() {
        let n = 585;
        let rounded = round_to_power_of_10(n);
        assert_eq!(rounded, 1_000);
    }

    #[test]
    fn test_digit_length() {
        let n = 12345;
        let len = digit_length(n);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_left_right_of_num() {
        let n = 1234;
        let lr = left_right_of_num(n);
        assert_eq!((12, 34), lr);

        let n = 123456;
        let lr = left_right_of_num(n);
        let exp = (123, 456);
        assert_eq!(exp, lr);

        let n = 12;
        let lr = left_right_of_num(n);
        assert_eq!((1, 2), lr);
    }

    #[test]
    fn test_left() {
        let n = 1234;
        let left = left_of_num(n, 100);
        assert_eq!(left, 12);
    }

    #[test]
    fn test_right() {
        let n = 1234;
        let right = right_of_num(n, 100);
        assert_eq!(right, 34);
    }
}

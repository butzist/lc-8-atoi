pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut trimmed = s
            .bytes()
            .skip_while(|s| s.is_ascii_whitespace())
            .take_while(|s| !s.is_ascii_whitespace())
            .peekable();

        let sign = {
            let first = trimmed.peek().cloned().unwrap_or_default();
            match first {
                b'+' => {
                    trimmed.next(); // discard
                    1
                }
                b'-' => {
                    trimmed.next(); // discard
                    -1
                }
                _ => 1,
            }
        };

        let mut num: i32 = 0;
        for c in trimmed {
            num = match c {
                b'0'..=b'9' => num
                    .saturating_mul(10)
                    .saturating_add(sign * Self::parse_char(c)),
                _ => return num,
            }
        }

        num
    }

    fn parse_char(c: u8) -> i32 {
        (c - b'0') as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("42", 42)]
    #[case("      -41", -41)]
    #[case("4193 with words", 4193)]
    #[case("99999999999999999999", i32::MAX)]
    #[case("-99999999999999999999", i32::MIN)]
    fn test(#[case] s: String, #[case] result: i32) {
        assert_eq!(Solution::my_atoi(s), result,);
    }
}

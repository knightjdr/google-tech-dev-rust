// Given 2 int values greater than 0, return whichever value is nearest to
// 21 without going over. Return 0 if they both go over.
pub fn blackjack(x: u32, y: u32) -> u32 {
  let target = 21;
  if x > target && y > target {
    return 0;
  } if y > target {
    return x;
  } if x > target {
    return y;
  }
  return if x > y {x} else {y};
}

#[cfg(test)]
mod tests {
  mod blackjack {
    use rstest::rstest;
    use crate::blackjack::blackjack;
    #[rstest]
    #[case(19, 21, 21)]
    #[case(21, 19, 21)]
    #[case(19, 22, 19)]
    #[case(22, 19, 19)]
    #[case(22, 50, 0)]
    #[case(22, 22, 0)]
    #[case(33, 1, 1)]
    #[case(1, 2, 2)]
    #[case(34, 33, 0)]
    #[case(17, 19, 19)]
    #[case(18, 17, 18)]
    #[case(16, 23, 16)]
    #[case(3, 4, 4)]
    #[case(3, 2, 3)]
    #[case(21, 20, 21)]
    fn blackjack_test(#[case] x: u32, #[case] y: u32, #[case] expected: u32) {
      assert_eq!(blackjack(x, y), expected)
    }
  }
}
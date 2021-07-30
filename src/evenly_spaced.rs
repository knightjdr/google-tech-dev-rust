// Given three ints, a b c, one of them is small, one is medium and one is large.
// Return true if the three values are evenly spaced, so the difference between
// small and medium is the same as the difference between medium and large.
pub fn evenly_spaced(a: i32, b: i32, c: i32) -> bool {
  let mut numbers = [a, b, c];
  numbers.sort();

  if numbers[1] - numbers[0] == numbers[2] - numbers[1] {
    return true;
  }
  return false;
}

#[cfg(test)]
mod tests {
  mod evenly_spaced {
    use rstest::rstest;
    use crate::evenly_spaced::evenly_spaced;
    #[rstest]
    #[case(2, 4, 6, true)]
    #[case(4, 6, 2, true)]
    #[case(4, 6, 3, false)]
    #[case(6, 2, 4, true)]
    #[case(6, 2, 8, false)]
    #[case(2, 2, 2, true)]
    #[case(2, 2, 3, false)]
    #[case(9, 10, 11, true)]
    #[case(10, 9, 11, true)]
    #[case(10, 9, 9, false)]
    #[case(2, 4, 4, false)]
    #[case(2, 2, 4, false)]
    #[case(3, 6, 12, false)]
    #[case(12, 3, 6, false)]
    fn evenly_spaced_test(#[case] a: i32, #[case] b: i32, #[case] c: i32, #[case] expected: bool) {
      assert_eq!(evenly_spaced(a, b, c), expected);
    }
  }
}
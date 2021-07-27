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
  mod evenly_spaced{
    use crate::evenly_spaced::evenly_spaced;
    #[test]
    fn evenly_spaced_numbers_true() {
      assert_eq!(evenly_spaced(2, 4, 6), true);
    }
  
    #[test]
    fn unevenly_spaced_numbers_false() {
      assert_eq!(evenly_spaced(2, 4, 7), false);
    }
  }
}
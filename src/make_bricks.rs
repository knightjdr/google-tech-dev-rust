// We want to make a row of bricks that is goal inches long. We have a number of
// small bricks (1 inch each) and big bricks (5 inches each). Return true if it is
// possible to make the goal by choosing from the given bricks. This is a little
// harder than it looks and can be done without any loops.
pub fn make_bricks(small_bricks_available: u32, big_bricks_available: u32, goal: u32) -> bool {
  if goal > (big_bricks_available * 5) + small_bricks_available {
    return false;
  }

  let small_bricks_needed = goal % 5;
  if small_bricks_needed > small_bricks_available {
    return false;
  }

  return true;
}

#[cfg(test)]
mod tests {
  mod make_bricks {
    use rstest::rstest;
    use crate::make_bricks::make_bricks;
    #[rstest]
    #[case(3, 1, 8, true)]
    #[case(3, 1, 9, false)]
    #[case(3, 2, 10, true)]
    #[case(3, 2, 8, true)]
    #[case(3, 2, 9, false)]
    #[case(6, 1, 11, true)]	
    #[case(6, 0, 11, false)]	
    #[case(1, 4, 11, true)]	
    #[case(0, 3, 10, true)]	
    #[case(1, 4, 12, false)]	
    #[case(3, 1, 7, true)]	
    #[case(1, 1, 7, false)]	
    #[case(2, 1, 7, true)]	
    #[case(7, 1, 11, true)]	
    #[case(7, 1, 8, true)]	
    #[case(7, 1, 13, false)]	
    #[case(43, 1, 46, true)]	
    #[case(40, 1, 46, false)]	
    #[case(40, 2, 47, true)]	
    #[case(40, 2, 50, true)]	
    #[case(40, 2, 52, false)]	
    #[case(22, 2, 33, false)]	
    #[case(0, 2, 10, true)]	
    #[case(1000000, 1000, 1000100, true)]	
    #[case(2, 1000000, 100003, false)]	
    #[case(20, 0, 19, true)]	
    #[case(20, 0, 21, false)]	
    #[case(20, 4, 51, false)]	
    #[case(20, 4, 39, true)]
    #[case(3, 1, 8, true)]
    #[case(3, 2, 10, true)]
    fn make_bricks_test(#[case] small_bricks_available: u32, #[case] big_bricks_available: u32, #[case] goal: u32, #[case] expected: bool) {
      assert_eq!(make_bricks(small_bricks_available, big_bricks_available, goal), expected)
    }
  }
}
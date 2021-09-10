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
    use crate::make_bricks::make_bricks;

    macro_rules! make_bricks_tests {
      ($($name:ident: $value:expr,)*) => {
      $(
        #[test]
        fn $name() {
            let (small, big, goal, expected) = $value;
            assert_eq!(expected, make_bricks(small, big, goal));
        }
      )*
      }
    }

    make_bricks_tests! {
      case01: (3, 1, 8, true),
      case02: (3, 1, 9, false),
      case03: (3, 2, 10, true),
      case04: (3, 2, 8, true),
      case05: (3, 2, 9, false),
      case06: (6, 1, 11, true),	
      case07: (6, 0, 11, false),	
      case08: (1, 4, 11, true),	
      case09: (0, 3, 10, true),	
      case10: (1, 4, 12, false),	
      case11: (3, 1, 7, true),	
      case12: (1, 1, 7, false),	
      case13: (2, 1, 7, true),	
      case14: (7, 1, 11, true),	
      case15: (7, 1, 8, true),	
      case16: (7, 1, 13, false),	
      case17: (43, 1, 46, true),	
      case18: (40, 1, 46, false),	
      case19: (40, 2, 47, true),	
      case20: (40, 2, 50, true),	
      case21: (40, 2, 52, false),	
      case22: (22, 2, 33, false),	
      case23: (0, 2, 10, true),	
      case24: (1000000, 1000, 1000100, true),	
      case25: (2, 1000000, 100003, false),	
      case26: (20, 0, 19, true),	
      case27: (20, 0, 21, false),	
      case28: (20, 4, 51, false),	
      case29: (20, 4, 39, true),
      case30: (3, 1, 8, true),
      case31: (3, 2, 10, true),
    }
  }
}
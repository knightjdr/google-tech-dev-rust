// Given three integers, a b c, one of them is small, one is medium and one is large.
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
    use crate::evenly_spaced::evenly_spaced;

    macro_rules! evenly_spaced_tests {
      ($($name:ident: $value:expr,)*) => {
      $(
        #[test]
        fn $name() {
            let (a, b, c, expected) = $value;
            assert_eq!(expected, evenly_spaced(a, b, c));
        }
      )*
      }
    }

    evenly_spaced_tests! {
      case01: (2, 4, 6, true),
      case02: (4, 6, 2, true),
      case03: (4, 6, 3, false),
      case04: (6, 2, 4, true),
      case05: (6, 2, 8, false),
      case06: (2, 2, 2, true),
      case07: (2, 2, 3, false),
      case08: (9, 10, 11, true),
      case09: (10, 9, 11, true),
      case10: (10, 9, 9, false),
      case11: (2, 4, 4, false),
      case12: (2, 2, 4, false),
      case13: (3, 6, 12, false),
      case14: (12, 3, 6, false),
    }
  }
}
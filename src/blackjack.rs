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
    use crate::blackjack::blackjack;

    macro_rules! blackjack_tests {
      ($($name:ident: $value:expr,)*) => {
      $(
        #[test]
        fn $name() {
            let (x, y, expected) = $value;
            assert_eq!(expected, blackjack(x, y));
        }
      )*
      }
    }

    blackjack_tests! {
      case01: (19, 21, 21),
      case02: (19, 21, 21),
      case03: (21, 19, 21),
      case04: (19, 22, 19),
      case05: (22, 19, 19),
      case06: (22, 50, 0),
      case07: (22, 22, 0),
      case08: (33, 1, 1),
      case09: (1, 2, 2),
      case10: (34, 33, 0),
      case11: (17, 19, 19),
      case12: (18, 17, 18),
      case13: (16, 23, 16),
      case14: (3, 4, 4),
      case15: (3, 2, 3),
      case16: (21, 20, 21),
    }
  }
}
// Given an slice of integers, return True if the requested sequence of numbers
// 1, 2, 3 appears in the array somewhere.
pub fn array123(numbers: &[i32]) -> bool {
  let desired_sequence = &[1, 2, 3];
  let desired_sequence_length = desired_sequence.len();

  let input_numbers_length = numbers.len();
  if input_numbers_length < desired_sequence_length {
    return false
  }

  let last_candidate_starting_index = input_numbers_length - desired_sequence_length;
  for i in 0..=last_candidate_starting_index {
    let terminating_index = i + desired_sequence_length;
    let candidate_subset = &numbers[i..terminating_index];
    if candidate_subset == desired_sequence {
      return true
    }
  }

  return false;
}

#[cfg(test)]
mod tests {
  mod array123 {
    use crate::array123::array123;

    macro_rules! array123_tests {
      ($($name:ident: $value:expr,)*) => {
      $(
        #[test]
        fn $name() {
            let (arr, expected) = $value;
            assert_eq!(expected, array123(&arr));
        }
      )*
      }
    }
    
    array123_tests! {
      case01: ([1, 1, 2, 3, 1], true),
      case02: ([1, 1, 2, 4, 1], false),
      case03: ([1, 1, 2, 1, 2, 3], true),
      case04: ([1, 1, 2, 1, 2, 1], false),
      case05: ([1, 2, 3, 1, 2, 3], true),
      case06: ([1, 2, 3], true),
      case07: ([1, 1, 1], false),
      case08: ([1, 2], false),
      case09: ([1], false),
      case10: ([], false),
    }
  }
}

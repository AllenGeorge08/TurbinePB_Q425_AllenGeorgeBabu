use std::num;
// use std::fmt::Error;
use std::{convert::TryInto, ops::Div};

#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// TODO: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
  if b == 0{
    Err(DivisionError::DivideByZero)
  } else if a == i64::MIN && b == -1 {
    Err(DivisionError::IntegerOverflow)   
} else if a%b != 0 {
    Err(DivisionError::NotDivisible)
}  else if a == 0 && b != 0{
    Ok(0)
}
else {
    Ok(a/b)
  }

}
// TODO: Add the correct return type and complete the function body.
// Desired output: `Ok([1, 11, 1426, 3])`
fn result_with_list() -> Result<[i32;4], DivisionError> {
    let numbers = [27, 297, 38502, 81];
    let division_results: Result<Vec<i32>,DivisionError>  = numbers.iter().map(|n|  divide(*n as i64, 27).map(|x| x as i32)).collect(); //e result with lisst collects all results into one Result of a collection..
    //e collect() automatically converts Iterator<Item=Result<T,E>> -> Result<Collection<T>,E>.
    let arr : Result<[i32;4],DivisionError> = division_results?.try_into().map_err(|_| DivisionError::NotDivisible); //e Into iter returnss owned values, we want to work with references that's why thiss..
    arr
}


// TODO: Add the correct return type and complete the function body.
// // Desired output: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() -> [Result<i32,DivisionError>;4]{
    let numbers = [27, 297, 38502, 81];
        // Map each number to a Result<i32, DivisionError>
    let results: Vec<Result<i32, DivisionError>> = numbers.iter().map(|&n| divide(n as i64, 27).map(|x| x as i32)).collect();
    results.try_into().expect("Incorrect array length")
}

fn main() {
    // You can optionally experiment here.
    let result = result_with_list(); 
    println!("{:#?}",result);

    let list_of_results = list_of_results();
    println!("{:?}",list_of_results);


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
        assert_eq!(divide(81, -1), Ok(-81));
        assert_eq!(divide(i64::MIN, i64::MIN), Ok(1));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}

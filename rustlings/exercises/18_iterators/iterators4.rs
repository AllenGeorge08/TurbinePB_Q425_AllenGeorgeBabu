fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion

    //e How to do this using an iterator? Yep, using map..How'll we convert num into an arr or vec?
    let numbers: Vec<u64> = (1..=num).collect();
    // let result : Vec<u64> = numbers.iter().map(|&x| x).collect();
    let factorial  =numbers.iter().fold(1, |acc,&x|  acc * x); //e Acc starts at 1 and then iterates over all elements in the array
    factorial

    //Alternative:
    // numbers.iter().product()
}

fn main() {
    // You can optionally experiment here.

    let factorial_10 = factorial(10);
    println!("{}", factorial_10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);

    }
}

fn main() {
    // You can optionally experiment here.

    let numbers = (22, 23, 24);
    let mut first = 0;
    (first, _, _) = numbers;
    println!("{:?}", first);
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        let mut second = 0;
        (_, second, _) = numbers;
        // let second = ???;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}

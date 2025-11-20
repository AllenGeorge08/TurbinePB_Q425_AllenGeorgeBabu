fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`...
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        //e We've options inside the vector
        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        // integer = optional_integers.pop() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }

        //e Now we want to compare it to every single option that's popped,cool...
        while let Some(integer) = optional_integers.pop() {
            assert_eq!(integer, Some(cursor));
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}

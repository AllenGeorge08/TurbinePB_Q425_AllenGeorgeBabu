fn main() {
    // You can optionally experiment here.
    let mut x: Vec<i32> = Vec::new();
    let z = &mut x;

    z.push(41);

    let y = &mut x;
    y.push(42);

    println!("{:?}", x);
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let z = &mut x;
        z.push(13);
        let y = &mut x;

        // let mut y = x.clone();
        // let mut z = x.clone();

        y.push(42);

        assert_eq!(x, [42, 13]);
    }
}

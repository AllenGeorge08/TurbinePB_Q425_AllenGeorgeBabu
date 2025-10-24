// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.

fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.
    let mut numbers: Vec<i16> = Vec::new(); //e Can't describe a generic here because at compile time .into() should have a defined type

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");

    let n3: u8 = 56;
    let n4: i8 = -56;
    push_value(&mut numbers, n3); //e Sequential borrows one after another , one drops when function ends...
    push_value(&mut numbers, n4);

    println!("{numbers:?}");
}

fn push_value<T: Into<i16>>(vec: &mut Vec<i16>, value: T) {
    vec.push(value.into());
}
